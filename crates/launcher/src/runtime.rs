use std::sync::Arc;

use gpui::{Pixels, px};
use parking_lot::RwLock;

use crate::{command::Command, plugin::Plugin};

#[derive(Clone, Debug)]
pub struct LauncherConfig {
    pub app_name: String,
    pub width: Pixels,
    pub height: Pixels,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            app_name: "Launcher".to_string(),
            width: px(600.0),
            height: px(400.0),
        }
    }
}

pub struct LauncherRuntime {
    config: LauncherConfig,
    plugins: Arc<RwLock<Vec<Box<dyn Plugin>>>>,
}

impl LauncherRuntime {
    pub fn new(config: LauncherConfig) -> Self {
        Self {
            config,
            plugins: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn register_plugin(&self, mut plugin: Box<dyn Plugin>) {
        plugin.on_load();
        self.plugins.write().push(plugin);
    }

    pub fn get_all_commands(&self) -> Vec<Command> {
        self.plugins
            .read()
            .iter()
            .flat_map(|plugin| plugin.commands())
            .collect()
    }

    pub fn config(&self) -> &LauncherConfig {
        &self.config
    }
}
