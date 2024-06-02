--- @class Core
local Core = {}

---@param string string
---@param args any[]
function Core.print(string, args) end

---@param string string
---@param args any[]
function Core.println(string, args) end

--- @type Core
local core = require("stdlib").core

return core
