//! Path Normalizer - Converts user paths to kernel-safe NT paths
//! IMPORTANT: Internal to Agent only

use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub struct PathNormalizer;

impl PathNormalizer {
    /// Convert DOS path to display format (for debugging only)
    pub fn normalize_display_path(path: &str) -> String {
        let mut normalized: String = path.replace('/', "\\").trim().to_string();
        
        if !normalized.ends_with('\\') && Path::new(&normalized).is_dir() {
            normalized.push('\\');
        }
        
        normalized
    }
    
    /// Convert to UTF-16 for Windows APIs
    pub fn to_wide_string(path: &str) -> Vec<u16> {
        OsStr::new(path)
            .encode_wide()
            .chain(Some(0))
            .collect()
    }
    
    /// Check if path looks like an NT path
    pub fn is_nt_path_like(path: &str) -> bool {
        path.trim().starts_with("\\Device\\") || path.trim().starts_with("\\\\?\\")
    }
    
    /// PLACEHOLDER: Convert DOS path to NT format
    /// ⚠️ TODO: Implement proper Windows API conversion (QueryDosDevice, NtCreateFile, etc.)
    pub fn dos_to_nt_path_placeholder(dos_path: &str, is_folder: bool) -> String {
        // This is a PLACEHOLDER - real implementation would use:
        // 1. QueryDosDevice to get device mapping
        // 2. NtCreateFile with OBJ_CASE_INSENSITIVE
        // 3. QueryObject to get NT path
        
        let normalized = Self::normalize_display_path(dos_path);
        
        // Simple placeholder conversion
        if normalized.starts_with("C:") {
            if is_folder {
                format!("\\Device\\HarddiskVolume4\\{}", &normalized[2..])
            } else {
                format!("\\Device\\HarddiskVolume4\\{}", &normalized[2..])
            }
        } else if normalized.starts_with("D:") {
            if is_folder {
                format!("\\Device\\HarddiskVolume5\\{}", &normalized[2..])
            } else {
                format!("\\Device\\HarddiskVolume5\\{}", &normalized[2..])
            }
        } else {
            // Generic fallback
            if is_folder {
                format!("\\Device\\HarddiskVolume4\\Placeholder\\{}", normalized)
            } else {
                format!("\\Device\\HarddiskVolume4\\Placeholder\\{}", normalized)
            }
        }
    }
    
    /// Validate path format
    pub fn validate_path_format(path: &str) -> Result<(), String> {
        if path.is_empty() {
            return Err("Path cannot be empty".to_string());
        }
        
        if path.contains('\0') {
            return Err("Path contains null character".to_string());
        }
        
        // Check for invalid characters
        let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
        for ch in invalid_chars {
            if path.contains(ch) {
                return Err(format!("Path contains invalid character: '{}'", ch));
            }
        }
        
        Ok(())
    }
}