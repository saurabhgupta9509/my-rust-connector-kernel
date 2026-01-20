// src/kernel/mod.rs
//! Kernel Integration Module (STEP 6)
//! Core Principle: Real-time policy enforcement via kernel minifilter

mod kernel_event_bridge;

pub use kernel_event_bridge::{
    KernelEventBridge, 
    KernelEvent, 
    KernelOperation, 
    EnforcementDecision,
    MockKernelEventGenerator
};

/// Initialize STEP 6 kernel integration
pub fn init_step6(ws_server: std::sync::Arc<crate::networking::WebSocketServer>) 
    -> (KernelEventBridge, tokio::sync::mpsc::Sender<KernelEvent>) 
{
    println!("🔧 Initializing STEP 6: Kernel Enforcement");
    println!("   • Kernel → Agent event bridge");
    println!("   • Real-time policy enforcement");
    println!("   • WebSocket event streaming");
    println!("   • CRITICAL: READ = BLOCK ALL rule");
    
    // Create kernel event bridge
    let (bridge, event_sender) = KernelEventBridge::new(ws_server);
    
    println!("✅ STEP 6: Kernel integration ready");
    println!("   Kernel events will be forwarded to WebSocket");
    
    (bridge, event_sender)
}