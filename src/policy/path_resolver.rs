//! Path Resolver (STEP 4.2)
//! Core Principle: Only Agent resolves IDs to NT paths, never exposed to Admin

use crate::fs_index::FilesystemIndex;
use crate::policy::policy_intent::{PolicyIntent, ProtectionScope};
use std::sync::Arc;

/// Resolves node IDs to NT paths (Agent internal only)
pub struct PathResolver {
    index: Arc<FilesystemIndex>,
}

impl PathResolver {
    /// Create new path resolver
    pub fn new(index: Arc<FilesystemIndex>) -> Self {
        PathResolver { index }
    }
    
    /// Get the filesystem index (for internal use)
    pub fn index(&self) -> &Arc<FilesystemIndex> {
        &self.index
    }
    
    fn normalize_nt_path(mut path: String) -> String {
    // Collapse \\ into \
        while path.contains("\\\\") {
            path = path.replace("\\\\", "\\");
        }

        // Remove trailing slash except root cases
        if path.ends_with('\\') && !path.ends_with(":\\") {
            path.pop();
        }

        path
    }


    /// Resolve node ID to NT path (INTERNAL USE ONLY)
    /// ⚠️ This is the SECURITY BOUNDARY - never expose NT paths
    pub fn resolve_nt_path(&self, node_id: u64) -> Result<String, String> {
        println!("🔄 PathResolver: Resolving ID {} → NT path", node_id);
        
        match self.index.resolve_nt_path(node_id) {
            Some(nt_path) => {
                
                let nt_path = Self::normalize_nt_path(nt_path);
                // Validate NT path format
                if !nt_path.starts_with("\\Device\\") {
                    return Err(format!("Invalid NT path format: {}", nt_path));
                }
                
                println!("   ✅ Resolved: {}", nt_path);
                Ok(nt_path)
            }
            None => {
                let error = format!("Node ID {} not found in filesystem index", node_id);
                println!("   ❌ {}", error);
                Err(error)
            }
        }
    }
    
    /// Resolve policy intent to kernel-ready NT path(s)
    /// This handles recursive folder expansion
    pub fn resolve_policy_intent(&self, intent: &PolicyIntent) -> Result<Vec<String>, String> {
        println!("🔄 PathResolver: Resolving policy intent for ID {}", intent.node_id);
        println!("   Scope: {:?}, Action: {:?}", intent.scope, intent.action);
        
        let base_nt_path = self.resolve_nt_path(intent.node_id)?;
        
        match intent.scope {
            ProtectionScope::File => {
                // Single file - exact match
                println!("   ✅ Single file: {}", base_nt_path);
                Ok(vec![base_nt_path])
            }
            
            ProtectionScope::Folder => {
                // Folder only (non-recursive) - needs trailing backslash
                let mut folder_path = base_nt_path;
                if !folder_path.ends_with('\\') {
                    folder_path.push('\\');
                }
                println!("   ✅ Folder (non-recursive): {}", folder_path);
                Ok(vec![folder_path])
            }
            
            ProtectionScope::FolderRecursive => {
                // Recursive folder - for now, just return folder path
                // In production, you might want to enumerate all subpaths
                // For kernel minifilter, prefix matching handles recursion
                let mut folder_path = base_nt_path;
                if !folder_path.ends_with('\\') {
                    folder_path.push('\\');
                }
                println!("   ✅ Folder (recursive - prefix match): {}", folder_path);
                Ok(vec![folder_path])
            }
        }
    }
    
    /// Validate that node exists and is accessible
    pub fn validate_node(&self, node_id: u64) -> Result<(), String> {
        println!("🔍 PathResolver: Validating node {}", node_id);
        
        match self.index.get_node(node_id) {
            Some(node) => {
                if node.is_accessible {
                    println!("   ✅ Node accessible: {} (ID: {})", node.name, node.id);
                    Ok(())
                } else {
                    let error = format!("Node {} is not accessible", node_id);
                    println!("   ❌ {}", error);
                    Err(error)
                }
            }
            None => {
                let error = format!("Node {} not found in index", node_id);
                println!("   ❌ {}", error);
                Err(error)
            }
        }
    }


}