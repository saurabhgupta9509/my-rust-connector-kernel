// // Og 

// pub mod fltlib;
// use fltlib::*;
// use windows_sys::Win32::Foundation::*;
// use std::ptr;
// fn wide(s: &str) -> Vec<u16> {
//     s.encode_utf16().chain(Some(0)).collect()
// }
// use std::collections::HashMap;
// type PolicyKey = (String, bool); // (path, is_folder)
// static mut ACTIVE_POLICIES: Option<HashMap<PolicyKey, FilePolicy>> = None;
// #[repr(C)]
// #[derive(Clone, Copy)]
// struct FilePolicy {
//     pub path: [u16; 260],
//     pub is_folder: u8,
//     pub block_read: u8,
//     pub block_write: u8,
//     pub block_delete: u8,
//     pub block_rename: u8,
//     pub block_create: u8,
//     pub block_all: u8,
//     pub timestamp: u64,
//     pub added_by: [u16; 64],
//     pub reserved: [u8; 8],
// }

// fn make_policy(
//     path: &str, 
//     is_folder: bool, 
//     block_read: u8,
//     block_write: u8,
//     block_delete: u8,
//     block_rename: u8,
//     block_create: u8,
//     block_all: u8
// ) -> FilePolicy {
//     let mut p = FilePolicy {
//         path: [0; 260],
//         is_folder: if is_folder { 1 } else { 0 },
//         block_read,
//         block_write,
//         block_delete,
//         block_rename,
//         block_create,
//         block_all,
//         timestamp: std::time::SystemTime::now()
//             .duration_since(std::time::UNIX_EPOCH)
//             .unwrap()
//             .as_secs(),
//         added_by: [0; 64],
//         reserved: [0; 8],
//     };

//     let wide_str: Vec<u16> = path.encode_utf16().collect();
//     let copy_len = wide_str.len().min(259);
//     p.path[..copy_len].copy_from_slice(&wide_str[..copy_len]);
    
//     // Ensure folder paths end with backslash
//     if is_folder && copy_len > 0 && p.path[copy_len-1] != b'\\' as u16 {
//         p.path[copy_len] = b'\\' as u16;
//     }

//     p
// }

// fn remove_policy(path: &str, is_folder: bool) -> FilePolicy {
//     make_policy(path, is_folder, 0, 0, 0, 0, 0, 0)
// }

// fn send_policy(handle: HANDLE, policy: &FilePolicy, description: &str) -> bool {
//     let mut returned: u32 = 0;
//     let status = unsafe {
//         FilterSendMessage(
//             handle,
//             policy as *const _ as _,
//             std::mem::size_of::<FilePolicy>() as u32,
//             ptr::null_mut(),
//             0,
//             &mut returned,
//         )
//     };

//     if status == 0 {
//         println!("✅ {} sent successfully", description);
//         true
//     } else {
//         println!("❌ Failed to send {}: NTSTATUS=0x{:X}", description, status);
//         false
//     }
// }

// fn main() {
//     println!("DLP PHASE-1 - NT PATHS ONLY");
//     println!("============================\n");

//     println!("Connecting to kernel driver...");


//     let mut handle: HANDLE = 0;
//     let port = wide("\\DlpPort");

//     let status = unsafe {
//         FilterConnectCommunicationPort(
//             port.as_ptr(),
//             0,
//             ptr::null(),
//             0,
//             ptr::null_mut(),
//             &mut handle,
//         )
//     };

//     if status != 0 {
//         println!("❌ Failed to connect to kernel: NTSTATUS=0x{:X}", status);
//         return;
//     }

//     println!("✅ Connected to kernel driver!\n");

//     println!("IMPORTANT: Using NT paths only\n");
//     println!("Example NT paths (use DebugView to see real paths):");
//     println!("• \\\\Device\\\\HarddiskVolume4\\\\TopSecret\\\\");
//     println!("• \\\\Device\\\\HarddiskVolume4\\\\Users\\\\Test\\\\file.txt");
//     println!();

//     // TEST 1: Full immutability for folder
//     println!("1. Setting immutability for NT folder:");
//     let nt_folder = make_policy(
//         "\\Device\\HarddiskVolume4\\TopSecret", // CHANGE THIS TO REAL NT PATH
//         true,    // folder
//         1,       // block_read = 0 (ALLOW reading)
//         1,       // block_write = 1 (BLOCK modifications)
//         1,       // block_delete = 1 (BLOCK deletion)
//         1,       // block_rename = 1 (BLOCK rename)
//         1,       // block_create = 1 (BLOCK new files)
//         0       // block_all = 0
//     );
//     send_policy(handle, &nt_folder, "NT folder immutability");

