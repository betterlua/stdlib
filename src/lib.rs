use mlua::prelude::*;
mod env;
mod filesystem;

fn import(lua: &Lua, name: String) -> LuaResult<()> {
    Ok(())
}

#[mlua::lua_module]
fn stdlib(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table_from([
        ("filesystem", filesystem::module(lua)?),
        ("env", env::module(lua)?),
    ])?;

    exports.set("import", lua.create_function(import)?)?;

    Ok(exports)
}
