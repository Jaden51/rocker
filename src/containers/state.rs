use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContainerState {
    Created,
    Running,
    Stopped,
    Removed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub pid: u32,
    pub command: Vec<String>,
    pub state: ContainerState,
}