//     // TEST 2: Test with specific file
//     println!("\n2. Protecting specific file:");
//     let nt_file = make_policy(
//         "\\Device\\HarddiskVolume5\\", // CHANGE THIS
//         false,   // file
//         0,       // block_read = 0
//         0,       // block_write = 1
//         0,       // block_delete = 1
//         0,       // block_rename = 1
//         0,       // block_create = 0 (N/A for files)
//         0        // block_all = 0
//     );
//     send_policy(handle, &nt_file, "NT file protection");

//     println!("\n============================");
//     println!("PHASE-1 ACTIVE");
//     println!("============================");
//     println!("Will BLOCK:");
//     println!("  • Write/modify");
//     println!("  • Delete");
//     println!("  • Rename");
//     println!("  • Create new files");
//     println!("  • Memory-mapped writes (PDF/Office)");
//     println!("\nWill ALLOW:");
//     println!("  • Read/open/view");
//     println!("  • Directory listing");
//     println!("============================\n");

//     println!("To get REAL NT paths:");
//     println!("1. Load the driver");
//     println!("2. Open DebugView as Administrator");
//     println!("3. Try to access a file");
//     println!("4. Look for 'DLP[CREATE]:' messages");
//     println!("5. Copy the exact NT path shown\n");

//     println!("Commands:");
//     println!("  test   - Send test paths");
//     println!("  remove <path> - Remove policy");
//     println!("  exit   - Quit");

//     loop {
//         print!("> ");
//         std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
//         let mut input = String::new();
//         std::io::stdin().read_line(&mut input).unwrap();
//         let input = input.trim().to_lowercase();
        
//         match input.as_str() {
//             "exit" => {
//                 println!("Exiting...");
//                 break;
//             },
//             "test" => {
//                 println!("Test paths sent. Check DebugView for actual NT paths.");
//             },
//             cmd if cmd.starts_with("remove ") => {
//                 let path = &cmd[7..].trim();
//                 println!("Removing policy for: {}", path);
//                 let remove = remove_policy(path, true);
//                 send_policy(handle, &remove, &format!("Remove policy: {}", path));
//             },
//             _ => println!("Unknown command. Type 'test', 'remove <path>', or 'exit'"),
//         }
//     }
    
//     println!("DLP Agent terminated.");
// }


//1st


// main.rs - DLP Rust Agent with proper error handling
// pub mod fltlib;

// use fltlib::*;
// use windows_sys::Win32::Foundation::*;
// use std::ptr;
// use std::io::{self, Write};

// fn wide(s: &str) -> Vec<u16> {
//     s.encode_utf16().chain(Some(0)).collect()
// }

// #[repr(C)]
// #[derive(Clone, Copy)]
// struct FilePolicy {
//     pub path: [u16; 260],
//     pub is_folder: u8,
//     pub block_read: u8,
//     pub block_write: u8,
//     pub block_delete: u8,
//     pub block_rename: u8,
//     pub block_create: u8,
//     pub block_all: u8,
//     pub timestamp: u64,
//     pub added_by: [u16; 64],
//     pub reserved: [u8; 8],
// }

// fn make_policy(
//     path: &str, 
//     is_folder: bool, 
//     block_read: u8,
//     block_write: u8,
//     block_delete: u8,
//     block_rename: u8,
//     block_create: u8,
//     block_all: u8
// ) -> FilePolicy {
//     let mut p = FilePolicy {
//         path: [0; 260],
//         is_folder: if is_folder { 1 } else { 0 },
//         block_read,
//         block_write,
//         block_delete,
//         block_rename,
//         block_create,
//         block_all,
//         timestamp: std::time::SystemTime::now()
//             .duration_since(std::time::UNIX_EPOCH)
//             .unwrap_or_default()
//             .as_secs(),
//         added_by: [0; 64],
//         reserved: [0; 8],
//     };

//     let wide_str: Vec<u16> = path.encode_utf16().collect();
//     let copy_len = wide_str.len().min(259);
//     if copy_len > 0 {
//         p.path[..copy_len].copy_from_slice(&wide_str[..copy_len]);
//     }
    
//     // Ensure folder paths end with backslash
//     if is_folder && copy_len > 0 && p.path[copy_len-1] != b'\\' as u16 {
//         if copy_len < 259 {
//             p.path[copy_len] = b'\\' as u16;
//         }
//     }

//     p
// }

// fn remove_policy(path: &str, is_folder: bool) -> FilePolicy {
//     make_policy(path, is_folder, 0, 0, 0, 0, 0, 0)
// }

