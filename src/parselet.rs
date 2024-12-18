use crate::core::{Parser, Precedence, Token, TokenType};
use crate::expression::{
    AssignExpression, CallExpression, ConditionalExpression, Expression, NameExpression,
    OperatorExpression, PostfixExpression, PrefixExpression,
};

// One of the two interfaces used by the Pratt parser. A PrefixParselet is
// associated with a token that appears at the beginning of an expression. Its
// parse() method will be called with the consumed leading token, and the
// parselet is responsible for parsing anything that comes after that token.
// This interface is also used for single-token expressions like variables, in
// which case parse() simply doesn't consume any more tokens.
// @author rnystrom
pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Box<dyn Expression>;
}

// Bantam has one single-token expression: named variables
pub struct NameParselet {}

// We can use a single struct for all the prefix operators since they only differ in the actual operator token itself
pub struct PrefixOperatorParselet {
    precedence: Precedence,
}

// Parses Parentheses used to group an expression `a * (b + c)`
pub struct GroupParselet {}

impl NameParselet {
    pub fn new() -> Self {
        Self {}
    }
}

// Parselet implementation to parse variables names
impl PrefixParselet for NameParselet {
    fn parse(&self, _parser: &mut Parser, token: Token) -> Box<dyn Expression> {
        Box::new(NameExpression::new(token.text))
    }
}

impl PrefixOperatorParselet {
    pub fn new(precedence: Precedence) -> Self {
        Self { precedence }
    }
}

impl PrefixParselet for PrefixOperatorParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Box<dyn Expression> {
        let operand = parser.parse_expression_precedence(self.precedence.clone());
        return Box::new(PrefixExpression::new(*token.get_type(), operand));
    }
}

impl GroupParselet {
    pub fn new() -> Self {
        Self {}
    }
}

impl PrefixParselet for GroupParselet {
    fn parse(&self, parser: &mut Parser, _token: Token) -> Box<dyn Expression> {
        let expr = parser.parse_expression();
        parser.consume_expected(TokenType::RightParen);
        expr
    }
}

// One of the two parselet interfaces used by the Pratt parser. An
// InfixParselet is associated with a token that appears in the middle of the
// expression it parses. Its parse() method will be called after the left-hand
// side has been parsed, and it in turn is responsible for parsing everything
// that comes after the token. This is also used for postfix expressions, in
// which case it simply doesn't consume any more tokens in its parse() call.
pub trait InfixParselet {
    fn parse(
        &self,
        parser: &mut Parser,
        left: Box<dyn Expression>,
        token: Token,
    ) -> Box<dyn Expression>;
    fn get_precedence(&self) -> Precedence;
}

pub struct BinaryOperatorParselet {
    precedence: Precedence,
    is_right: bool,
}

pub struct PostfixOperatorParselet {
    precedence: Precedence,
}

// a ? b : c
pub struct ConditionalParselet {}

// a = b
// left side has to be a name
// expressions are right-associative
// a = b = c becomes a = (b = c)
pub struct AssignParselet {}

pub struct CallParselet {}

impl BinaryOperatorParselet {
    pub fn new(precedence: Precedence, is_right: bool) -> Self {
        Self {
            precedence,
            is_right,
        }
    }
}

impl InfixParselet for BinaryOperatorParselet {
    fn parse(
        &self,
        parser: &mut Parser,
        left: Box<dyn Expression>,
        token: Token,
    ) -> Box<dyn Expression> {
        let op_prec = self.precedence as usize - if self.is_right { 1 } else { 0 };
        let right = parser.parse_expression_precedence(op_prec.into());

        Box::new(OperatorExpression::new(left, *token.get_type(), right))
    }

    fn get_precedence(&self) -> Precedence {
        self.precedence
    }
}

impl PostfixOperatorParselet {
    pub fn new(precedence: Precedence) -> Self {
        Self { precedence }
    }
}

impl InfixParselet for PostfixOperatorParselet {
    fn parse(
        &self,
        _parser: &mut Parser,
        left: Box<dyn Expression>,
        token: Token,
    ) -> Box<dyn Expression> {
        Box::new(PostfixExpression::new(left, *token.get_type()))
    }

    fn get_precedence(&self) -> Precedence {
        self.precedence
    }
}

impl ConditionalParselet {
    pub fn new() -> Self {
        Self {}
    }
}

impl InfixParselet for ConditionalParselet {
    fn parse(
        &self,
        parser: &mut Parser,
        left: Box<dyn Expression>,
        _token: Token,
    ) -> Box<dyn Expression> {
        let then_arm = parser.parse_expression();
        parser.consume_expected(TokenType::Colon);

        let else_prec = Precedence::Conditional as usize - 1;
        let else_arm = parser.parse_expression_precedence(else_prec.into());
        Box::new(ConditionalExpression::new(left, then_arm, else_arm))
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Conditional
    }
}

impl AssignParselet {
    pub fn new() -> Self {
        Self {}
    }
}

impl InfixParselet for AssignParselet {
    fn parse(
        &self,
        parser: &mut Parser,
        left: Box<dyn Expression>,
        _token: Token,
    ) -> Box<dyn Expression> {
        let right_prec = Precedence::Assignment as usize - 1;
        let right = parser.parse_expression_precedence(right_prec.into());

        let left_name_expr = match left.as_any().downcast_ref::<NameExpression>() {
            Some(ne) => ne,
            None => panic!("left hand side of assignment must be a name"),
        };

        let name = left_name_expr.name();
        Box::new(AssignExpression::new(name.clone(), right))
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Assignment
    }
}

impl CallParselet {
    pub fn new() -> Self {
        Self {}
    }
}

impl InfixParselet for CallParselet {
    fn parse(
        &self,
        parser: &mut Parser,
        left: Box<dyn Expression>,
        _token: Token,
    ) -> Box<dyn Expression> {
        let mut args = Vec::new();

        // Could be no args
        if !parser.match_tok(TokenType::RightParen) {
            loop {
                args.push(parser.parse_expression());

                if !parser.match_tok(TokenType::Comma) {
                    break;
                }
            }
            parser.consume_expected(TokenType::RightParen);
        }

        Box::new(CallExpression::new(left, args))
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Call
    }
}
