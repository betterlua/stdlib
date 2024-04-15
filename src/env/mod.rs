use mlua::prelude::*;
use std::env::*;

fn env_current_dir(_: &Lua, _: ()) -> mlua::Result<String> {
    let cwd = current_dir()?;
    Ok(cwd.to_string_lossy().to_string())
}

fn env_set_current_dir(_: &Lua, path: String) -> mlua::Result<()> {
    set_current_dir(&path)?;
    Ok(())
}

fn env_vars(lua: &Lua, _: ()) -> mlua::Result<LuaTable> {
    let vars = vars().collect::<Vec<_>>();
    let vars = lua.create_table_from(vars)?;
    Ok(vars)
}

fn env_var(_: &Lua, name: String) -> mlua::Result<String> {
    Ok(var(name).unwrap_or_default())
}

fn env_var_remove(_: &Lua, name: String) -> mlua::Result<()> {
    remove_var(name);
    Ok(())
}

fn env_var_set(_: &Lua, args: (String, String)) -> mlua::Result<()> {
    let (name, value) = (args.0, args.1);
    set_var(name, value);
    Ok(())
}

fn env_temp_dir(_: &Lua, _: ()) -> mlua::Result<String> {
    let cwd = temp_dir();
    let cwd = cwd.display().to_string();
    Ok(cwd)
}

fn env_current_exe(_: &Lua, _: ()) -> mlua::Result<String> {
    let cwd = current_exe()?;
    let cwd = cwd.display().to_string();
    Ok(cwd)
}

pub fn module(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let exports = lua.create_table()?;
    exports.set("current_dir", lua.create_function(env_current_dir)?)?;
    exports.set("set_current_dir", lua.create_function(env_set_current_dir)?)?;
    exports.set("vars", lua.create_function(env_vars)?)?;
    exports.set("var", lua.create_function(env_var)?)?;
    exports.set("remove_var", lua.create_function(env_var_remove)?)?;
    exports.set("set_var", lua.create_function(env_var_set)?)?;
    exports.set("temp_dir", lua.create_function(env_temp_dir)?)?;
    exports.set("current_exe", lua.create_function(env_current_exe)?)?;
    let consts = lua
        .create_table_from([
            ("arch", consts::ARCH),
            ("os", consts::OS),
            ("os_family", consts::FAMILY),
            ("exe_suffix", consts::EXE_SUFFIX),
            ("exe_prefix", consts::EXE_EXTENSION),
            ("dll_suffix", consts::DLL_SUFFIX),
            ("dll_extension", consts::DLL_EXTENSION),
            ("dll_prefix", consts::DLL_PREFIX),
        ])
        .unwrap();
    exports.set("consts", consts)?;
    Ok(exports)
}
