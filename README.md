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

## Modules

### Filesystem

#### read_file(path)
```lua
local fs = require("stdlib.filesystem")

local file = fs.read_file("test.txt")

print(file.bytes())

print(file.string())

```

#### read_dir(dir)
```lua
local fs = require("stdlib.filesystem")

local dir = fs.read_dir(".")

for _, entry in ipairs(dir) do
    print(entry)
end
```

#### copy(from, to)
```lua
local fs = require("stdlib.filesystem")

fs.copy("test.txt", "test2.txt")
```

#### remove_file(path)
```lua
local fs = require("stdlib.filesystem")

fs.remove_file("test2.txt")
```

#### remove_dir(path)
```lua
local fs = require("stdlib.filesystem")

fs.remove_dir("test")
```

#### remove_dir_all(path)
```lua
local fs = require("stdlib.filesystem")

fs.remove_dir_all("test/blah/blah")
```

#### create_file(path)
```lua
local fs = require("stdlib.filesystem")

fs.create_file("test.txt")
```

#### create_dir(path)
```lua
local fs = require("stdlib.filesystem")

fs.create_dir("test")
```

#### create_dir_all(path)
```lua
local fs = require("stdlib.filesystem")

fs.create_dir_all("test/blah/blah")
```

### Environment

#### env_current_dir()
```lua
local env = require("stdlib.environment")

print(env.current_dir())
```

#### env_set_current_dir(path)
```lua
local env = require("stdlib.environment")

env.set_current_dir("test")
```

#### vars()
```lua
local env = require("stdlib.environment")

for name, value in env.vars() do
    print(name, value)
end
```

#### var(name)
```lua
local env = require("stdlib.environment")

print(env.var("HOME"))
```

#### remove_var(name)
```lua
local env = require("stdlib.environment")

env.remove_var("HOME")
```

#### set_var(name, value)
```lua
local env = require("stdlib.environment")

env.set_var("HOME", "test")
```

#### temp_dir()
```lua
local env = require("stdlib.environment")

print(env.temp_dir())
```

#### current_exe()
```lua
local env = require("stdlib.environment")

print(env.current_exe())
```

#### consts
```lua
local consts = require("stdlib.environment").consts

local arch = consts.arch --- x86 | x86_64| arm | aarch64 | loongarch64 | m68k | csky | mips | mips64 | powerpc | powerpc64 | riscv64 | s390x | sparc64
local os = consts.os --- linux | macos | windows | freebsd | netbsd | openbsd | dragonfly | solaris | ios | android
local os_family = consts.os_family --- unix | windows
local exe_suffix = consts.exe_suffix --- .exe | .nexe | .pexe | ""
local exe_prefix = consts.exe_prefix ---- .exe | ""
local dll_suffix = consts.dll_suffix --- .dll | .so | .dylib
local dll_extension = consts.dll_extension --- .dll | .so | .dylib
local dll_prefix = consts.dll_prefix --- lib | ""

```

### Process

#### proc_id()
```lua
local proc = require("stdlib.process")

print(proc.id())
```

#### exit(code)
```lua
local proc = require("stdlib.process")

proc.exit(0)
```

#### abort()
```lua
local proc = require("stdlib.process")

proc.abort()
```

### Types

#### string

#### number

#### boolean

#### table

#### function

#### nil

#### struct { ... }
```lua
local types = require("stdlib.types")

local MyStruct = types.struct {
    str = types.string,
    number = types.number,
    bool = types.boolean
}

```

#### enum { ... }
```lua
local types = require("stdlib.types")

local MyEnum = types.enum {
    { "A", 1 },
    { "B", 2 }
    { "C", 3 }
    { "D", 4 }
}
```

## Roadmap

- [ ] Filesystem
- [x] Environment
- [ ] Process

## Authors

- [@frodggy](https://www.github.com/frodggy)
