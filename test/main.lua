local std = require("stdlib")
local env = std.env;
local types = std.types;

function dump(o)
    if type(o) == 'table' then
        local s = '{\n'
        for k, v in pairs(o) do
            if type(k) ~= 'number' then k = '"' .. k .. '"' end
            s = s .. ' [' .. k .. '] = ' .. dump(v) .. ',\n'
        end
        return s .. '} '
    else
        return tostring(o)
    end
end

local User = types.struct {
    name = types.string
}

local user = User {
    name = "Hello"
}

local MyEnum = types.enum {
    {"string", { types.string }},
    {"number", { types.number }},
}

local x = MyEnum.string {
    string = "Hello",
    number = 1
}
