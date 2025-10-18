use plum_launcher::lua_plugin::LuaPlugin;
use plum_launcher::runtime::{LauncherConfig, LauncherRuntime};

fn main() -> anyhow::Result<()> {
    let config = LauncherConfig::default();
    let runtime = LauncherRuntime::new(config);

    // Load a Lua plugin.
    // NOTE: The path is relative to the workspace root, so make sure
    // "examples/example_plugin.lua" exists at the top level of your project.
    let plugin = LuaPlugin::from_file("examples/example_plugin.lua")?;
    runtime.register_plugin(Box::new(plugin));

    // Simulate a user query and match commands
    let query = "hello";
    let commands = runtime.get_all_commands();
    let matching: Vec<_> = commands.iter().filter(|c| c.matches(query)).collect();

    for cmd in matching {
        println!("Matched: {} - {}", cmd.id, cmd.title);
    }

    Ok(())
}
