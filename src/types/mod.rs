mod r#struct;
mod r#enum;

pub struct Type;
impl Type {
    pub const STRING: &'static str = "__stdlib_type_string__";
    pub const NUMBER: &'static str = "__stdlib_type_number__";
    pub const BOOLEAN: &'static str = "__stdlib_type_boolean__";
    pub const TABLE: &'static str = "__stdlib_type_table__";
    pub const STRUCT: &'static str = "__stdlib_type_struct__";
    pub const ENUM: &'static str = "__stdlib_type_enum__";
    pub const NIL: &'static str = "__stdlib_type_nil__";

    pub fn from_lua(value: mlua::Value) -> String {
        let type_ = match value {
            mlua::Value::String(_) => Self::STRING,
            mlua::Value::Number(_) => Self::NUMBER,
            mlua::Value::Integer(_) => Self::NUMBER,
            mlua::Value::Boolean(_) => Self::BOOLEAN,
            mlua::Value::Table(table) => {
                let is_custom = table.contains_key("__stdlib_type").unwrap();

                if is_custom {
                    let r#type = table.get::<&str, String>("__stdlib_type").unwrap();
                    let r#type = r#type.as_str();
                    if Type::is_type(r#type) {
                        if r#type == Self::STRUCT {
                            Self::STRUCT
                        } else if r#type == Self::ENUM {
                            Self::ENUM
                        } else {
                            Self::TABLE
                        }
                    } else {
                        Self::TABLE
                    }
                } else {
                    Self::TABLE
                }
            }
            mlua::Value::Nil => Self::NIL,
            _ => Self::NIL,
        };

        type_.to_string()
    }

    pub fn is_type(r#type: &str) -> bool {
        let regex =
            regex::Regex::new(r"^__stdlib_type_(string|number|boolean|table|struct|nil)__$")
                .unwrap();

        regex.is_match(r#type)
    }

    pub fn pretty(value: &str) -> String {
        match value {
            Self::STRING => "string".to_string(),
            Self::NUMBER => "number".to_string(),
            Self::BOOLEAN => "boolean".to_string(),
            Self::TABLE => "table".to_string(),
            Self::STRUCT => "struct".to_string(),
            Self::ENUM => "enum".to_string(),
            Self::NIL => "nil".to_string(),
            _ => "unknown".to_string(),
        }
    }
}

pub fn module(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let exports = lua.create_table()?;
    exports.set("string", Type::STRING)?;
    exports.set("number", Type::NUMBER)?;
    exports.set("boolean", Type::BOOLEAN)?;
    exports.set("table", Type::TABLE)?;
    // exports.set("enum", Type::ENUM)?;
    // exports.set("struct", Type::r#struct)?;
    exports.set("nil", Type::NIL)?;
    exports.set("struct", lua.create_function(r#struct::r#struct)?)?;
    exports.set("enum", lua.create_function(r#enum::r#enum)?)?;
    Ok(exports)
}
