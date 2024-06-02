--- @class Filesystem
local Filesystem = {}

--- @param path string
--- @return string
function Filesystem.read_file(path) end

--- @param path string
function Filesystem.read_dir(path) end

--- @param from string
--- @param to string
function Filesystem.copy(from, to) end

--- @param path string
function Filesystem.remove_file(path) end

--- @param path string
function Filesystem.remove_dir(path) end

--- @param path string
function Filesystem.remove_dir_all(path) end

--- @param path string
function Filesystem.create_file(path) end

--- @param path string
function Filesystem.create_dir(path) end

--- @param path string
function Filesystem.create_dir_all(path) end

--- @type Filesystem
local fs = require("stdlib").filesystem

return fs
