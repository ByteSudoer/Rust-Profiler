use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use sysinfo::*;
use tokio::sync::broadcast;

use crate::system::app_state::{AppState, Snapshot, SnapshotMemory};

pub fn create_realtime_memory_route() -> Router {
    let (tx, _) = broadcast::channel::<Snapshot>(1);
    let (tx2, _) = broadcast::channel::<SnapshotMemory>(1);
    let mem_struct = crate::system::memory::Memory::new();

    let app_state = AppState {
        tx: tx.clone(),
        tx_memory: tx2.clone(),
    };

    tokio::task::spawn_blocking(move || loop {
        let x = mem_struct.get_used_percent();
        // println!("Used percent : {}", x);
        let _ = tx2.send(x);
        std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    });
    tracing::info!("Memory usage requested");
    Router::new()
        .route("/realtime/mem", get(realtime_memory_get))
        .with_state(app_state.clone())
}

#[axum::debug_handler]
async fn realtime_memory_get(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| async { realtime_memory_stream(state, ws).await })
}

async fn realtime_memory_stream(app_state: AppState, mut ws: WebSocket) {
    let mut rx = app_state.tx_memory.subscribe();

    while let Ok(msg) = rx.recv().await {
        ws.send(Message::Text(serde_json::to_string(&msg).unwrap()))
            .await
            .unwrap();
    }
}