// fn send_policy(handle: HANDLE, policy: &FilePolicy, description: &str) -> bool {
//     let mut returned: u32 = 0;
//     let status = unsafe {
//         FilterSendMessage(
//             handle,
//             policy as *const _ as _,
//             std::mem::size_of::<FilePolicy>() as u32,
//             ptr::null_mut(),
//             0,
//             &mut returned,
//         )
//     };

//     if status == 0 {
//         println!("✅ {} sent successfully", description);
//         true
//     } else {
//         println!("❌ Failed to send {}: NTSTATUS=0x{:X}", description, status);
//         false
//     }
// }

// fn wait_for_exit() {
//     println!("\nPress Enter to exit...");
//     let mut input = String::new();
//     let _ = io::stdin().read_line(&mut input);
// }

// fn main() {
//     // Set panic hook to prevent silent crashes
//     std::panic::set_hook(Box::new(|panic_info| {
//         println!("\n❌ RUST PANIC OCCURRED!");
//         println!("Error: {}", panic_info);
//         wait_for_exit();
//     }));

//     println!("DLP Rust Agent - Phase 1 (Safe Version)");
//     println!("=======================================\n");

//     // Check if running as administrator (optional)
//     println!("Note: This program needs Administrator privileges");
//     println!("If it closes immediately, run Command Prompt as Admin\n");

//     println!("Step 1: Checking driver connection...");
    
//     let mut handle: HANDLE = 0;
//     let port = wide("\\DlpPort");

//     println!("Attempting to connect to \\\\DlpPort...");
    
//     let status = unsafe {
//         FilterConnectCommunicationPort(
//             port.as_ptr(),
//             0,
//             ptr::null(),
//             0,
//             ptr::null_mut(),
//             &mut handle,
//         )
//     };

//     if status != 0 {
//         println!("❌ FAILED to connect to kernel driver!");
//         println!("NTSTATUS error code: 0x{:X}", status);
//         println!("\nPossible solutions:");
//         println!("1. Run Command Prompt AS ADMINISTRATOR");
//         println!("2. Make sure DLP driver is loaded:");
//         println!("   sc query DLP");
//         println!("   If not running: sc start DLP");
//         println!("3. Driver might not be installed:");
//         println!("   Install driver first");
//         wait_for_exit();
//         return;
//     }

//     println!("✅ SUCCESS! Connected to kernel driver");
//     println!("Connection handle: {:?}", handle);
//     println!("\nStep 2: Sending test policy...\n");

//     // PHASE-1: Allow reading, block everything else
//     let test_policy = make_policy(
//         "\\Device\\HarddiskVolume4\\TopSecret\\Saurabh_Gupta_Resume.pdf", // CHANGE THIS TO REAL NT PATH
//         false,    // folder
//         1,       // block_read = 0 (ALLOW reading)
//         1,       // block_write = 1 (BLOCK modifications)
//         1,       // block_delete = 1 (BLOCK deletion)
//         1,       // block_rename = 1 (BLOCK rename)
//         1,       // block_create = 1 (BLOCK new files)
//         0        // block_all = 0 (not using BlockAll)
//     );
    
//     //  let test_policy = make_policy(
//     //     "\\Device\\x\\",
//     //     true,    // folder
//     //     0,       // block_read = 0 (ALLOW reading)
//     //     0,       // block_write = 1 (BLOCK modifications)
//     //     0,       // block_delete = 1 (BLOCK deletion)
//     //     0,       // block_rename = 1 (BLOCK rename)
//     //     1,       // block_create = 1 (BLOCK new files)
//     //     0        // block_all = 0 (not using BlockAll)
//     // );

//     send_policy(handle, &test_policy, "Phase-1 immutability policy");

//     println!("\n========================================");
//     println!("DLP PHASE-1 ACTIVE");
//     println!("========================================");
//     println!("Protecting: C:\\TopSecret\\");
//     println!("\n✅ ALLOWED Operations:");
//     println!("  • Opening files (double-click)");
//     println!("  • Viewing file contents");
//     println!("  • Reading documents/PDFs");
//     println!("  • Listing directory");
//     println!("\n❌ BLOCKED Operations:");
//     println!("  • Saving/modifying files");
//     println!("  • Deleting files");
//     println!("  • Renaming files");
//     println!("  • Creating new files");
//     println!("  • Copying files (blocked at kernel)");
//     println!("========================================\n");

