meta = {
    id = "hello_plugin",
    name = "Hello Plugin",
    version = "0.1.0",
    author = "Your Name",
    description = "A simple plugin that says hello."
}

commands = {
    {
        id = "say_hello",
        title = "Say Hello",
        action = function()
            plugin.log("Hello from Lua!")
            return "success"
        end
    }
}
