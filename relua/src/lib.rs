use std::{borrow::Borrow, collections::BTreeMap, rc::Rc};

//

pub mod engine;
pub mod syntax;

//

pub struct State {
    vars: BTreeMap<Rc<str>, Value>,
}

//

impl State {
    pub const fn new() -> Self {
        Self {
            vars: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, var: impl Into<Rc<str>>, val: impl Into<Value>) {
        self.vars.insert(var.into(), val.into());
    }

    pub fn get<Q>(&self, key: &Q) -> Value
    where
        Q: ?Sized + Ord,
        Rc<str>: Borrow<Q>,
    {
        self.vars.get(key).cloned().unwrap_or(Value::Nil)
    }

    pub fn run<'a, S>(&mut self, relua: S) -> Result<(), &'static str>
    where
        S: Into<Script<'a>>,
    {
        match relua.into() {
            Script::Interpret(script) => syntax::parse(script),
        };

        Ok(())
    }
}

//

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Script<'a> {
    Interpret(&'a str),
}

impl<'a> From<&'a str> for Script<'a> {
    fn from(value: &'a str) -> Self {
        Script::Interpret(value)
    }
}

//

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    // Userdata,
    // Function,
    // Thread,
    // Table,
}

//

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::Nil
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::Number(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Self::Number(v as _)
    }
}

impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Self::Number(v as _)
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Self::Number(v as _)
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Self::Number(v as _)
    }
}

impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Self::Number(v as _)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self::Number(v as _)
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Self::Number(v as _)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Number(v as _)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Self::Number(v as _)
    }
}

impl From<isize> for Value {
    fn from(v: isize) -> Self {
        Self::Number(v as _)
    }
}

impl From<usize> for Value {
    fn from(v: usize) -> Self {
        Self::Number(v as _)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

/* impl<V: Into<Value>> From<Option<V>> for Value {
    fn from(v: Option<V>) -> Self {
        v.map(f)
        if let Some(v) =
        Self::Nil
    }
} */