//     println!("IMPORTANT TEST INSTRUCTIONS:");
//     println!("1. Create folder: C:\\TopSecret\\");
//     println!("2. Put test files inside (e.g., test.txt, test.pdf)");
//     println!("3. Test these operations:");
//     println!("   a. Double-click test.txt → Should OPEN ✅");
//     println!("   b. Try to edit and save → Should FAIL ❌");
//     println!("   c. Try to delete file → Should FAIL ❌");
//     println!("   d. Try to create new file → Should FAIL ❌");
//     println!("\nTo remove protection, use command: remove\n");

//     // Interactive loop
//     loop {
//         print!("> ");
//         io::stdout().flush().unwrap();
        
//         let mut input = String::new();
//         match io::stdin().read_line(&mut input) {
//             Ok(_) => {
//                 let cmd = input.trim().to_lowercase();
                
//                 match cmd.as_str() {
//                     "exit" | "quit" | "q" => {
//                         println!("Exiting DLP Agent...");
//                         break;
//                     },
//                     "remove" | "clear" => {
//                         println!("Removing all policies...");
//                         let remove_policy = remove_policy("C:\\TopSecret\\", true);
//                         send_policy(handle, &remove_policy, "Remove protection");
//                         println!("Protection removed. Files are now mutable.");
//                     },
//                     "test" => {
//                         println!("Current protection active:");
//                         println!("• C:\\TopSecret\\ - Immutable (read-only)");
//                         println!("Test by trying to modify files in that folder.");
//                     },
//                     "help" | "?" => {
//                         println!("Available commands:");
//                         println!("  test    - Show current protection status");
//                         println!("  remove  - Remove all protection");
//                         println!("  exit    - Exit program");
//                         println!("  help    - Show this help");
//                     },
//                     "" => continue, // Empty input
//                     _ => {
//                         println!("Unknown command. Type 'help' for available commands.");
//                     }
//                 }
//             },
//             Err(e) => {

//                 println!("Error reading input: {}", e);
//                 break;
//             }
//         }
//     }

//     println!("DLP Agent terminated.");
//     wait_for_exit();
// }





// main.rs - DLP Rust Agent (Minimal, Correct, Kernel-Safe)

// pub mod fltlib;

// use fltlib::*;
// use windows_sys::Win32::{
//     Foundation::*,
//     Storage::FileSystem::*,
//     System::SystemServices::*,
//     UI::Shell::ShellExecuteW,
//     UI::WindowsAndMessaging::SW_SHOW,
//     Security::*,
//     System::Threading::*,
// };

// use std::ptr;
// use std::io::{self,Write};
// use std::os::windows::ffi::{OsStrExt, OsStringExt};

// // --------------------------------------------------
// // Utility
// // --------------------------------------------------

// fn wide(s: &str) -> Vec<u16> {
//     s.encode_utf16().chain(Some(0)).collect()
// }

// // --------------------------------------------------
// // Admin elevation
// // --------------------------------------------------

// fn is_elevated() -> bool {
//     unsafe {
//         let mut token: HANDLE = 0;
//         let mut elevation: TOKEN_ELEVATION = std::mem::zeroed();
//         let mut size: u32 = 0;

//         if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
//             return false;
//         }

//         if GetTokenInformation(
//             token,
//             TokenElevation,
//             &mut elevation as *mut _ as *mut _,
//             std::mem::size_of::<TOKEN_ELEVATION>() as u32,
//             &mut size,
//         ) != 0 {
//             elevation.TokenIsElevated != 0
//         } else {
//             false
//         }
//     }
// }

// fn restart_as_admin() {
//     let exe = std::env::current_exe().unwrap();
//     let exe_w: Vec<u16> = exe.as_os_str().encode_wide().chain(Some(0)).collect();
//     let verb = wide("runas");

//     unsafe {
//         ShellExecuteW(
//             0,
//             verb.as_ptr(),
//             exe_w.as_ptr(),
//             ptr::null(),
//             ptr::null(),
//             SW_SHOW,
//         );
//     }

//     std::process::exit(0);
// }

// // --------------------------------------------------
// // Disk detection (ONLY HarddiskVolumeX)
// // --------------------------------------------------

// fn list_harddisk_volumes() -> Vec<String> {
//     let mut result = Vec::new();

//     unsafe {
//         let mask = GetLogicalDrives();

//         for i in 0..26 {
//             if (mask & (1 << i)) != 0 {
//                 let letter = (b'A' + i as u8) as char;
//                 let dos = format!("{}:", letter);
//                 let dos_w: Vec<u16> = dos.encode_utf16().chain(Some(0)).collect();

//                 let mut buffer = vec![0u16; 512];

