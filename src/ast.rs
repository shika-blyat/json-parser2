#[derive(Debug)]
pub struct Object {
    pub values: Vec<PairStringValue>,
}

#[derive(Debug)]
pub struct PairStringValue {
    pub name: String,
    pub value: Value,
}

#[derive(Debug)]
pub struct Array {
    pub values: Vec<Value>,
}
#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(i32),
    String(String),
    Array(Array),
    Object(Object),
}
