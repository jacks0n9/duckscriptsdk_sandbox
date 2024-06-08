use duckscript::types::{command::Commands, error::ScriptError};
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
            "Echo",
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
        whitelist.contains(&module)
    });
    commands.aliases.retain(|_,command_name|commands.commands.contains_key(command_name));
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;
    use duckscript::{runner, types::runtime::Context};

    #[test]
    fn test_no_fs(){
        let mut context=Context::new();
        load(&mut context.commands).unwrap();
        assert!(runner::run_script("readfile ./Cargo.toml", context).is_err());
    }
}