//                 let len = QueryDosDeviceW(
//                     dos_w.as_ptr(),
//                     buffer.as_mut_ptr(),
//                     buffer.len() as u32,
//                 );

//                 if len != 0 {
//                     let device = String::from_utf16_lossy(
//                         &buffer[..buffer.iter().position(|&x| x == 0).unwrap()]
//                     );

//                     if device.starts_with("\\Device\\HarddiskVolume") {
//                         result.push(device);
//                     }
//                 }
//             }
//         }
//     }

//     result
// }

// // --------------------------------------------------
// // Policy structure
// // --------------------------------------------------

// #[repr(C)]
// #[derive(Clone, Copy)]
// struct FilePolicy {
//     pub path: [u16; 260],
//     pub is_folder: u8,
//     pub block_read: u8,
//     pub block_write: u8,
//     pub block_delete: u8,
//     pub block_rename: u8,
//     pub block_create: u8,
//     pub block_all: u8,
//     pub timestamp: u64,
//     pub added_by: [u16; 64],
//     pub reserved: [u8; 8],
// }

// fn make_policy(
//     path: &str,
//     is_folder: bool,
//     block_read: u8,
//     block_write: u8,
//     block_delete: u8,
//     block_rename: u8,
//     block_create: u8,
//     block_all: u8,
// ) -> FilePolicy {
//     let mut p = FilePolicy {
//         path: [0; 260],
//         is_folder: if is_folder { 1 } else { 0 },
//         block_read,
//         block_write,
//         block_delete,
//         block_rename,
//         block_create,
//         block_all,
//         timestamp: std::time::SystemTime::now()
//             .duration_since(std::time::UNIX_EPOCH)
//             .unwrap_or_default()
//             .as_secs(),
//         added_by: [0; 64],
//         reserved: [0; 8],
//     };

//     let wide_path: Vec<u16> = path.encode_utf16().collect();
//     let len = wide_path.len().min(259);
//     p.path[..len].copy_from_slice(&wide_path[..len]);

//     if is_folder && len > 0 && p.path[len - 1] != b'\\' as u16 {
//         p.path[len] = b'\\' as u16;
//     }

//     p
// }

// // --------------------------------------------------
// // Kernel communication
// // --------------------------------------------------

// fn connect_to_kernel() -> Result<HANDLE, String> {
//     let mut handle: HANDLE = 0;
//     let port = wide("\\DlpPort");

//     let status = unsafe {
//         FilterConnectCommunicationPort(
//             port.as_ptr(),
//             0,
//             ptr::null(),
//             0,
//             ptr::null_mut(),
//             &mut handle,
//         )
//     };

//     if status != 0 {
//         Err(format!("Kernel connect failed: NTSTATUS 0x{:X}", status))
//     } else {
//         Ok(handle)
//     }
// }

// fn send_policy(handle: HANDLE, policy: &FilePolicy) {
//     let mut returned = 0u32;

//     let status = unsafe {
//         FilterSendMessage(
//             handle,
//             policy as *const _ as _,
//             std::mem::size_of::<FilePolicy>() as u32,
//             ptr::null_mut(),
//             0,
//             &mut returned,
//         )
//     };

//     if status == 0 {
//         println!("✅ Policy sent successfully");
//     } else {
//         println!("❌ Failed to send policy: NTSTATUS 0x{:X}", status);
//     }
// }

// // --------------------------------------------------
// // Main
// // --------------------------------------------------

// fn main() {
//     println!("🚀 DLP Rust Agent (Minimal)");

//     if !is_elevated() {
//         println!("⚠️  Restarting as Administrator...");
//         restart_as_admin();
//     }

//     let handle = match connect_to_kernel() {
//         Ok(h) => {
//             println!("✅ Connected to kernel driver");
//             h
//         }
//         Err(e) => {
//             println!("❌ {}", e);
//             return;
//         }
//     };

//     println!("\n🔍 Detected disks:");

//     let disks = list_harddisk_volumes();

//     if disks.is_empty() {
//         println!("❌ No HarddiskVolume found");
//         return;
//     }

//     for d in &disks {
//         println!("{}", d);
//     }

//     // Automatically pick first disk
//     let selected_path = format!("{}\\", disks[0]);
//     println!("\n🔒 Applying READ-ONLY policy on:");
//     println!("{}", selected_path);

//     let policy = make_policy(
//         &selected_path,
//         true,
//         0, // allow read
//         1, // block write
//         1, // block delete
//         1, // block rename
//         1, // block create
//         0,
//     );

//     send_policy(handle, &policy);

