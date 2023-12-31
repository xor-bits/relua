use core::fmt;
use std::rc::Rc;

pub use engine::interpreter::State;

//

pub mod engine;
pub mod syntax;

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

#[derive(Clone)]
pub struct Function(Rc<dyn Fn(Vec<Value>) -> Value>);

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<>")
    }
}

impl<F: Fn(Vec<Value>) -> R + 'static, R: Into<Value>> From<F> for Function {
    fn from(value: F) -> Self {
        Function(Rc::new(move |args| value(args).into()) as _)
    }
}

//

#[derive(Debug, Clone, Default)]
pub enum Value {
    #[default]
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Function(Function),
    // Userdata,
    // Function,
    // Thread,
    // Table,
}

impl Value {
    pub const fn as_type(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Boolean(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Function(_) => "function",
        }
    }

    /// Returns `true` if the value is [`Nil`].
    ///
    /// [`Nil`]: Value::Nil
    #[must_use]
    pub const fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    /// Returns `true` if the value is [`Boolean`].
    ///
    /// [`Boolean`]: Value::Boolean
    #[must_use]
    pub const fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(..))
    }

    /// Returns `true` if the value is [`Number`].
    ///
    /// [`Number`]: Value::Number
    #[must_use]
    pub const fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    /// Returns `true` if the value is [`String`].
    ///
    /// [`String`]: Value::String
    #[must_use]
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }

    /// Returns `true` if the value is [`Function`].
    ///
    /// [`Function`]: Value::Function
    #[must_use]
    pub const fn is_function(&self) -> bool {
        matches!(self, Self::Function(..))
    }

    #[must_use]
    pub const fn as_boolean(&self) -> Option<&bool> {
        if let Self::Boolean(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_number(&self) -> Option<&f64> {
        if let Self::Number(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_string(&self) -> Option<&String> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_function(&self) -> Option<&Function> {
        if let Self::Function(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(v) => write!(f, "{v}"),
            Value::Number(v) => write!(f, "{v}"),
            Value::String(v) => write!(f, "{v:?}"),
            Value::Function(_) => write!(f, "<function>"),
        }
    }
}

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

impl<F: Into<Function>> From<F> for Value {
    fn from(v: F) -> Self {
        Self::Function(v.into())
    }
}
