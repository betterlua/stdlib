use mlua::prelude::*;
use mlua::Lua;

enum PrintValue {
    String(String),
    Number(f64),
    Integer(i64),
    Bool(bool),
    Nil,
}

pub fn print(_: &Lua, args: (String, Vec<LuaValue>)) -> LuaResult<()> {
    let printable: Vec<PrintValue> = args.1.into_iter().map(value_to_printable).collect();
    let string = handle_format(args.0, printable);
    print!("{string}");
    Ok(())
}

pub fn println(_: &Lua, args: (String, Vec<LuaValue>)) -> LuaResult<()> {
    let printable: Vec<PrintValue> = args.1.into_iter().map(value_to_printable).collect();
    let string = handle_format(args.0, printable);
    println!("{string}");
    Ok(())
}

fn handle_format(string: String, args: Vec<PrintValue>) -> String {
    let regex = regex::Regex::new(r"\{(\d+)\}").unwrap();

    regex
        .replace_all(string.as_str(), |caps: &regex::Captures| {
            let index = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let arg = args.get(index).unwrap();
            match arg {
                PrintValue::String(s) => s.to_string(),
                PrintValue::Number(n) => n.to_string(),
                PrintValue::Integer(i) => i.to_string(),
                PrintValue::Bool(b) => b.to_string(),
                PrintValue::Nil => "nil".to_string(),
            }
        })
        .to_string()
}

fn value_to_printable(value: LuaValue) -> PrintValue {
    match value {
        LuaValue::String(s) => PrintValue::String(s.to_str().unwrap().to_string()),
        LuaValue::Number(n) => PrintValue::Number(n),
        LuaValue::Integer(i) => PrintValue::Integer(i),
        LuaValue::Boolean(b) => PrintValue::Bool(b),
        LuaValue::Nil => PrintValue::Nil,
        _ => panic!("can not print value of type {}", value.type_name()),
    }
}
