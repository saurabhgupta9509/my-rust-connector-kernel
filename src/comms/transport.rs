//! Transport Layer - HTTP/WebSocket server for Admin communication (Design Only)
//! Core Principle: Expose Query API Server over network
//! IMPORTANT: This is design documentation only. Actual networking will be in STEP 3.

use super::api_server::QueryApiServer;
use super::protocol::AdminRequest;
use std::sync::Arc;
use std::net::SocketAddr;
use parking_lot::RwLock;

/// Configuration for transport layer (Design Phase)
#[derive(Debug, Clone)]
pub struct TransportConfig {
    pub bind_address: SocketAddr,
    pub http_enabled: bool,
    pub websocket_enabled: bool,
    pub max_connections: usize,
    pub request_timeout_secs: u64,
}

impl Default for TransportConfig {
    fn default() -> Self {
        TransportConfig {
            bind_address: "127.0.0.1:8080".parse().unwrap(),
            http_enabled: true,
            websocket_enabled: false,
            max_connections: 100,
            request_timeout_secs: 30,
        }
    }
}

/// Transport Server (Design Phase)
/// This struct documents the transport design without implementing actual networking.
/// Actual networking will be implemented in STEP 3 with proper HTTP/WebSocket servers.
pub struct TransportServer {
    api_server: Arc<QueryApiServer>,
    config: TransportConfig,
    is_running: Arc<RwLock<bool>>,
}

impl TransportServer {
    /// Create new transport server (design phase)
    pub fn new(api_server: Arc<QueryApiServer>, config: TransportConfig) -> Self {
        TransportServer {
            api_server,
            config,
            is_running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Document the transport layer design
    pub fn document_design(&self) {
        println!("📡 TRANSPORT LAYER DESIGN (STEP 2)");
        println!("=================================");
        println!();
        println!("✅ Architecture Principles:");
        println!("   • Agent ↔ Admin communication via IDs only");
        println!("   • Admin never sees NT paths");
        println!("   • All operations are read-only");
        println!("   • No mock data in Agent runtime");
        println!();
        println!("✅ Planned Endpoints (for STEP 3):");
        println!("   GET  /api/v1/drives           - List all drives");
        println!("   GET  /api/v1/nodes/:id        - Get node info");
        println!("   GET  /api/v1/nodes/:id/children - List children");
        println!("   POST /api/v1/nodes/:id/expand  - Expand directory");
        println!("   POST /api/v1/nodes/:id/collapse - Collapse directory");
        println!("   GET  /api/v1/search/local?parent=...&q=... - Local search");
        println!("   GET  /api/v1/stats            - Get statistics");
        println!("   GET  /api/v1/ping             - Health check");
        println!();
        println!("✅ Security Model:");
        println!("   • Admin inputs validated by Agent");
        println!("   • IDs are validated before use");
        println!("   • No path injection possible");
        println!("   • Read-only access only");
        println!();
        println!("⚠️  Important Limitations:");
        println!("   • Search is LOCAL only (expanded nodes)");
        println!("   • No global filesystem search yet");
        println!("   • No actual networking in STEP 2");
        println!("   • Mock clients belong to tests, not runtime");
    }
    
    /// Mark transport as "design ready"
    pub fn mark_design_ready(&self) {
        let mut running = self.is_running.write();
        *running = true;
        println!("✅ Transport layer design complete");
        println!("   Ready for implementation in STEP 3");
    }
    
    /// Check if design is ready
    pub fn is_design_ready(&self) -> bool {
        *self.is_running.read()
    }
}

// =============================================
// Note to developers:
// =============================================
// The DemoAdminClient has been REMOVED from runtime code.
// Mock clients should only exist in test files, not in production Agent code.
// 
// In STEP 3, we will implement:
// 1. Real HTTP server (using axum/warp)
// 2. Real WebSocket server for real-time updates
// 3. Proper Admin client library
//
// For now (STEP 2), we focus on protocol design and API contract.