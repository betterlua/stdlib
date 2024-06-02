use std::fs;

use mlua::Table;

fn read_file(lua: &mlua::Lua, path: String) -> mlua::Result<Table> {
    let contents = fs::read(path);
    let contents = match contents {
        Ok(contents) => contents,
        Err(err) => return Err(mlua::Error::external(err)),
    };

    let contents_clone = contents.clone();

    let table = lua.create_table()?;

    let string = lua
        .create_function(move |_, _: ()| {
            let contents = contents.clone();
            let string = String::from_utf8(contents).unwrap();
            Ok(string)
        })
        .unwrap();

    let bytes = lua
        .create_function(move |_, _: ()| {
            let contents = contents_clone.clone();
            Ok(contents)
        })
        .unwrap();

    table.set("string", string)?;
    table.set("bytes", bytes)?;

    Ok(table)
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
