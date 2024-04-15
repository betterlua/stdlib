use std::fs;

fn read_file(_: &mlua::Lua, path: String) -> mlua::Result<Option<String>> {
    let contents = fs::read_to_string(path);

    match contents {
        Ok(contents) => Ok(Some(contents)),
        Err(_) => Ok(None),
    }
}

pub fn module(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let exports = lua.create_table()?;
    exports.set("read_file", lua.create_function(read_file)?)?;
    Ok(exports)
}
