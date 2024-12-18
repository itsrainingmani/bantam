use std::{cell::RefCell, rc::Rc};

use bantam::core::{BantamParser, Lexer};

struct BantamHarness {
    parser: Rc<RefCell<BantamParser>>,
}

impl BantamHarness {
    pub fn new(input: &str) -> Self {
        let lexer = Lexer::new(input.to_owned());
        let parser = BantamParser::new(Box::new(lexer));

        Self {
            parser: Rc::new(RefCell::new(parser)),
        }
    }

    fn run_test(&self, expected: &str) {
        let result = self.parser.borrow_mut().parse_expression();
        let mut actual = String::new();
        result.print(&mut actual);
        assert_eq!(actual, expected);
    }
}
#[cfg(test)]
mod tests {
    use crate::BantamHarness;

    #[test]
    fn test_basic_calls() {
        let cases = vec![
            ("a()", "a()"),
            ("a(b)", "a(b)"),
            ("a(b, c)", "a(b, c)"),
            ("a(b)(c)", "a(b)(c)"),
            ("a(b) + c(d)", "(a(b) + c(d))"),
            ("a(b ? c : d, e + f)", "a((b ? c : d), (e + f))"),
        ];

        for (input, expected) in cases {
            let test_harness = BantamHarness::new(input);
            test_harness.run_test(expected);
        }
    }

    #[test]
    fn test_unary_precedence() {
        let cases = vec![("~!-+a", "(~(!(-(+a))))"), ("a!!!", "(((a!)!)!)")];

        for (input, expected) in cases {
            let test_harness = BantamHarness::new(input);
            test_harness.run_test(expected);
        }
    }

    #[test]
    fn test_unary_binary_precedence() {
        let cases = vec![
            ("-a * b", "((-a) * b)"),
            ("!a + b", "((!a) + b)"),
            ("~a ^ b", "((~a) ^ b)"),
            ("-a!", "(-(a!))"),
            ("!a!", "(!(a!))"),
        ];

        for (input, expected) in cases {
            let test_harness = BantamHarness::new(input);
            test_harness.run_test(expected);
        }
    }

    #[test]
    fn test_binary_precedence() {
        let test_harness = BantamHarness::new("a = b + c * d ^ e - f / g");
        test_harness.run_test("(a = ((b + (c * (d ^ e))) - (f / g)))");
    }

    #[test]
    fn test_binary_associativity() {
        let cases = vec![
            ("a = b = c", "(a = (b = c))"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a ^ b ^ c", "(a ^ (b ^ c))"),
        ];

        for (input, expected) in cases {
            let test_harness = BantamHarness::new(input);
            test_harness.run_test(expected);
        }
    }

    #[test]
    fn test_conditional_operator() {
        let cases = vec![
            ("a ? b : c ? d : e", "(a ? b : (c ? d : e))"),
            ("a ? b ? c : d : e", "(a ? (b ? c : d) : e)"),
            ("a + b ? c * d : e / f", "((a + b) ? (c * d) : (e / f))"),
        ];

        for (input, expected) in cases {
            let test_harness = BantamHarness::new(input);
            test_harness.run_test(expected);
        }
    }

    #[test]
    fn test_grouping() {
        let cases = vec![
            ("a + (b + c) + d", "((a + (b + c)) + d)"),
            ("a ^ (b + c)", "(a ^ (b + c))"),
            ("(!a)!", "((!a)!)"),
        ];

        for (input, expected) in cases {
            let test_harness = BantamHarness::new(input);
            test_harness.run_test(expected);
        }
    }

    #[test]
    fn test_name() {
        let cases = vec![("abba", "abba")];

        for (input, expected) in cases {
            let test_harness = BantamHarness::new(input);
            test_harness.run_test(expected);
        }
    }
}
