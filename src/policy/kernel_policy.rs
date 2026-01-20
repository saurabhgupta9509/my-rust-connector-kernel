//! Kernel Policy Model (STEP 4.3)
//! Core Principle: Convert Admin intent to kernel-understandable rules
//! IMPORTANT: Implements READ = BLOCK ALL enterprise DLP rule

use crate::policy::{ProtectionScope, policy_intent::{PolicyIntent, ProtectionAction, ProtectionOperations}};

/// How kernel should match the path
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathMatchType {
    Exact,      // Exact NT path match (files)
    Prefix,     // NT path prefix match (folders - recursive)
}

/// Kernel-ready policy
#[derive(Debug, Clone)]
pub struct KernelPolicy {
    pub policy_id: u64,                 // Unique policy ID
    pub nt_path: String,                // NT path from PathResolver (INTERNAL ONLY)
    pub match_type: PathMatchType,      // How to match the path
    pub is_recursive: bool,             // For folders: apply to subfolders
    pub blocked_ops: KernelOperations,  // Operations to block
    pub audit_ops: KernelOperations,    // Operations to audit (monitor)
    pub created_by: String,             // Admin who created it
    pub timestamp: u64,                 // When created
    pub comment: Option<String>,        // Optional comment
}

/// Kernel operations (binary flags for kernel)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelOperations {
    pub read: bool,  // Kernel doesn't understand READ directly
    pub write: bool,
    pub delete: bool,
    pub rename: bool,
    pub create: bool,
    pub copy: bool,     // Copy operation
    pub execute: bool,  // Execute operation
}

impl Default for KernelOperations {
    fn default() -> Self {
        KernelOperations {
            read: false,
            write: false,
            delete: false,
            rename: false,
            create: false,
            copy: false,
            execute: false,
        }
    }
}

impl KernelOperations {
    /// Check if any operation is set
    pub fn is_empty(&self) -> bool {
        // ✅ REMOVED: !self.read &&
        !self.write && !self.delete && 
        !self.rename && !self.create && !self.copy && !self.execute
    }
    
    /// Convert to binary flags (for kernel communication)
    pub fn to_flags(&self) -> (u8, u8, u8, u8, u8, u8 , u8) {
        (
            0,
            self.write as u8,
            self.delete as u8,
            self.rename as u8,
            self.create as u8,
            self.copy as u8,
            self.execute as u8,
        )
    }

    /// Check if this represents a "block all" state (after READ expansion)
    pub fn is_block_all(&self) -> bool {
        // When READ was selected, all these will be true after expansion
        // self.write && self.delete && self.rename && 
        // self.create && self.copy && self.execute
        self.read
    }
    
    /// Convert from ProtectionOperations (Admin intent) to KernelOperations
    /// IMPORTANT: READ flag from admin intent expands to all operations
    pub fn from_protection_operations(ops: &ProtectionOperations, action: ProtectionAction) -> Self {
        // First, expand READ if it's present
        let expanded_ops = ops.expand_for_kernel();
        
        KernelOperations {
            read: expanded_ops.read,
            write: expanded_ops.write,
            delete: expanded_ops.delete,
            rename: expanded_ops.rename,
            create: expanded_ops.create,
            copy: expanded_ops.copy,
            execute: expanded_ops.execute,
        }
    }
    
    /// Create a "block all" operations set
    pub fn block_all() -> Self {
        KernelOperations {
            read: false, // Kernel doesn't need this
            write: true,
            delete: true,
            rename: true,
            create: true,
            copy: true,
            execute: true,
        }
    }
}

/// Policy normalizer - converts Admin intent to kernel policy
pub struct PolicyNormalizer;

impl PolicyNormalizer {
    /// Normalize NT path based on scope
    fn normalize_nt_path(nt_path: String, scope: crate::policy::policy_intent::ProtectionScope) -> String {
        let mut path = nt_path;
        
        match scope {
            crate::policy::policy_intent::ProtectionScope::File => {
                // Files: keep as-is
                path
            }
            crate::policy::policy_intent::ProtectionScope::Folder |
            crate::policy::policy_intent::ProtectionScope::FolderRecursive => {
                // Folders: ensure trailing backslash for prefix matching
                // if !path.ends_with('\\') {
                //     path.push('\\');
                // }
                  while path.ends_with("\\\\") {
                    path.pop();
                }

                match scope {
                    ProtectionScope::File => {
                        while path.ends_with('\\') {
                            path.pop();
                        }
                    }
                    ProtectionScope::Folder | ProtectionScope::FolderRecursive => {
                        if !path.ends_with('\\') {
                            path.push('\\');
                        }
                    }
                 }
                path
            }
        }
    }

