use plum_launcher::lua_plugin::LuaPlugin;
use plum_launcher::runtime::{LauncherConfig, LauncherRuntime};

fn main() -> anyhow::Result<()> {
    // Create a default launcher configuration
    let config = LauncherConfig::default();
    let runtime = LauncherRuntime::new(config);

    // NOTE: The working directory when running this example is the workspace root.
    // Make sure the Lua plugin file exists at "examples/example_plugin.lua" relative to the workspace root.
    let plugin = LuaPlugin::from_file("examples/example_plugin.lua")?;
    runtime.register_plugin(Box::new(plugin));

    // List all registered commands
    for cmd in runtime.get_all_commands() {
        println!("Command: {} - {}", cmd.id, cmd.title);
    }

    Ok(())
}
