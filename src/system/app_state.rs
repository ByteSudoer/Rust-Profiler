use tokio::sync::broadcast;

pub type Snapshot = Vec<f32>;

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<Snapshot>,
}

impl Default for AppState {
    fn default() -> Self {
        let (txd, _) = broadcast::channel(1);
        Self { tx: txd }
    }
}

#[allow(dead_code)]
impl AppState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1);
        AppState { tx }
    }
}
