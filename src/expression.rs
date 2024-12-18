use std::any::Any;

use crate::core::TokenType;

pub trait Expression {
    fn print(&self, builder: &mut String) -> ();

    fn as_any(&self) -> &dyn Any;
}

pub struct NameExpression {
    name: String,
}

pub struct PrefixExpression {
    operator: TokenType,
    right: Box<dyn Expression>,
}

pub struct OperatorExpression {
    left: Box<dyn Expression>,
    operator: TokenType,
    right: Box<dyn Expression>,
}

pub struct PostfixExpression {
    left: Box<dyn Expression>,
    operator: TokenType,
}

pub struct ConditionalExpression {
    condition: Box<dyn Expression>,
    then_arm: Box<dyn Expression>,
    else_arm: Box<dyn Expression>,
}

pub struct AssignExpression {
    name: String,
    right: Box<dyn Expression>,
}

pub struct CallExpression {
    function: Box<dyn Expression>,
    args: Vec<Box<dyn Expression>>,
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

impl OperatorExpression {
    pub fn new(left: Box<dyn Expression>, operator: TokenType, right: Box<dyn Expression>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Expression for OperatorExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str("(");
        self.left.print(builder);
        builder.push_str(" ");
        builder.push(self.operator.punctuator().unwrap());
        builder.push_str(" ");
        self.right.print(builder);
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PostfixExpression {
    pub fn new(left: Box<dyn Expression>, operator: TokenType) -> Self {
        Self { left, operator }
    }
}

impl Expression for PostfixExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str("(");
        self.left.print(builder);
        builder.push(self.operator.punctuator().unwrap());
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ConditionalExpression {
    pub fn new(
        condition: Box<dyn Expression>,
        then_arm: Box<dyn Expression>,
        else_arm: Box<dyn Expression>,
    ) -> Self {
        Self {
            condition,
            then_arm,
            else_arm,
        }
    }
}

impl Expression for ConditionalExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str("(");
        self.condition.print(builder);
        builder.push_str(" ? ");
        self.then_arm.print(builder);
        builder.push_str(" : ");
        self.else_arm.print(builder);
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AssignExpression {
    pub fn new(name: String, right: Box<dyn Expression>) -> Self {
        Self { name, right }
    }
}

impl Expression for AssignExpression {
    fn print(&self, builder: &mut String) -> () {
        builder.push_str("(");
        builder.push_str(&self.name);
        builder.push_str(" = ");
        self.right.print(builder);
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CallExpression {
    pub fn new(function: Box<dyn Expression>, args: Vec<Box<dyn Expression>>) -> Self {
        Self { function, args }
    }
}

impl Expression for CallExpression {
    fn print(&self, builder: &mut String) -> () {
        self.function.print(builder);
        builder.push_str("(");
        let mut i = 0;
        for arg in self.args.iter() {
            arg.print(builder);
            if i + 1 < self.args.len() {
                builder.push_str(", ");
            }
            i += 1;
        }
        builder.push_str(")");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
