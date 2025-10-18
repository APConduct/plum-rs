use crate::command::{Command, CommandContext, CommandHandler, CommandResult};
use anyhow::{Context, Result};
use mlua::prelude::*;
use parking_lot::Mutex;
use std::path::Path;
use std::sync::Arc;

use crate::plugin::{Plugin, PluginMeta};

/// Lua-based plugin that loads from a .lua file
pub struct LuaPlugin {
    meta: PluginMeta,
    lua: Arc<Mutex<Lua>>,
    commands: Vec<Command>,
}

// Safety: mlua::Lua is not Send/Sync, but we wrap it in Arc<Mutex<_>> and never share the raw Lua across threads.
unsafe impl Send for LuaPlugin {}
unsafe impl Sync for LuaPlugin {}

impl LuaPlugin {
    /// Load a Lua plugin from a file
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let lua = Lua::new();

        // Set up the plugin API
        Self::setup_api(&lua).map_err(|e| anyhow::anyhow!("Failed to setup Lua API: {}", e))?;

        // Load the plugin script
        let script = std::fs::read_to_string(path).context("Failed to read Lua plugin file")?;

        lua.load(&script)
            .set_name(path.to_string_lossy().as_ref())
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to execute Lua plugin: {}", e))?;

        // Extract metadata
        let meta = Self::extract_meta(&lua)?;

        // Extract commands
        let commands = Self::extract_commands(&lua)?;

        Ok(Self {
            meta,
            lua: Arc::new(Mutex::new(lua)),
            commands,
        })
    }

    /// Load a Lua plugin from a string
    pub fn from_string(name: &str, script: &str) -> Result<Self> {
        let lua = Lua::new();

        Self::setup_api(&lua).map_err(|e| anyhow::anyhow!("Failed to setup Lua API: {}", e))?;

        lua.load(script)
            .set_name(name)
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to execute Lua plugin: {}", e))?;

        let meta = Self::extract_meta(&lua)?;
        let commands = Self::extract_commands(&lua)?;

        Ok(Self {
            meta,
            lua: Arc::new(Mutex::new(lua)),
            commands,
        })
    }

    fn setup_api(lua: &Lua) -> std::result::Result<(), LuaError> {
        let globals = lua.globals();

        // Create the plugin API table
        let api = lua.create_table()?;

        // Add helper functions
        api.set(
            "log",
            lua.create_function(|_, msg: String| {
                println!("[Lua Plugin] {}", msg);
                Ok(())
            })?,
        )?;

        globals.set("plugin", api)?;

        Ok(())
    }

    fn extract_meta(lua: &Lua) -> Result<PluginMeta> {
        let globals = lua.globals();
        let meta_table: LuaTable = globals
            .get("meta")
            .map_err(|e| anyhow::anyhow!("Plugin must define a 'meta' table: {}", e))?;

        Ok(PluginMeta {
            id: meta_table
                .get("id")
                .map_err(|e| anyhow::anyhow!("Missing 'id' in meta: {}", e))?,
            name: meta_table
                .get("name")
                .map_err(|e| anyhow::anyhow!("Missing 'name' in meta: {}", e))?,
            version: meta_table
                .get("version")
                .map_err(|e| anyhow::anyhow!("Missing 'version' in meta: {}", e))?,
            author: meta_table
                .get("author")
                .map_err(|e| anyhow::anyhow!("Missing 'author' in meta: {}", e))?,
            description: meta_table
                .get("description")
                .map_err(|e| anyhow::anyhow!("Missing 'description' in meta: {}", e))?,
        })
    }

    fn extract_commands(lua: &Lua) -> Result<Vec<Command>> {
        let globals = lua.globals();
        let commands_table: LuaTable = globals
            .get("commands")
            .map_err(|e| anyhow::anyhow!("Plugin must define a 'commands' table: {}", e))?;

        let mut commands = Vec::new();

        for pair in commands_table.pairs::<LuaValue, LuaTable>() {
            let (_, cmd_table) =
                pair.map_err(|e| anyhow::anyhow!("Error reading command table: {}", e))?;

            let id: String = cmd_table
                .get("id")
                .map_err(|e| anyhow::anyhow!("Missing 'id' in command: {}", e))?;
            let title: String = cmd_table
                .get("title")
                .map_err(|e| anyhow::anyhow!("Missing 'title' in command: {}", e))?;
            let subtitle: Option<String> = cmd_table.get("subtitle").ok();
            let keywords: Option<Vec<String>> = cmd_table.get("keywords").ok();
            let action: LuaFunction = cmd_table
                .get("action")
                .map_err(|e| anyhow::anyhow!("Missing 'action' in command: {}", e))?;

            // Store the Lua function in a way the handler can call it
            let lua_clone = lua.clone();
            let action_str = lua.create_registry_value(action).map_err(|e| {
                anyhow::anyhow!("Failed to create registry value for action: {}", e)
            })?;

            let handler = LuaCommandHandler {
                lua: lua_clone,
                action_key: action_str,
            };

            let mut command = Command::new(id, title, handler);

            if let Some(sub) = subtitle {
                command = command.with_subtitle(sub);
            }

            if let Some(kw) = keywords {
                command = command.with_keywords(kw);
            }

            commands.push(command);
        }

        Ok(commands)
    }
}

impl Plugin for LuaPlugin {
    fn meta(&self) -> PluginMeta {
        self.meta.clone()
    }

    fn commands(&self) -> Vec<Command> {
        self.commands.clone()
    }
}

/// Command handler that executes Lua code
#[derive(Debug)]
struct LuaCommandHandler {
    lua: Lua,
    action_key: LuaRegistryKey,
}

impl CommandHandler for LuaCommandHandler {
    fn execute(&self, _ctx: &CommandContext) -> CommandResult {
        let lua_guard = &self.lua;

        match lua_guard.registry_value::<LuaFunction>(&self.action_key) {
            Ok(action) => {
                match action.call::<LuaValue>(LuaValue::Nil) {
                    Ok(result) => {
                        // Parse the result from Lua
                        match result {
                            LuaValue::String(s) => match s.to_str().as_deref() {
                                Ok("success") => CommandResult::Success,
                                Ok(other) => CommandResult::Error(other.to_string()),
                                Err(_) => {
                                    CommandResult::Error("Invalid UTF-8 in Lua string".to_string())
                                }
                            },
                            LuaValue::Boolean(true) => CommandResult::Success,
                            LuaValue::Boolean(false) => CommandResult::SuccessKeepOpen,
                            LuaValue::Nil => CommandResult::Success,
                            _ => CommandResult::Success,
                        }
                    }
                    Err(e) => CommandResult::Error(format!("Lua error: {}", e)),
                }
            }
            Err(e) => CommandResult::Error(format!("Failed to get Lua function: {}", e)),
        }
    }
}

// We need to implement Send + Sync manually since Lua isn't Send
// This is safe because we protect access with Arc<Mutex>
unsafe impl Send for LuaCommandHandler {}
unsafe impl Sync for LuaCommandHandler {}
