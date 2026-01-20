//! Explorer UI Behavior (STEP 3)
//! Core Principle: Admin sees ONLY IDs, never NT paths
//! UI manages expand/collapse/loading/selection states

mod explorer_state;
mod interaction_rules;
mod explorer_controller;

pub use explorer_state::{ExplorerState, NodeUIState, SelectionInfo, PendingProtection, ExplorerStats};
pub use interaction_rules::InteractionEngine;
pub use explorer_controller::ExplorerController;

/// Initialize STEP 3 UI layer
pub fn init_step3(api_server: std::sync::Arc<crate::comms::QueryApiServer>) -> std::sync::Arc<ExplorerController> {
    println!("🎨 Initializing STEP 3: Explorer UI Behavior");
    println!("   • Admin sees ONLY IDs, never NT paths");
    println!("   • Search is LOCAL ONLY (expanded nodes)");
    println!("   • Selection prepares for STEP 4 (no kernel calls)");
    
    ExplorerController::new(api_server)
}