//     println!("\nDLP policy active. Press Enter to exit.");
//     let _ = io::stdin().read_line(&mut String::new());
// }



//new one which is working tested 

pub mod fltlib;
use fltlib::*;
use windows_sys::Win32::Foundation::*;
use std::ptr;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
// use ctrlc;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

// Global policy registry
type PolicyKey = String; // Normalized NT path (folders end with \)
static ACTIVE_POLICIES: Lazy<Mutex<HashMap<PolicyKey, FilePolicy>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

#[repr(C)]
#[derive(Clone, Copy)]
struct FilePolicy {
    pub path: [u16; 260],
    pub is_folder: u8,
    pub block_read: u8,
    pub block_write: u8,
    pub block_delete: u8,
    pub block_rename: u8,
    pub block_create: u8,
    pub block_all: u8,
    pub timestamp: u64,
    pub added_by: [u16; 64],
    pub reserved: [u8; 8],
}

/// Normalize NT path consistently
/// - Converts to UTF-16
/// - For folders: ensures trailing backslash
/// - Truncates to 259 characters (leaving room for optional trailing backslash)
fn normalize_nt_path(path: &str, is_folder: bool) -> (Vec<u16>, String) {
    let mut normalized = String::from(path);
    
    // Trim and normalize
    normalized = normalized.trim().replace('/', "\\");
    
    // Ensure proper NT path format
    if !normalized.starts_with("\\Device\\") {
        // If not already an NT path, assume it's already normalized from user input
        // (In production, you'd want to validate this more thoroughly)
    }
    
    // For folders, ensure trailing backslash
    if is_folder && !normalized.ends_with('\\') {
        normalized.push('\\');
    }
    
    // Convert to UTF-16
    let wide_str: Vec<u16> = normalized.encode_utf16().collect();
    
    (wide_str, normalized)
}

fn make_policy(
    path: &str,
    is_folder: bool,
    block_read: u8,
    block_write: u8,
    block_delete: u8,
    block_rename: u8,
    block_create: u8,
    block_all: u8
) -> (FilePolicy, String) {
    let (wide_str, normalized_path) = normalize_nt_path(path, is_folder);
    
    let mut p = FilePolicy {
        path: [0; 260],
        is_folder: if is_folder { 1 } else { 0 },
        block_read,
        block_write,
        block_delete,
        block_rename,
        block_create,
        block_all,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        added_by: [0; 64],
        reserved: [0; 8],
    };

    // Copy the normalized path
    let copy_len = wide_str.len().min(259);
    p.path[..copy_len].copy_from_slice(&wide_str[..copy_len]);
    
    (p, normalized_path)
}

/// Remove a policy by sending zeroed flags
fn remove_policy_internal(handle: HANDLE, path: &str, is_folder: bool) -> bool {
    let (remove_policy, normalized_path) = make_policy(path, is_folder, 0, 0, 0, 0, 0, 0);
    send_policy(handle, &remove_policy, &format!("Remove policy: {}", normalized_path))
}

/// Apply a policy with proper update logic
fn apply_policy(
    handle: HANDLE,
    path: &str,
    is_folder: bool,
    block_read: u8,
    block_write: u8,
    block_delete: u8,
    block_rename: u8,
    block_create: u8,
    block_all: u8
) -> bool {
    let (new_policy, normalized_path) = make_policy(
        path, is_folder,
        block_read, block_write, block_delete,
        block_rename, block_create, block_all
    );
    
    // Check if policy already exists
    let mut policies = ACTIVE_POLICIES.lock().unwrap();
    
    if let Some(existing_policy) = policies.get(&normalized_path) {
        // Policy exists - remove it first
        println!("⚠️ Policy exists for {}, removing old policy...", normalized_path);
        
        let (remove_policy, _) = make_policy(path, is_folder, 0, 0, 0, 0, 0, 0);
        if !send_policy(handle, &remove_policy, &format!("Remove old policy: {}", normalized_path)) {
            println!("❌ Failed to remove old policy");
            return false;
        }
    }
    
    // Send new policy
    let description = if policies.contains_key(&normalized_path) {
        format!("Update policy: {}", normalized_path)
    } else {
        format!("Add policy: {}", normalized_path)
    };
    
    if send_policy(handle, &new_policy, &description) {
        // Store in registry
        policies.insert(normalized_path.clone(), new_policy);
        true
    } else {
        false
    }
}

