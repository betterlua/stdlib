# BetterLua StdLib

## Better synax

### Better Structs
```lua
local types = require("stdlib").types

local MyStruct = types.struct {
    str = types.string,
    number = types.number,
    bool = types.boolean
}

local instance = MyStruct {
    str = "Hello world!" -- works fine,
    number = false --- throws "invalid type for field `number` expected `number` found `boolean`"
}
--- also throws arrow for missing field 'bool'

```

### Better Enums
```lua

-- Basic enum
local MyEnum1 = types.enum {
    { "A", 1 },
    { "B", 2 }
}

local MyEnum2 = types.enum {
    -- Tuple Enum
    { "C", { types.string } },
    -- Struct Enum
    { "D", { number = types.number } }
}

local A = MyEnum1.A
local B = MyEnum1.B
local C = MyEnum2.C { "Test" }
local D = MyEnum2.D { number = 3 }
```


## Roadmap

- [ ] Filesystem
- [x] Environment
- [ ] Process

## Authors

- [@frodggy](https://www.github.com/frodggy)
