use std::fs;

use mlua::Table;

fn read_file(_: &mlua::Lua, path: String) -> mlua::Result<String> {
    let contents = fs::read_to_string(path);

    let contents = match contents {
        Ok(contents) => Ok(contents),
        Err(err) => Err(mlua::Error::external(err)),
    };

    contents
}

fn read_dir(lua: &mlua::Lua, path: String) -> mlua::Result<Table> {
    let contents = fs::read_dir(path);
    let contents = match contents {
        Ok(contents) => {
            let mut entries = vec![];
            let mut i = 0;
            for entry in contents {
                let entry = entry.unwrap();
                let name = entry.file_name().into_string().unwrap();
                entries.push((i, name));
                i += 1;
            }
            let entries = lua.create_table_from(entries)?;
            Ok(entries)
        }
        Err(err) => return Err(mlua::Error::external(err)),
    };

    contents
}

fn copy(_: &mlua::Lua, path: (String, String)) -> mlua::Result<()> {
    let (from, to) = path;
    let res = fs::copy(from, to);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(mlua::Error::external(err)),
    }
}

fn remove_file(_: &mlua::Lua, path: String) -> mlua::Result<()> {
    let res = fs::remove_file(path);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(mlua::Error::external(err)),
    }
}

fn remove_dir(_: &mlua::Lua, path: String) -> mlua::Result<()> {
    let res = fs::remove_dir(path);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(mlua::Error::external(err)),
    }
}

fn remove_dir_all(_: &mlua::Lua, path: String) -> mlua::Result<()> {
    let res = fs::remove_dir_all(path);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(mlua::Error::external(err)),
    }
}

fn create_file(_: &mlua::Lua, path: String) -> mlua::Result<()> {
    let res = fs::File::create(path);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(mlua::Error::external(err)),
    }
}

fn create_dir(_: &mlua::Lua, path: String) -> mlua::Result<()> {
    let res = fs::create_dir(path);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(mlua::Error::external(err)),
    }
}

fn create_dir_all(_: &mlua::Lua, path: String) -> mlua::Result<()> {
    let res = fs::create_dir_all(path);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(mlua::Error::external(err)),
    }
}

pub fn module(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {

    let exports = lua.create_table()?;
    exports.set("read_file", lua.create_function(read_file)?)?;
    exports.set("read_dir", lua.create_function(read_dir)?)?;
    exports.set("copy", lua.create_function(copy)?)?;
    exports.set("remove_file", lua.create_function(remove_file)?)?;
    exports.set("remove_dir", lua.create_function(remove_dir)?)?;
    exports.set("remove_dir_all", lua.create_function(remove_dir_all)?)?;
    exports.set("create_file", lua.create_function(create_file)?)?;
    exports.set("create_dir", lua.create_function(create_dir)?)?;
    exports.set("create_dir_all", lua.create_function(create_dir_all)?)?;
    Ok(exports)
}
