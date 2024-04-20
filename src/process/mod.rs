use std::process::{id, abort, exit};

pub fn process_proc_id(_: &mlua::Lua, _: ()) -> mlua::Result<u32> {
    Ok(id())
}

pub fn process_exit(_: &mlua::Lua, code: i32) -> mlua::Result<()> {
    exit(code);
}

pub fn process_abort(_: &mlua::Lua, _: ()) -> mlua::Result<()> {
    abort();
}

pub fn module(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let exports = lua.create_table()?;
    exports.set("proc_id", lua.create_function(process_proc_id)?)?;
    exports.set("exit", lua.create_function(process_exit)?)?;
    exports.set("abort", lua.create_function(process_abort)?)?;
    Ok(exports)
}
