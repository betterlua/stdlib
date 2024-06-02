use std::collections::HashMap;

use mlua::LuaSerdeExt;

use super::{r#struct::StructError, Type};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EnumType {
    Number(String, i64),
    Struct(String, HashMap<String, String>),
}

impl<'lua> mlua::FromLua<'lua> for EnumType {
    fn from_lua(
        value: mlua::prelude::LuaValue<'lua>,
        _: &'lua mlua::prelude::Lua,
    ) -> mlua::prelude::LuaResult<Self> {
        let table = value.as_table().unwrap();
        let binding = table.get::<i32, mlua::Value>(1).unwrap();
        let branch = binding.as_str().unwrap();
        let value = table.get::<i32, mlua::Value>(2).unwrap();
        if let mlua::Value::Table(table) = value {
            let mut obj = HashMap::new();
            table.pairs::<mlua::Value, mlua::Value>().for_each(|val| {
                let x = val.unwrap();
                let (k, v) = x;
                let k = k.to_string().unwrap();
                let v = v.to_string().unwrap();
                obj.insert(k, v);
            });
            Ok(EnumType::Struct(String::from(branch), obj))
        } else {
            Ok(EnumType::Number(
                String::from(branch),
                value.as_integer().unwrap(),
            ))
        }
    }
}

pub fn r#enum(lua: &mlua::Lua, values: Vec<EnumType>) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;
    for value in values {
        match value {
            EnumType::Number(name, value) => {
                table.set(name.clone(), value)?;
                table.set(value, name.clone())?;
            }
            EnumType::Struct(name, value) => {
                for (field, r#type) in value.clone() {
                    if !Type::is_type(&r#type) {
                        return Err(mlua::Error::external(StructError::StructInvalidType {
                            field,
                            r#type,
                        }));
                    }
                }
                table.set(
                    name.clone(),
                    lua.create_function(move |_lua, obj: HashMap<String, mlua::Value>| {
                        let obj = obj.clone();
                        for (field, r#type) in value.clone() {
                            if !obj.contains_key(&field) {
                                return Err(mlua::Error::external(StructError::MissingField {
                                    field,
                                    r#type,
                                }));
                            }

                            let value = obj.get(&field).unwrap();
                            let obj_type = Type::from_lua(value.clone());
                            if obj_type != r#type {
                                return Err(mlua::Error::external(StructError::InvalidType {
                                    field,
                                    exptected: Type::pretty(&r#type),
                                    found: Type::pretty(&obj_type),
                                }));
                            }
                        }

                        println!("obj {:?}", obj);

                        Ok(obj)
                    })?,
                )?;
            }
        }
    }
    table.set("__stdlib_type", lua.to_value("__stdlib_type_enum__")?)?;

    Ok(table)
}
