--- @class Process
local Proccess = {}

--- @return number
function Proccess.id() end

--- @param code number
function Proccess.exit(code) end

function Proccess.abort() end

--- @type Process
local process = require("stdlib").process

return process
