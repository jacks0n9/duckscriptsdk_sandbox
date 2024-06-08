use duckscript::{types::{command::{Command, CommandResult, Commands}, error::ScriptError, runtime::Context}};
use duckscriptsdk;
pub fn load(commands: &mut Commands)->Result<(),ScriptError>{
    duckscriptsdk::load(commands)?;
    commands.commands.retain(|name,_|{
        let module = match name.split("::").nth(1) {
            Some(module) => module,
            None => return false,
        };
        let whitelist = [
            "math",
            "error",
            "string",
            "collections",
            "IsCommandDefined",
            "Print",
            "random",
            "ReadUserInput",
            "flowcontrol",
            "Release",
            "var",
            "hash",
            "Println",
            "math",
            "Eval",
            "semver",
            "json",
            "Not",
            "Noop",
            "test"
        ];
        return whitelist.contains(&module);
    });
    commands.aliases.retain(|_,command_name|commands.commands.get(command_name).is_some());
    Ok(())
}