    /// Normalize policy intent into kernel policy(ies)
    pub fn normalize(
        intent: &PolicyIntent,
        nt_paths: Vec<String>,
        policy_id: u64,
    ) -> Vec<KernelPolicy> {
        println!("🔄 PolicyNormalizer: Converting intent to kernel policy");
        println!("   Intent: {}", intent.describe());
        
        // Log READ semantics warning if applicable
        if intent.operations.read && matches!(intent.action, ProtectionAction::Block) {
            println!("   ⚠️  READ flag detected in intent - expanding to BLOCK ALL");
            println!("   ℹ️  Kernel will receive ALL operations blocked");
        }
        
        nt_paths.into_iter()    
            .map(|nt_path| {
                let normalized_path = Self::normalize_nt_path(nt_path, intent.scope);
                let (match_type, is_recursive) = match intent.scope {
                    crate::policy::policy_intent::ProtectionScope::File => {
                        (PathMatchType::Exact, false)
                    }
                    crate::policy::policy_intent::ProtectionScope::Folder => {
                        (PathMatchType::Prefix, false)
                    }
                    crate::policy::policy_intent::ProtectionScope::FolderRecursive => {
                        (PathMatchType::Prefix, true)
                    }
                };
                
                // Convert protection operations to kernel operations
                let (blocked_ops, audit_ops) = match intent.action {
                    ProtectionAction::Block => {
                        // Block operations - convert from ProtectionOperations
                        let blocked = KernelOperations::from_protection_operations(
                            &intent.operations, 
                            intent.action
                        );
                        
                        (blocked, KernelOperations::default())
                    }
                    ProtectionAction::Allow => {
                        // Allow mode: block everything EXCEPT these operations
                        // This is inverse logic for allow-lists
                        let allowed = if intent.operations.read {
                            // If READ is allowed, block everything else
                            // But READ with Allow is invalid (should be caught in validation)
                            KernelOperations::default() // Fallback
                        } else {
                            // For Allow mode, we block what's NOT in the allow list
                            let expanded_ops = intent.operations.expand_for_kernel();
                            KernelOperations {
                               read: !expanded_ops.read,  // Not in kernel ops
                                write: !expanded_ops.write,
                                delete: !expanded_ops.delete,
                                rename: !expanded_ops.rename,
                                create: !expanded_ops.create,
                                copy: !expanded_ops.copy,
                                execute: !expanded_ops.execute,
                            }
                        };
                        
                        (allowed, KernelOperations::default())
                    }
                    ProtectionAction::Audit => {
                        // Audit mode: allow but log (don't block)
                        let audit = KernelOperations::from_protection_operations(
                            &intent.operations, 
                            intent.action
                        );
                        
                        (KernelOperations::default(), audit)
                    }
                };
                
                let policy = KernelPolicy {
                    policy_id,
                    nt_path: normalized_path,
                    match_type,
                    is_recursive,
                    blocked_ops,
                    audit_ops,
                    created_by: intent.created_by.clone(),
                    timestamp: intent.timestamp,
                    comment: intent.comment.clone(),
                };
                
                println!("   ✅ Created kernel policy ID {} for path", policy_id);
                println!("      Match: {:?}, Recursive: {}", match_type, is_recursive);
                
                // Print operations based on action type
                match intent.action {
                    ProtectionAction::Block => {
                        if intent.operations.read {
                            println!("      🔒 READ = BLOCK ALL: All operations blocked");
                        } else {
                            println!("      Blocked ops: W{} D{} RN{} C{} CP{} EX{}",
                                // ✅ REMOVED: blocked_ops.read as u8,
                                blocked_ops.write as u8,
                                blocked_ops.delete as u8,
                                blocked_ops.rename as u8,
                                blocked_ops.create as u8,
                                blocked_ops.copy as u8,
                                blocked_ops.execute as u8,
                            );
                        }
                    }
                    ProtectionAction::Allow => {
                        // For Allow, show what's allowed (not blocked)
                        println!("      Allowed ops: R{} W{} D{} RN{} C{} CP{} EX{}",
                            !intent.operations.read as u8,  // Show READ from intent
                            !blocked_ops.write as u8,  // Inverse for display
                            !blocked_ops.delete as u8,
                            !blocked_ops.rename as u8,
                            !blocked_ops.create as u8,
                            !blocked_ops.copy as u8,
                            !blocked_ops.execute as u8,
                        );
                    }
                    ProtectionAction::Audit => {
                        println!("      Audit ops: W{} D{} RN{} C{} CP{} EX{}",
                            // ✅ REMOVED: audit_ops.read as u8,
                            audit_ops.write as u8,
                            audit_ops.delete as u8,
                            audit_ops.rename as u8,
                            audit_ops.create as u8,
                            audit_ops.copy as u8,
                            audit_ops.execute as u8,
                        );
                    }
                }
                
                policy
            })
            .collect()
    }
    
