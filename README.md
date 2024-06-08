# Duckscript SDK Sandbox
This sandbox removes functionallity like fs, env, and os, from the duckscript sdk so you can allow users to use it safely.
## Usage
```rs
use duckscriptsdk_sandbox as dsss;

dsss::load(&mut context.commands)?;
```