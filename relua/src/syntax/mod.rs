use lalrpop_util::lalrpop_mod;

use crate::Value;

//

lalrpop_mod!(pub relua);

//

pub fn parse(s: &str) -> Chunk {
    relua::ChunkParser::new().parse(s).unwrap()
}

//

#[derive(Debug, Clone)]
pub struct Chunk {
    pub statements: Vec<Stat>,
}

#[derive(Debug, Clone)]
pub enum Stat {
    Assign(Assign),
    FnCall(FnCall),
    // Control,
    // FnCall,
    // Decl,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub varlist: VarList,
    pub explist: ExpList,
}

#[derive(Debug, Clone)]
pub struct VarList(pub Vec<Var>);

#[derive(Debug, Clone)]
pub struct ExpList(pub Vec<Exp>);

#[derive(Debug, Clone)]
pub struct Var(pub String);

#[derive(Debug, Clone)]
pub enum Exp {
    Value(Value),
    Var(Var),
    FnCall(FnCall),
}

#[derive(Debug, Clone)]
pub struct FnCall {
    // pub func: Box<Exp>,
    pub func: Var,
    pub args: Args,
}

#[derive(Debug, Clone)]
pub struct Args {
    pub explist: ExpList,
}