    /// Validate kernel policy before sending to kernel
    pub fn validate(policy: &KernelPolicy) -> Result<(), String> {
        // Validate NT path format
        if !policy.nt_path.starts_with("\\Device\\") {
            return Err(format!("Invalid NT path format: {}", policy.nt_path));
        }
        
        // Validate path ending for prefix matches
        if policy.match_type == PathMatchType::Prefix && !policy.nt_path.ends_with('\\') {
            return Err("Prefix match paths must end with backslash".to_string());
        }
        
        // Validate that we're not blocking everything (which would be useless)
        if policy.blocked_ops.is_empty() && policy.audit_ops.is_empty() {
            return Err("Policy has no operations to block or audit".to_string());
        }
        
        Ok(())
    }
}

impl KernelPolicy {
    /// Check if this policy blocks all operations (after READ expansion)
    pub fn is_block_all(&self) -> bool {
        self.blocked_ops.is_block_all()
    }
    
    /// Check if operation should be blocked
    pub fn should_block_operation(&self, operation: &str) -> bool {
        match operation {
            "read" => {
                // ✅ FIXED: Kernel doesn't directly block read
                // Instead, we check if this is a "block all" policy
                // which indicates READ was selected in the intent
                self.is_block_all()
            }
            "write" => self.blocked_ops.write,
            "delete" => self.blocked_ops.delete,
            "rename" => self.blocked_ops.rename,
            "create" => self.blocked_ops.create,
            "copy" => self.blocked_ops.copy,
            "execute" => self.blocked_ops.execute,
            _ => false,
        }
    }
    
    /// Check if operation should be audited
    pub fn should_audit_operation(&self, operation: &str) -> bool {
        match operation {
            "read" => {
                // ✅ FIXED: For audit, we can have read auditing
                // But read is not in KernelOperations, so we need to track it separately
                // For now, return false as audit_ops doesn't have read field
                false
            }
            "write" => self.audit_ops.write,
            "delete" => self.audit_ops.delete,
            "rename" => self.audit_ops.rename,
            "create" => self.audit_ops.create,
            "copy" => self.audit_ops.copy,
            "execute" => self.audit_ops.execute,
            _ => false,
        }
    }
    
    /// Get a description of what this kernel policy does
    pub fn describe(&self) -> String {
        if self.is_block_all() {
            format!("BLOCK ALL operations on {} (Policy ID: {})", 
                self.nt_path, self.policy_id)
        } else {
            let blocked_ops: Vec<&str> = {
                let mut ops = Vec::new();
                if self.blocked_ops.write { ops.push("write"); }
                if self.blocked_ops.delete { ops.push("delete"); }
                if self.blocked_ops.rename { ops.push("rename"); }
                if self.blocked_ops.create { ops.push("create"); }
                if self.blocked_ops.copy { ops.push("copy"); }
                if self.blocked_ops.execute { ops.push("execute"); }
                ops
            };
            
            let audit_ops: Vec<&str> = {
                let mut ops = Vec::new();
                // ✅ REMOVED: read auditing for now
                if self.audit_ops.write { ops.push("write"); }
                if self.audit_ops.delete { ops.push("delete"); }
                if self.audit_ops.rename { ops.push("rename"); }
                if self.audit_ops.create { ops.push("create"); }
                if self.audit_ops.copy { ops.push("copy"); }
                if self.audit_ops.execute { ops.push("execute"); }
                ops
            };
            
            let mut description = format!("Policy ID {} on {}: ", self.policy_id, self.nt_path);
            
            if !blocked_ops.is_empty() {
                description.push_str(&format!("Block [{}]", blocked_ops.join(", ")));
            }
            
            if !audit_ops.is_empty() {
                if !blocked_ops.is_empty() {
                    description.push_str(", ");
                }
                description.push_str(&format!("Audit [{}]", audit_ops.join(", ")));
            }
            
            if self.is_recursive {
                description.push_str(" (Recursive)");
            }
            
            description
        }
    }
}