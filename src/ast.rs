use std::{fmt, rc::Rc};

pub struct Pair<T, U> {
    head: T,
    tail: U,
}

pub type Complement = Pair<Value, Value>;

// struct TypeDescriptor {
//     name: Option<Rc<Word>>,
// }

#[derive(Clone)]
pub enum Value {
    /// The empty `Value`
    Nil,
    /// A series of letters or phonemes or something
    Word(Rc<Word>),
    /// A pair of a `Value` and its complement
    Complement(Rc<Complement>),
    /// An operation which can be performed on an object
    Verb(Rc<dyn Verb>),
    /// What it says on the tin, honestly
    Number(f64),
    Sequence(Rc<Vec<Value>>),
    // TypeDescriptor<Rc<TypeDescriptor>>,
}

impl Value {
    pub fn make_word(word: &str) -> Value {
        Rc::new(Word(word.to_owned())).into()
    }
    pub fn make_complement(head: Value, tail: Vec<Value>) -> Value {
        match &tail[..] {
            [] => head,
            [tail] => {
                Rc::new(Pair { head, tail: tail.clone() }).into()
            }
            _ => {
                let tail = Value::make_sequence(tail);
                Rc::new(Pair { head, tail }).into()
            }
        }
    }
    pub fn make_sequence(contents: Vec<Value>) -> Value {
        Value::Sequence(Rc::new(contents))
    }
}

pub trait Verb {
    fn apply(&self, complement: Value) -> Value;
    fn name(&self) -> Option<Rc<Word>>;
    // fn check_type(&self, complement: Value) -> Rc<TypeDescriptor>;
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        
        match (self, other) {
            (Value::Word(lhs), Value::Word(rhs)) => lhs == rhs,
            (Value::Complement(lhs), Value::Complement(rhs)) => Rc::ptr_eq(lhs, rhs),
            (Value::Verb(lhs), Value::Verb(rhs)) => Rc::ptr_eq(lhs, rhs),
            (Value::Number(lhs), Value::Number(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "NOTHING"),
            Value::Word(r) => fmt::Debug::fmt(r, f),
            Value::Complement(c) => fmt::Debug::fmt(c, f),
            Value::Verb(v) => if let Some(name) = v.name() {
                fmt::Debug::fmt(&name, f)
            } else {
                write!(f, "FORBIDDENMAGIC")
            },
            Value::Number(n) => fmt::Debug::fmt(n, f),
            Value::Sequence(s) => fmt::Debug::fmt(s, f),
        }
    }
}

impl From<f64> for Value { fn from(f: f64) -> Value { Value::Number(f) } }
impl From<Rc<Word>> for Value { fn from(w: Rc<Word>) -> Value { Value::Word(w) } }
impl From<Rc<Complement>> for Value { fn from(c: Rc<Complement>) -> Value { Value::Complement(c) } }
impl From<Rc<dyn Verb>> for Value { fn from(v: Rc<dyn Verb>) -> Value { Value::Verb(v) } }

#[derive(PartialEq, Eq)]
pub struct Word(String);

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Debug for Complement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?} . {:?})", self.head, self.tail)
    }
}
