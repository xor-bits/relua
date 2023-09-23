use lalrpop_util::lalrpop_mod;

use self::relua::NumberParser;

//

lalrpop_mod!(pub relua, "/syntax/relua.rs");

//

pub fn parse(s: &str) {
    let n = NumberParser::new().parse(s).unwrap();
    tracing::info!("AST: {n:#?}");
}
