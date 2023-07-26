use tokio::sync::broadcast;

pub type Snapshot = Vec<f32>;
pub type SnapshotMemory = f64;

#[derive(Clone, Debug)]
pub struct AppState {
    pub tx: broadcast::Sender<Snapshot>,
    pub tx_memory: broadcast::Sender<SnapshotMemory>,
}

impl Default for AppState {
    fn default() -> Self {
        let (txd, _) = broadcast::channel(1);
        let (txd2, _) = broadcast::channel(1);
        Self {
            tx: txd,
            tx_memory: txd2,
        }
    }
}

#[allow(dead_code)]
impl AppState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1);
        let (tx2, _) = broadcast::channel(1);
        AppState {
            tx: tx,
            tx_memory: tx2,
        }
    }
}