/// Remove a policy from both kernel and registry
fn remove_policy(handle: HANDLE, path: &str, is_folder: bool) -> bool {
    let (_, normalized_path) = normalize_nt_path(path, is_folder);
    let mut policies = ACTIVE_POLICIES.lock().unwrap();
    
    if policies.contains_key(&normalized_path) {
        if remove_policy_internal(handle, path, is_folder) {
            policies.remove(&normalized_path);
            println!("✅ Removed policy for: {}", normalized_path);
            true
        } else {
            println!("❌ Failed to remove policy for: {}", normalized_path);
            false
        }
    } else {
        println!("⚠️ No active policy found for: {}", normalized_path);
        false
    }
}

/// Cleanup all policies on exit
fn cleanup_all_policies(handle: HANDLE) {
    println!("🧹 Cleaning up all active policies...");
    
    let mut policies = ACTIVE_POLICIES.lock().unwrap();
    let count = policies.len();
    
    if count == 0 {
        println!("✅ No active policies to clean up");
        return;
    }
    
    println!("📋 Removing {} active policies...", count);
    
    let mut successful = 0;
    let mut failed = 0;
    
    // Create a copy of keys to avoid borrowing issues
    let keys: Vec<(String, bool)> = policies.iter()
        .map(|(path, policy)| (path.clone(), policy.is_folder == 1))
        .collect();
    
    for (path, is_folder) in keys {
        if remove_policy_internal(handle, &path, is_folder) {
            successful += 1;
        } else {
            failed += 1;
        }
    }
    
    // Clear the registry
    policies.clear();
    
    println!("✅ Cleanup complete:");
    println!("   Successful removals: {}", successful);
    println!("   Failed removals: {}", failed);
    
    if failed > 0 {
        println!("⚠️ WARNING: Some policies may still be active in kernel!");
    }
}

fn send_policy(handle: HANDLE, policy: &FilePolicy, description: &str) -> bool {
    let mut returned: u32 = 0;
    let status = unsafe {
        FilterSendMessage(
            handle,
            policy as *const _ as _,
            std::mem::size_of::<FilePolicy>() as u32,
            ptr::null_mut(),
            0,
            &mut returned,
        )
    };

    if status == 0 {
        println!("✅ {}", description);
        true
    } else {
        println!("❌ Failed to send {}: NTSTATUS=0x{:X}", description, status);
        false
    }
}

/// Display current active policies
fn list_policies() {
    let policies = ACTIVE_POLICIES.lock().unwrap();
    
    println!("📋 Active Policies ({}):", policies.len());
    println!("{:-<80}", "");
    
    for (path, policy) in policies.iter() {
        let policy_type = if policy.is_folder == 1 { "Folder" } else { "File " };
        println!("{}: {}", policy_type, path);
        println!("  Flags: R{} W{} D{} RN{} C{} A{}",
            policy.block_read,
            policy.block_write,
            policy.block_delete,
            policy.block_rename,
            policy.block_create,
            policy.block_all
        );
        println!();
    }
}

