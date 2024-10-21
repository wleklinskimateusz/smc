use std::char;

pub trait TokenCollector {
    fn open_brace(&mut self, line: u32, position: u32);
    fn close_brace(&mut self, line: u32, position: u32);
    fn open_parenthesis(&mut self, line: u32, position: u32);
    fn close_parenthesis(&mut self, line: u32, position: u32);
    fn open_angle_bracket(&mut self, line: u32, position: u32);
    fn close_angle_bracket(&mut self, line: u32, position: u32);
    fn dash(&mut self, line: u32, position: u32);
    fn colon(&mut self, line: u32, position: u32);
    fn name(&mut self, name: &str, line: u32, position: u32);
    fn error(&mut self, message: &str, line: u32, position: u32);
}

pub struct LexicalAnalyzer {
    collector: Box<dyn TokenCollector>,
    line_number: u32,
    position: u32,
}

impl LexicalAnalyzer {
    pub fn new(collector: Box<dyn TokenCollector>) -> Self {
        LexicalAnalyzer {
            collector,
            line_number: 0,
            position: 0,
        }
    }

    pub fn lex(&mut self, s: &str) {
        self.line_number = 1;
        let lines = s.split("\n");
        for line in lines {
            self.lex_line(line);
            self.line_number += 1;
            self.position = 0;
        }
    }

    fn lex_line(&mut self, line: &str) {
        while self.position < line.len().try_into().unwrap() {
            if !self.find_token(line) {
                self.collector
                    .error("Unknown token", self.line_number, self.position);
                self.position += 1;
            }
        }
    }

    fn lex_token(&mut self, line: &str) {}

    fn find_token(&mut self, line: &str) -> bool {
        self.find_whitespace(line) || self.find_single_char_token(line) || self.find_name(line)
    }

    fn find_whitespace(&mut self, line: &str) -> bool {
        let mut found = false;
        let chars = line.chars().skip(self.position.try_into().unwrap());
        for c in chars {
            if char::is_whitespace(c) {
                self.position += 1;
                found = true;
            } else {
                break;
            }
        }

        found
    }

    fn find_single_char_token(&mut self, line: &str) -> bool {
        let c = line.chars().nth(self.position.try_into().unwrap()).unwrap();
        match c {
            '{' => {
                self.collector.open_brace(self.line_number, self.position);
            }
            '}' => {
                self.collector.close_brace(self.line_number, self.position);
            }
            '(' => {
                self.collector
                    .open_parenthesis(self.line_number, self.position);
            }
            ')' => {
                self.collector
                    .close_parenthesis(self.line_number, self.position);
            }
            '<' => {
                self.collector
                    .open_angle_bracket(self.line_number, self.position);
            }
            '>' => {
                self.collector
                    .close_angle_bracket(self.line_number, self.position);
            }
            '-' => {
                self.collector.dash(self.line_number, self.position);
            }
            ':' => {
                self.collector.colon(self.line_number, self.position);
            }
            _ => return false,
        };
        self.position += 1;
        true
    }

    fn find_name(&mut self, line: &str) -> bool {
        let mut found = false;
        let mut name = String::new();
        let chars = line.chars().skip(self.position.try_into().unwrap());
        for c in chars {
            if char::is_alphanumeric(c) || c == '_' {
                name.push(c);
                self.position += 1;
                found = true;
            } else {
                break;
            }
        }

        if found {
            self.collector.name(&name, self.line_number, self.position);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn test_empty_line() {
        assert_lines("", "");
    }

    #[test]
    fn test_whitespace_line() {
        assert_lines("  ", "");
    }

    #[test]
    fn test_single_char_tokens() {
        assert_lines("{", "{");
        assert_lines("}", "}");
        assert_lines("(", "(");
        assert_lines(")", ")");
        assert_lines("<", "<");
        assert_lines(">", ">");
        assert_lines("-", "-");
        assert_lines(":", ":");
    }

    #[test]
    fn test_name() {
        assert_lines("name", "name");
        assert_lines("name1", "name1");
        assert_lines("name_1", "name_1");
        assert_lines("name 1", "name,1");
    }

    #[test]
    fn test_assert_combination() {
        assert_lines("{ name }", "{,name,}");
        assert_lines("{     name1 }", "{,name1,}");
        assert_lines("{ name_1   }", "{,name_1,}");
        assert_lines("{ name 1       }", "{,name,1,}");
    }

    #[test]
    fn test_multiline() {
        assert_lines("{ name }\n{ name1 }", "{,name,},{,name1,}");
    }

    #[test]
    fn test_error() {
        assert_lines("{ name }\n{ name1 } %", "{,name,},{,name1,},ERROR");
        assert_lines(
            "{ name }\n{ name1 } %$ ^ ",
            "{,name,},{,name1,},ERROR,ERROR,ERROR",
        );
    }

    fn assert_lines(lines: &str, expected: &str) {
        let tokens = Rc::new(RefCell::new(String::new()));

        // Mock struct defined inside the test
        struct MockCollector {
            tokens_ref: Rc<RefCell<String>>,
        }

        // Implement the TokenCollector trait for MockCollector
        impl<'a> TokenCollector for MockCollector {
            fn open_brace(&mut self, line: u32, position: u32) {
                self.tokens_ref.borrow_mut().push_str("{");
                self.tokens_ref.borrow_mut().push(',');
            }

            fn close_brace(&mut self, line: u32, position: u32) {
                self.tokens_ref.borrow_mut().push('}');
                self.tokens_ref.borrow_mut().push(',');
            }

            fn open_parenthesis(&mut self, line: u32, position: u32) {
                self.tokens_ref.borrow_mut().push('(');
                self.tokens_ref.borrow_mut().push(',');
            }

            fn close_parenthesis(&mut self, line: u32, position: u32) {
                self.tokens_ref.borrow_mut().push(')');
                self.tokens_ref.borrow_mut().push(',');
            }

            fn open_angle_bracket(&mut self, line: u32, position: u32) {
                self.tokens_ref.borrow_mut().push('<');
                self.tokens_ref.borrow_mut().push(',');
            }

            fn close_angle_bracket(&mut self, line: u32, position: u32) {
                self.tokens_ref.borrow_mut().push('>');
                self.tokens_ref.borrow_mut().push(',');
            }

            fn dash(&mut self, line: u32, position: u32) {
                self.tokens_ref.borrow_mut().push('-');
                self.tokens_ref.borrow_mut().push(',');
            }

            fn colon(&mut self, line: u32, position: u32) {
                self.tokens_ref.borrow_mut().push(':');
                self.tokens_ref.borrow_mut().push(',');
            }

            fn name(&mut self, name: &str, line: u32, position: u32) {
                self.tokens_ref.borrow_mut().push_str(name);
                self.tokens_ref.borrow_mut().push(',');
            }

            fn error(&mut self, message: &str, line: u32, position: u32) {
                self.tokens_ref.borrow_mut().push_str("ERROR");
                self.tokens_ref.borrow_mut().push(',');
            }
        }
        let mock_collector = MockCollector {
            tokens_ref: Rc::clone(&tokens),
        };

        let mut analyzer = LexicalAnalyzer::new(Box::new(mock_collector));
        analyzer.lex(lines);
        let mut tokens = tokens.borrow_mut();

        tokens.pop();

        assert_eq!(tokens.as_str(), expected);
    }
}
