use std::{
    borrow::Borrow,
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{
    syntax::{self, Assign, Chunk, Exp, ExpList, FnCall, Stat, Var},
    Script, Value,
};

//

pub struct State {
    global_vars: Table,
    local_vars: Vec<Table>,
}

//

impl State {
    pub const fn new() -> Self {
        Self {
            global_vars: Table::new(),
            local_vars: vec![],
        }
    }

    pub fn run<'a, S>(&mut self, relua: S) -> Result<(), &'static str>
    where
        S: Into<Script<'a>>,
    {
        match relua.into() {
            Script::Interpret(script) => {
                let chunk = syntax::parse(script);
                run_chunk(self, chunk);
            }
        };

        Ok(())
    }
}

impl Deref for State {
    type Target = Table;

    fn deref(&self) -> &Self::Target {
        &self.global_vars
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_vars
    }
}

//

pub struct Table {
    fields: BTreeMap<Rc<str>, Value>,
}

impl Table {
    pub const fn new() -> Self {
        Self {
            fields: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, var: impl Into<Rc<str>>, val: impl Into<Value>) {
        self.fields.insert(var.into(), val.into());
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        Q: ?Sized + Ord,
        Rc<str>: Borrow<Q>,
    {
        self.fields.get(key)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
    where
        Q: ?Sized + Ord,
        Rc<str>: Borrow<Q>,
    {
        self.fields.get_mut(key)
    }

    pub fn entry(&mut self, key: impl Into<Rc<str>>) -> &mut Value {
        self.fields.entry(key.into()).or_default()
    }
}

//

pub fn run_chunk(state: &mut State, chunk: Chunk) {
    for stat in chunk.statements {
        run_stat(state, stat);
    }
}

pub fn run_stat(state: &mut State, stat: Stat) {
    match stat {
        Stat::Assign(assign) => run_assign(state, assign),
        Stat::FnCall(fncall) => _ = run_fncall(state, fncall),
    }
}

pub fn run_assign(state: &mut State, assign: Assign) {
    assert_eq!(
        assign.explist.0.len(),
        assign.varlist.0.len(),
        "varlist length should be the same as explist length"
    );

    for (var, val) in assign
        .varlist
        .0
        .into_iter()
        .zip(run_explist(state, assign.explist))
    {
        *run_var(state, var) = val;
    }
}

pub fn run_explist(state: &mut State, explist: ExpList) -> Vec<Value> {
    explist
        .0
        .into_iter()
        .map(|exp| run_exp(state, exp))
        .collect()
}

pub fn run_var(state: &mut State, var: Var) -> &mut Value {
    if let Some(local) = state.local_vars.last_mut() {
        if let Some(val) = local.get_mut(var.0.as_str()) {
            return val;
        }
    }

    state.global_vars.entry(var.0.as_str())
}

pub fn run_exp(state: &mut State, exp: Exp) -> Value {
    match exp {
        Exp::Value(v) => v,
        Exp::Var(var) => run_var(state, var).clone(),
        Exp::FnCall(fncall) => run_fncall(state, fncall),
    }
}

pub fn run_fncall(state: &mut State, fncall: FnCall) -> Value {
    let func = run_var(state, fncall.func).clone();

    let func = match func {
        Value::Function(fn_id) => fn_id,
        _ => panic!("attempt to call `{}`", func.as_type()),
    };

    let args = fncall
        .args
        .explist
        .0
        .into_iter()
        .map(|exp| run_exp(state, exp))
        .collect();

    (func.0)(args)
}
