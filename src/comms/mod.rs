//! Admin ↔ Agent Communication Layer (STEP 2)
//! Secure, ID-based, read-only query API
//! Core Principle: Admin only sees IDs, Agent owns filesystem truth

mod protocol;
mod api_server;
mod transport;

pub use protocol::{AdminRequest, AgentResponse, ErrorCode, DriveInfo, NodeInfo, StatsInfo};
pub use api_server::QueryApiServer;
pub use transport::{TransportServer, TransportConfig};

/// Initialize STEP 2 communication layer
pub fn init_step2(
    scanner: std::sync::Arc<crate::filesystem_scanner::FileSystemScanner>,
    query: std::sync::Arc<crate::query_interface::QueryInterface>,
) -> (std::sync::Arc<QueryApiServer>, TransportConfig) {
    println!("📡 Initializing STEP 2: Admin ↔ Agent Communication Layer");
    println!("   Protocol: ID-based, read-only, secure");
    println!("   No mock data in runtime Agent");
    println!("   No networking yet (STEP 3)");
    
    // Create API server
    let api_server = std::sync::Arc::new(QueryApiServer::new(scanner, query));
    
    // Default transport configuration
    let config = TransportConfig::default();
    
    (api_server, config)
}