use serde::{Deserialize, Serialize};

use crate::command::Command;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginMeta {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
}

pub trait Plugin: Send + Sync {
    fn meta(&self) -> PluginMeta;
    fn commands(&self) -> Vec<Command>;
    fn on_load(&mut self) {}
    fn on_unload(&mut self) {}
}
