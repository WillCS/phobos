use super::table::LuaTable;
use super::function::LuaFunction;

pub enum LuaValue {
    Nil,
    Boolean(bool),
    Number(LuaNumber),
    LuaString(String),
    Function(Box<LuaFunction>),
    Userdata,
    Thread,
    Table(Box<LuaTable>)
}

impl LuaValue {
    pub fn name(&self) -> &'static str {
        match self {
            LuaValue::Nil          => "nil",
            LuaValue::Boolean(_)   => "boolean",
            LuaValue::Number(_)    => "number",
            LuaValue::LuaString(_) => "string",
            LuaValue::Function(_)  => "function",
            LuaValue::Thread       => "thread",
            LuaValue::Userdata     => "userdata",
            LuaValue::Table(_)     => "table"
        }        
    }
}

pub struct LuaNumber { }
