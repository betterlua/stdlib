--- @class Consts
--- @field public arch string
--- @field public os string
--- @field public os_family string
--- @field public exe_suffix string
--- @field public exe_prefix string
--- @field public dll_extension string
--- @field public dll_prefix string
--- @field public dll_suffix string
local Consts = {}

--- @class Environment
--- @field public consts Consts
local Environment = {}

--- @return string
function Environment.current_dir() end

--- @param path string
function Environment.set_current_dir(path) end

--- @return { [string]: string }
function Environment.vars() end

--- @param name string
--- @return string
function Environment.var(name) end

--- @param name string
--- @param value string
function Environment.set_var(name, value) end

--- @param name string
function Environment.remove_var(name) end

--- @return string
function Environment.temp_dir() end

--- @return string
function Environment.current_exe() end

--- @type Environment
local env = require("stdlib").env

return env
