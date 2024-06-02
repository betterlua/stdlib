use mlua::prelude::*;
mod env;
mod filesystem;
mod process;
mod types;
mod print;

fn import(_: &Lua, name: String) -> LuaResult<()> {
    println!("import {}", name);
    Ok(())
}
pub fn core(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {

    let exports = lua.create_table()?;
    exports.set("print", lua.create_function(print::print)?)?;
    exports.set("println", lua.create_function(print::println)?)?;
    Ok(exports)
}


#[mlua::lua_module]
fn stdlib(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table_from([
        ("filesystem", filesystem::module(lua)?),
        ("env", env::module(lua)?),
        ("process", process::module(lua)?),
        ("types", types::module(lua)?),
        ("core", core(lua)?)
    ])?;

    lua.globals().set("import", lua.create_function(import)?)?;


    Ok(exports)
}
