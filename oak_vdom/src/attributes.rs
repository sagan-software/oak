use super::Text;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Attributes(BTreeMap<Text, AttributeValue>);

impl std::fmt::Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, (key, val)) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            match val {
                AttributeValue::Text(_) => {
                    write!(f, "{}=\"{}\"", key, val)?;
                }
                AttributeValue::Bool(show) => {
                    if *show {
                        write!(f, "{}", key)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Attributes {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert<A>(&mut self, attr: A)
    where
        A: Into<Attribute>,
    {
        let attr = attr.into();
        self.0.insert(attr.0, attr.1);
    }

    // pub fn get<K>(&mut self, key: K) -> Option<Attribute>
    // where
    //     K: Into<Text>,
    // {
    //     let key = key.into();
    //     let val = self.0.get(&key);
    //     Attribute(key, val);
    // }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AttributeValue {
    Text(Text),
    Bool(bool),
}

impl std::fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AttributeValue::Text(text) => write!(f, "{}", text), // TODO escape
            AttributeValue::Bool(show) => write!(f, "{}", show),
        }
    }
}

impl<T> From<T> for AttributeValue
where
    T: Into<Text>,
{
    fn from(text: T) -> Self {
        AttributeValue::Text(text.into())
    }
}

impl From<bool> for AttributeValue {
    fn from(val: bool) -> Self {
        AttributeValue::Bool(val)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute(Text, AttributeValue);

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.1 {
            AttributeValue::Text(_) => write!(f, "{}=\"{}\"", self.0, self.1),
            AttributeValue::Bool(show) => {
                if *show {
                    write!(f, "{}", self.0)
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl<K, V> From<(K, V)> for Attribute
where
    K: Into<Text>,
    V: Into<AttributeValue>,
{
    fn from((key, val): (K, V)) -> Self {
        Attribute(key.into(), val.into())
    }
}

impl<K> From<K> for Attribute
where
    K: Into<Text>,
{
    fn from(key: K) -> Self {
        Attribute(key.into(), AttributeValue::Bool(true))
    }
}

#[macro_export]
macro_rules! declare_text_attributes {
    ($($t1:ident, $t2:expr)*) => ($(
        pub fn $t1<V: Into<AttributeValue>>(val: V) -> Attribute {
            Attribute($t2.into(), val.into())
        }
    )*);

    ($($x:ident)*) => ($(
        declare_text_attributes!($x, stringify!($x));
    )*);
}

#[macro_export]
macro_rules! declare_bool_attributes {
    ($($t1:ident, $t2:expr)*) => ($(
        pub fn $t1(val: bool) -> Attribute {
            Attribute($t2.into(), AttributeValue::Bool(val))
        }
    )*);

    ($($x:ident)*) => ($(
        declare_bool_attributes!($x, stringify!($x));
    )*);
}

declare_text_attributes! {
    placeholder
    name
    value
    id
    href
    class
    src
}

declare_text_attributes! {
    type_, "type"
    for_, "for"
}

declare_bool_attributes! {
    autofocus
    checked
    hidden
}
