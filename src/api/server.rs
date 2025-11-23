//! REST API server

use crate::api::handlers::HandlerState;
use crate::error::ElectionError;
use axum::routing::{get, post};
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// REST API server for election operations
pub struct ApiServer {
    /// Port to listen on
    port: u16,
}

impl ApiServer {
    /// Create a new API server
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    /// Start the server
    pub async fn start(&self) -> Result<(), ElectionError> {
        // Create handler state
        let state = HandlerState::new();

        // Build the router
        let app = Router::new()
            .route("/elections/run", post(crate::api::handlers::run_election))
            .route("/elections/:election_id/results", get(crate::api::handlers::get_election_results))
            .route("/elections/:election_id/diagnostics", get(crate::api::handlers::get_election_diagnostics))
            .route("/health", get(health_check))
            .with_state(state);

        // Create the address
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        
        // Start the server
        let listener = TcpListener::bind(addr).await
            .map_err(|e| ElectionError::InvalidData {
                message: format!("Failed to bind to port {}: {}", self.port, e),
            })?;

        eprintln!("🚀 API server listening on http://{}", addr);
        eprintln!("   POST   /elections/run");
        eprintln!("   GET    /elections/:id/results");
        eprintln!("   GET    /elections/:id/diagnostics");
        eprintln!("   GET    /health");

        axum::serve(listener, app).await
            .map_err(|e| ElectionError::InvalidData {
                message: format!("Server error: {}", e),
            })?;

        Ok(())
    }
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

