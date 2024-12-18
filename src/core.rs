use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::{
    expression::Expression,
    parselet::{
        AssignParselet, BinaryOperatorParselet, CallParselet, ConditionalParselet, GroupParselet,
        InfixParselet, NameParselet, PostfixOperatorParselet, PrefixOperatorParselet,
        PrefixParselet,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
        match *self {
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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Assign => write!(f, "ASSIGN"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Asterisk => write!(f, "ASTERISK"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::Caret => write!(f, "CARET"),
            TokenType::Tilde => write!(f, "TILDE"),
            TokenType::Bang => write!(f, "BANG"),
            TokenType::Question => write!(f, "QUESTION"),
            TokenType::Colon => write!(f, "COLON"),
            TokenType::Name => write!(f, "NAME"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    pub text: String,
}

impl Token {
    pub fn new(token_type: TokenType, text: String) -> Self {
        Self { token_type, text }
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \"{}\"", self.token_type, self.text)
    }
}

// Defines the different precedence levels used by the infix parsers. These
// determine how a series of infix expressions will be grouped. For example,
// "a + b * c - d" will be parsed as "(a + (b * c)) - d" because "*" has higher
// precedence than "+" and "-". Here, bigger numbers mean higher precedence.
#[derive(PartialEq, PartialOrd, Clone, Debug, Copy)]
pub enum Precedence {
    Everything = 0,
    Assignment = 1,
    Conditional = 2,
    Sum = 3,
    Product = 4,
    Exponent = 5,
    Prefix = 6,
    Postfix = 7,
    Call = 8,
}

impl From<usize> for Precedence {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Everything,
            1 => Self::Assignment,
            2 => Self::Conditional,
            3 => Self::Sum,
            4 => Self::Product,
            5 => Self::Exponent,
            6 => Self::Prefix,
            7 => Self::Postfix,
            8 => Self::Call,
            _ => panic!("Invalid precedence value"),
        }
    }
}

// A very primitive lexer. Takes a string and splits it into a series of
// Tokens. Operators and punctuation are mapped to unique keywords. Names,
// which can be any series of letters, are turned into NAME tokens. All other
// characters are ignored (except to separate names). Numbers and strings are
// not supported. This is really just the bare minimum to give the parser
// something to work with.
#[derive(Debug, Clone)]
pub struct Lexer {
    index: usize,
    text: Vec<char>,
    punctuators: HashMap<char, TokenType>,
}

impl Lexer {
    pub fn new(text_input: String) -> Self {
        let mut punctuators: HashMap<char, TokenType> = HashMap::new();

        // Register TokenTypes that are explicit punctuators
        for tt in TokenType::values() {
            if let Some(x) = tt.punctuator() {
                punctuators.insert(x, tt);
            }
        }

        println!("{}", text_input);

        Self {
            index: 0,
            text: text_input.chars().collect(),
            punctuators,
        }
    }

    pub fn has_next(&self) -> bool {
        self.index < self.text.len()
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.text.len() {
            let c = self.text.get(self.index).unwrap();
            self.index += 1;

            if self.punctuators.contains_key(&c) {
                return Some(Token::new(
                    *self.punctuators.get(c).unwrap(),
                    String::from(*c),
                ));
            } else if c.is_alphabetic() {
                let start = self.index - 1;
                while self.index < self.text.len() {
                    if !self.text.get(self.index).unwrap().is_alphabetic() {
                        break;
                    }
                    self.index += 1;
                }

                let name: String = self.text[start..self.index].iter().collect();
                return Some(Token::new(TokenType::Name, name));
            } else {
                // Ignore all other chars (whitespace etc.)
                continue;
            }
        }

        // Once we've reached the end of the string, just return EOF tokens. We'll
        // just keeping returning them as many times as we're asked so that the
        // parser's lookahead doesn't have to worry about running out of tokens.
        Some(Token::new(TokenType::EOF, String::new()))
    }
}

pub struct Parser {
    tokens: Box<dyn Iterator<Item = Token>>,
    read: Vec<Token>,

    // We have separate tables for prefix and infix expressions because sometimes we have both a prefix and infix parselet for the same TokenType. For example, the prefix parselet for `(` handles grouping in an expression like `a * (b + c)`. Meanwhile the infix parselet for `(` handles function calls like `a(b)`
    prefix_parselets: HashMap<TokenType, Rc<dyn PrefixParselet>>,
    infix_parselets: HashMap<TokenType, Rc<dyn InfixParselet>>,
}

impl Parser {
    pub fn new(tokens: Box<dyn Iterator<Item = Token>>) -> Self {
        Self {
            tokens,
            read: Vec::new(),
            prefix_parselets: HashMap::new(),
            infix_parselets: HashMap::new(),
        }
    }

    pub fn register_prefix(&mut self, tt: TokenType, parselet: Box<dyn PrefixParselet>) -> () {
        self.prefix_parselets.insert(tt, Rc::from(parselet));
    }

    pub fn register_infix(&mut self, tt: TokenType, parselet: Box<dyn InfixParselet>) -> () {
        self.infix_parselets.insert(tt, Rc::from(parselet));
    }

    pub fn parse_expression_precedence(&mut self, precedence: Precedence) -> Box<dyn Expression> {
        let mut token: Token = self.consume();
        println!("{}", token);
        let prefix = self
            .prefix_parselets
            .get(token.get_type())
            .expect(&format!("Could not parse {}.", token.get_text()))
            .clone();

        let mut left = prefix.parse(self, token);

        // if parse_expression() encounters an expression whose precedence is lower than we allow, it stops parsing and returns what it has so far
        while precedence < self.get_precedence() {
            token = self.consume();
            let infix = self.infix_parselets.get(token.get_type()).unwrap().clone();
            left = infix.parse(self, left, token);
        }

        left
    }

    pub fn parse_expression(&mut self) -> Box<dyn Expression> {
        self.parse_expression_precedence(Precedence::Everything)
    }

    // Since match is a keyword
    pub fn match_tok(&mut self, expected: TokenType) -> bool {
        let token = self.look_ahead(0);
        if *token.get_type() != expected {
            // panic!("Expected {} and found {}", expected, token.get_type());
            false
        } else {
            self.consume();
            true
        }
    }

    pub fn consume_expected(&mut self, expected: TokenType) -> Token {
        let tok = self.look_ahead(0);
        if *tok.get_type() != expected {
            panic!("Expect token {} and found {}", expected, tok.get_type());
        }

        self.consume()
    }

    pub fn consume(&mut self) -> Token {
        self.look_ahead(0);
        self.read.remove(0)
    }

    fn look_ahead(&mut self, distance: usize) -> Token {
        while distance >= self.read.len() {
            self.read.push(self.tokens.next().unwrap());
        }

        return self.read[distance].clone();
    }

    // Helper function to get the precedence of the current token or a default value if there's no infix parselet for the token
    fn get_precedence(&mut self) -> Precedence {
        let tok_type: TokenType = *self.look_ahead(0).get_type();
        if let Some(infix_parser) = self.infix_parselets.get(&tok_type) {
            infix_parser.get_precedence()
        } else {
            Precedence::Everything
        }
    }
}

pub struct BantamParser {
    parser: Parser,
}

impl BantamParser {
    pub fn new(tokens: Box<dyn Iterator<Item = Token>>) -> Self {
        let mut bp = Self {
            parser: Parser::new(tokens),
        };

        // Register tokens that need special parselets
        bp.register_prefix(TokenType::Name, Box::new(NameParselet::new()));
        bp.register_infix(TokenType::Assign, Box::new(AssignParselet::new()));
        bp.register_infix(TokenType::Question, Box::new(ConditionalParselet::new()));
        bp.register_prefix(TokenType::LeftParen, Box::new(GroupParselet::new()));
        bp.register_infix(TokenType::LeftParen, Box::new(CallParselet::new()));

        // Register the simple operator parselets
        bp.prefix(TokenType::Plus, Precedence::Prefix);
        bp.prefix(TokenType::Minus, Precedence::Prefix);
        bp.prefix(TokenType::Tilde, Precedence::Prefix);
        bp.prefix(TokenType::Bang, Precedence::Prefix);

        // For kicks, we'll make "!" both prefix and postfix, kinda like ++
        bp.postfix(TokenType::Bang, Precedence::Postfix);

        bp.infix_left(TokenType::Plus, Precedence::Sum);
        bp.infix_left(TokenType::Minus, Precedence::Sum);
        bp.infix_left(TokenType::Asterisk, Precedence::Product);
        bp.infix_left(TokenType::Slash, Precedence::Product);
        bp.infix_right(TokenType::Caret, Precedence::Exponent);

        bp
    }

    pub fn register_prefix(&mut self, tt: TokenType, parselet: Box<dyn PrefixParselet>) -> () {
        self.parser.register_prefix(tt, parselet);
    }

    pub fn register_infix(&mut self, tt: TokenType, parselet: Box<dyn InfixParselet>) -> () {
        self.parser.register_infix(tt, parselet);
    }

    /// Register a prefix unary operator parselet for the given token and precedence
    pub fn prefix(&mut self, tt: TokenType, precedence: Precedence) -> () {
        self.register_prefix(tt, Box::new(PrefixOperatorParselet::new(precedence)));
    }

    /// Register a postfix unary operator parselet for the given token and precedence
    pub fn postfix(&mut self, tt: TokenType, precedence: Precedence) -> () {
        self.register_infix(tt, Box::new(PostfixOperatorParselet::new(precedence)));
    }

    /// Register a left-associative binary operator parselet for the given token and precedence
    pub fn infix_left(&mut self, tt: TokenType, precedence: Precedence) -> () {
        self.register_infix(tt, Box::new(BinaryOperatorParselet::new(precedence, false)));
    }

    /// Register a right-associative binary operator parselet for the given token and precedence
    pub fn infix_right(&mut self, tt: TokenType, precedence: Precedence) -> () {
        self.register_infix(tt, Box::new(BinaryOperatorParselet::new(precedence, true)));
    }

    pub fn parse_expression(&mut self) -> Box<dyn Expression> {
        self.parser.parse_expression()
    }
}
