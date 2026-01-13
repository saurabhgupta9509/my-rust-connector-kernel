use std::mem::zeroed;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FilePolicy {
    pub path: [u16; 260], // WCHAR[MAX_PATH]
    pub is_folder: u8,
    pub block_read: u8,
    pub block_write: u8,
    pub block_delete: u8,
    pub block_rename: u8,
    pub block_all: u8,
}

impl FilePolicy {
    pub fn new(path: &str) -> Self {
        let mut policy: FilePolicy = unsafe { zeroed() };

        let wide: Vec<u16> = path.encode_utf16().collect();
        for i in 0..wide.len().min(259) {
            policy.path[i] = wide[i];
        }

        policy.is_folder = 1;
        policy.block_write = 1;
        policy.block_rename = 1;
        policy.block_delete = 1;

        policy
    }
}
