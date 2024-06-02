use std::collections::HashMap;

use mlua::LuaSerdeExt;
use thiserror::Error;

use crate::types::Type;

#[derive(Debug, Error)]
pub enum StructError {
    #[error("missing field `{field}` of type `{r#type}`")]
    MissingField { field: String, r#type: String },
    #[error("invalid type for field `{field}` expected `{exptected}` found `{found}`")]
    InvalidType {
        field: String,
        exptected: String,
        found: String,
    },
    #[error("invalid type for field `{field}` {r#type}")]
    StructInvalidType { field: String, r#type: String },
}

pub fn r#struct(
    lua: &mlua::Lua,
    structure: HashMap<String, String>,
) -> mlua::Result<mlua::Function> {
    for (field, r#type) in structure.clone() {
        if !Type::is_type(&r#type) {
            return Err(mlua::Error::external(StructError::StructInvalidType {
                field,
                r#type,
            }));
        }
    }

    let r#struct = lua.create_function(move |lua, obj: HashMap<String, mlua::Value>| {
        let mut obj = obj.clone();
        for (field, r#type) in structure.clone() {
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

        obj.insert("__stdlib_type".to_string(), lua.to_value("__stdlib_type_struct__")?);

        let table: mlua::Table = lua.create_table_from(obj)?;

        Ok(table)
    })?;

    Ok(r#struct)
}
