use std::any::Any;

use crate::core::TokenType;

pub trait Expression {
    fn print(&self, builder: &mut String) -> ();

    fn as_any(&self) -> &dyn Any;
}

pub struct NameExpression {
    name: String,
}

impl NameExpression {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Expression for NameExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str(&self.name);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct PrefixExpression {
    operator: TokenType,
    right: Box<dyn Expression>,
}

impl PrefixExpression {
    pub fn new(tt: TokenType, right: Box<dyn Expression>) -> Self {
        Self {
            operator: tt,
            right,
        }
    }
}

impl Expression for PrefixExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str("(");
        builder.push(self.operator.punctuator().unwrap());
        self.right.print(builder);
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
