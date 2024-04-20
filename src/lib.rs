use mlua::prelude::*;
mod env;
mod filesystem;
mod process;
mod types;

fn import(_: &Lua, name: String) -> LuaResult<()> {
    println!("import {}", name);
    Ok(())
}

#[mlua::lua_module]
fn stdlib(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table_from([
        ("filesystem", filesystem::module(lua)?),
        ("env", env::module(lua)?),
        ("process", process::module(lua)?),
        ("types", types::module(lua)?),
    ])?;

    lua.globals().set("import", lua.create_function(import)?)?;


    Ok(exports)
}