fn main() {
    println!("DLP PHASE-1 - NT PATHS ONLY");
    println!("============================\n");

    println!("Connecting to kernel driver...");
    
    let mut handle: HANDLE = 0;
    let port = wide("\\DlpPort");

    let status = unsafe {
        FilterConnectCommunicationPort(
            port.as_ptr(),
            0,
            ptr::null(),
            0,
            ptr::null_mut(),
            &mut handle,
        )
    };

    if status != 0 {
        println!("❌ Failed to connect to kernel: NTSTATUS=0x{:X}", status);
        return;
    }

    println!("✅ Connected to kernel driver!\n");
    
    // Setup Ctrl+C handler for cleanup
    let handle_for_ctrlc = handle;
    ctrlc::set_handler(move || {
        println!("\n🛑 Ctrl+C received, cleaning up...");
        cleanup_all_policies(handle_for_ctrlc);
        std::process::exit(0);
    }).expect("Error setting Ctrl+C handler");

    println!("IMPORTANT: Using NT paths only\n");
    println!("Example NT paths (use DebugView to see real paths):");
    println!("• \\\\Device\\\\HarddiskVolume4\\\\TopSecret\\\\");
    println!("• \\\\Device\\\\HarddiskVolume4\\\\Users\\\\Test\\\\file.txt");
    println!();

    // TEST: Apply some initial policies
    println!("1. Setting immutability for NT folder:");
    apply_policy(
        handle,
        "\\Device\\HarddiskVolume4\\Agentstarts",
        true,    // folder
        0,       // block_read = 0 (ALLOW reading)
        1,       // block_write = 1 (BLOCK modifications)
        1,       // block_delete = 1 (BLOCK deletion)
        1,       // block_rename = 1 (BLOCK rename)
        1,       // block_create = 1 (BLOCK new files)
        0        // block_all = 0
    );

    println!("\n2. Protecting specific file: ");
    apply_policy(
        handle,
        "\\Device\\HarddiskVolume4\\TopSecret\\Saurabh_Gupta_Resum.pdf",
        false,   // file
        0,       // block_read = 0
        0,       // block_write = 1
        0,       // block_delete = 1
        0,       // block_rename = 1
        0,       // block_create = 0 (N/A for files)
        1        // block_all = 0
    );

    println!("\n============================");
    println!("PHASE-1 ACTIVE");
    println!("============================");
    println!("Will BLOCK:");
    println!("  • Write/modify");
    println!("  • Delete");
    println!("  • Rename");
    println!("  • Create new files");
    println!("  • Memory-mapped writes (PDF/Office)");
    println!("\nWill ALLOW:");
    println!("  • Read/open/view");
    println!("  • Directory listing");
    println!("============================\n");

    println!("Commands:");
    println!("  test        - Send test paths");
    println!("  list        - Show active policies");
    println!("  apply       - Apply new policy (interactive)");
    println!("  update      - Update existing policy (interactive)");
    println!("  remove <path> - Remove policy for path");
    println!("  cleanup     - Remove all policies");
    println!("  exit        - Quit and cleanup");

    loop {
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let lower_input = input.to_lowercase();
        
        match lower_input.as_str() {
            "exit" => {
                println!("Exiting...");
                cleanup_all_policies(handle);
                break;
            },
            "test" => {
                println!("Test: Applying sample policy...");
                apply_policy(
                    handle,
                    "\\Device\\HarddiskVolume4\\TestFolder",
                    true,
                    0, 1, 1, 1, 1, 0
                );
            },
            "list" => {
                list_policies();
            },
            "apply" | "update" => {
                println!("Enter NT path:");
                let mut path = String::new();
                std::io::stdin().read_line(&mut path).unwrap();
                let path = path.trim();
                
                println!("Is folder? (y/n):");
                let mut is_folder = String::new();
                std::io::stdin().read_line(&mut is_folder).unwrap();
                let is_folder = is_folder.trim().to_lowercase() == "y";
                
                println!("Block read? (0/1):");
                let mut block_read = String::new();
                std::io::stdin().read_line(&mut block_read).unwrap();
                let block_read = block_read.trim().parse::<u8>().unwrap_or(0);
                
                println!("Block write? (0/1):");
                let mut block_write = String::new();
                std::io::stdin().read_line(&mut block_write).unwrap();
                let block_write = block_write.trim().parse::<u8>().unwrap_or(0);
                
                println!("Block delete? (0/1):");
                let mut block_delete = String::new();
                std::io::stdin().read_line(&mut block_delete).unwrap();
                let block_delete = block_delete.trim().parse::<u8>().unwrap_or(0);
                
                println!("Block rename? (0/1):");
                let mut block_rename = String::new();
                std::io::stdin().read_line(&mut block_rename).unwrap();
                let block_rename = block_rename.trim().parse::<u8>().unwrap_or(0);
                
                println!("Block create? (0/1):");
                let mut block_create = String::new();
                std::io::stdin().read_line(&mut block_create).unwrap();
                let block_create = block_create.trim().parse::<u8>().unwrap_or(0);
                
                println!("Block all? (0/1):");
                let mut block_all = String::new();
                std::io::stdin().read_line(&mut block_all).unwrap();
                let block_all = block_all.trim().parse::<u8>().unwrap_or(0);
                
                apply_policy(
                    handle,
                    path,
                    is_folder,
                    block_read,
                    block_write,
                    block_delete,
                    block_rename,
                    block_create,
                    block_all
                );
            },
            cmd if cmd.starts_with("remove ") => {
                let path = &cmd[7..].trim();
                println!("Removing policy for: {}", path);
                
                println!("Is folder? (y/n):");
                let mut is_folder = String::new();
                std::io::stdin().read_line(&mut is_folder).unwrap();
                let is_folder = is_folder.trim().to_lowercase() == "y";
                
                remove_policy(handle, path, is_folder);
            },
            "cleanup" => {
                cleanup_all_policies(handle);
            },
            _ => {
                println!("Available commands:");
                println!("  test        - Send test paths");
                println!("  list        - Show active policies");
                println!("  apply       - Apply new policy");
                println!("  update      - Update existing policy");
                println!("  remove <path> - Remove policy for path");
                println!("  cleanup     - Remove all policies");
                println!("  exit        - Quit and cleanup");
            },
        }
    }
    
    println!("DLP Agent terminated.");
}