use super::super::syntax_tree::LuaFunctionDef;

pub enum LuaFunction {
    Native(),
    Embedded(LuaFunctionDef)
}
