use crate::core::{Parser, Token};
use crate::expression::{Expression, NameExpression, PrefixExpression};

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

// We can use a single struct for all the prefix operators since they only differ in the actual operator token itself
pub struct PrefixOperatorParselet {
    precedence: usize,
}

impl PrefixOperatorParselet {
    pub fn new(precedence: usize) -> Self {
        Self { precedence }
    }
}

impl PrefixParselet for PrefixOperatorParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Box<dyn Expression> {
        let operand = parser.parse_expression_precedence(self.precedence);
        return Box::new(PrefixExpression::new(*token.get_type(), operand));
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
    fn get_precedence(&self) -> usize;
}
