use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Comma,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    Tilde,
    Bang,
    Question,
    Colon,
    Name,
    EOF,
}

impl TokenType {
    pub fn punctuator(&self) -> Option<char> {
        match &self {
            TokenType::LeftParen => Some('('),
            TokenType::RightParen => Some(')'),
            TokenType::Comma => Some(','),
            TokenType::Assign => Some('='),
            TokenType::Plus => Some('+'),
            TokenType::Minus => Some('-'),
            TokenType::Asterisk => Some('*'),
            TokenType::Slash => Some('/'),
            TokenType::Caret => Some('^'),
            TokenType::Tilde => Some('~'),
            TokenType::Bang => Some('!'),
            TokenType::Question => Some('?'),
            TokenType::Colon => Some(':'),
            _ => None,
        }
    }

    pub fn values() -> Vec<TokenType> {
        Vec::from([
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::Comma,
            TokenType::Assign,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Asterisk,
            TokenType::Slash,
            TokenType::Caret,
            TokenType::Tilde,
            TokenType::Bang,
            TokenType::Question,
            TokenType::Colon,
            TokenType::Name,
            TokenType::EOF,
        ])
    }
}

pub struct Token {
    token_type: TokenType,
    text: String,
}

impl Token {
    pub fn new(token_type: TokenType, text: String) -> Self {
        Self { token_type, text }
    }

    pub fn get_type(self) -> TokenType {
        self.token_type
    }

    pub fn get_text(self) -> String {
        self.text
    }
}

/*
 * Defines the different precedence levels used by the infix parsers. These
 * determine how a series of infix expressions will be grouped. For example,
 * "a + b * c - d" will be parsed as "(a + (b * c)) - d" because "*" has higher
 * precedence than "+" and "-". Here, bigger numbers mean higher precedence.
 */
pub enum Precedence {
    Assignment = 1,
    Conditional = 2,
    Sum = 3,
    Product = 4,
    Exponent = 5,
    Prefix = 6,
    Postfix = 7,
    Call = 8,
}

/*
 * A very primitive lexer. Takes a string and splits it into a series of
 * Tokens. Operators and punctuation are mapped to unique keywords. Names,
 * which can be any series of letters, are turned into NAME tokens. All other
 * characters are ignored (except to separate names). Numbers and strings are
 * not supported. This is really just the bare minimum to give the parser
 * something to work with.
 */
pub struct Lexer {
    index: usize,
    text: Vec<char>,
    punctuators: HashMap<char, TokenType>,
}

impl Lexer {
    pub fn new(text_input: String) -> Self {
        let mut punctuators: HashMap<char, TokenType> = HashMap::new();

        for tt in TokenType::values() {
            if let Some(x) = tt.punctuator() {
                punctuators.insert(x, tt).unwrap();
            }
        }

        Self {
            index: 0,
            text: text_input.chars().collect(),
            punctuators,
        }
    }

    pub fn has_next(self) -> bool {
        true
    }

    pub fn next(&mut self) -> Token {
        while self.index < self.text.len() {
            self.index += 1;
            let c = self.text.get(self.index).unwrap();
            if self.punctuators.contains_key(c) {
                return Token::new(*self.punctuators.get(c).unwrap(), String::from(*c));
            } else if c.is_alphabetic() {
                let start = self.index - 1;
                while self.index < self.text.len() {
                    if !self.text.get(self.index).unwrap().is_alphabetic() {
                        break;
                    }
                    self.index += 1;
                }

                let name: String = String::from_iter(&self.text[start..self.index].to_vec());
                return Token::new(TokenType::Name, name);
            } else {
                // Ignore all other chars (whitespace etc.)
            }
        }

        // Once we've reached the end of the string, just return EOF tokens. We'll
        // just keeping returning them as many times as we're asked so that the
        // parser's lookahead doesn't have to worry about running out of tokens.
        Token::new(TokenType::EOF, String::from(""))
    }
}

pub struct Expression {}

// #[derive(Debug)]
// pub enum TokenType {
//     NAME = 0,
//     PLUS = 1,
//     MINUS = 2,
//     TILDE = 3,
//     BANG = 4,
// }
//
// // Map from TokenTypes to chunks of parsing code (parselets)
// trait PrefixParselet {
//     fn parse(parser: Parser, token: Token) -> Expression;
// }
//
// pub struct Parser {
//     m_prefix_parselets: HashMap<TokenType, dyn PrefixParselet>,
// }
//
// impl Parser {
//     pub fn new() -> Self {
//         Self {
//             m_prefix_parselets: HashMap::new(),
//         }
//     }
//     pub fn parseExpression(self) -> Result<Expression, Box<dyn Error>> {
//         let token: Token = consume();
//         let prefix: PrefixParselet = self
//             .m_prefix_parselets
//             .get(token.getType())
//             .except("Could not parse token");
//
//         return Ok(prefix.parse(token));
//     }
// }
//
// struct ConditionalExpression {}
//
// struct NameParselet {}
//
// impl PrefixParselet for NameParselet {
//     fn parse(parser: Parser, token: Token) -> Expression {
//         return NameExpression::new(token.getText());
//     }
// }
//
// struct PrefixOperationParselet {}
//
// // Single implementation for all prefix ops since they only differ in the actual operator token
// // Call back into parseExpression to parse the operand that appears after operator (parse the a
// // in -a)
// // The recursion takes care of nested operators
// impl PrefixParselet for PrefixOperationParselet {
//     fn parse(parser: Parser, token: Token) -> Expression {
//         let operand: Expression = parser.parseExpression().unwrap();
//         return PrefixExpression::new(token.getType(), operand);
//     }
// }
