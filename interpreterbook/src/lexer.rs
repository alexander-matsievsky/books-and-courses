use crate::token;
use crate::token::Token::*;

#[derive(Debug)]
pub struct Lexer {
    ch: char,
    input: Vec<char>,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            ch: '\x00',
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
        };
        lexer.read_char();
        lexer
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\x00'
        } else {
            self.input[self.read_position]
        }
    }

    fn read_char(&mut self) {
        self.ch = self.peek_char();
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_literal(&mut self) -> String {
        let start = self.position;
        while self.ch.is_alphabetic() {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    fn read_numeric(&mut self) -> String {
        let start = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }
}

impl Iterator for Lexer {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.ch.is_whitespace() {
            self.read_char();
        }
        if self.ch.is_alphabetic() {
            let literal = self.read_literal();
            return Some(match literal.as_str() {
                "else" => Else,
                "false" => False,
                "fn" => Function,
                "if" => If,
                "let" => Let,
                "return" => Return,
                "true" => True,
                _ => Ident(literal),
            });
        }
        if self.ch.is_numeric() {
            return Some(Int(self.read_numeric()));
        }
        let token = match self.ch {
            '*' => Asterisk,
            ',' => Comma,
            '>' => Gt,
            '{' => Lbrace,
            '(' => Lparen,
            '<' => Lt,
            '-' => Minus,
            '+' => Plus,
            '}' => Rbrace,
            ')' => Rparen,
            ';' => Semicolon,
            '/' => Slash,
            '\x00' => {
                if self.read_position >= self.input.len() {
                    return None;
                }
                Eof
            }
            '=' => {
                if self.peek_char() != '=' {
                    Assign
                } else {
                    self.read_char();
                    Eq
                }
            }
            '!' => {
                if self.peek_char() != '=' {
                    Bang
                } else {
                    self.read_char();
                    NotEq
                }
            }
            char => Illegal(char),
        };
        self.read_char();
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::token::Token::*;

    #[test]
    fn lexer_1() {
        let input = "=+(){},;";
        let expected = vec![
            Assign, Plus, Lparen, Rparen, Lbrace, Rbrace, Comma, Semicolon,
        ];
        let actual = lexer::Lexer::new(String::from(input));
        for (expected, actual) in expected.into_iter().zip(actual) {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn lexer_2() {
        let input = "
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
        ";
        let expected = vec![
            Let,
            Ident("five".to_string()),
            Assign,
            Int("5".to_string()),
            Semicolon,
            Let,
            Ident("ten".to_string()),
            Assign,
            Int("10".to_string()),
            Semicolon,
            Let,
            Ident("add".to_string()),
            Assign,
            Function,
            Lparen,
            Ident("x".to_string()),
            Comma,
            Ident("y".to_string()),
            Rparen,
            Lbrace,
            Ident("x".to_string()),
            Plus,
            Ident("y".to_string()),
            Semicolon,
            Rbrace,
            Semicolon,
            Let,
            Ident("result".to_string()),
            Assign,
            Ident("add".to_string()),
            Lparen,
            Ident("five".to_string()),
            Comma,
            Ident("ten".to_string()),
            Rparen,
            Semicolon,
            Eof,
        ];
        let actual = lexer::Lexer::new(String::from(input));
        for (expected, actual) in expected.into_iter().zip(actual) {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn lexer_3() {
        let input = "
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
              x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        ";
        let expected = vec![
            Let,
            Ident("five".to_string()),
            Assign,
            Int("5".to_string()),
            Semicolon,
            Let,
            Ident("ten".to_string()),
            Assign,
            Int("10".to_string()),
            Semicolon,
            Let,
            Ident("add".to_string()),
            Assign,
            Function,
            Lparen,
            Ident("x".to_string()),
            Comma,
            Ident("y".to_string()),
            Rparen,
            Lbrace,
            Ident("x".to_string()),
            Plus,
            Ident("y".to_string()),
            Semicolon,
            Rbrace,
            Semicolon,
            Let,
            Ident("result".to_string()),
            Assign,
            Ident("add".to_string()),
            Lparen,
            Ident("five".to_string()),
            Comma,
            Ident("ten".to_string()),
            Rparen,
            Semicolon,
            Bang,
            Minus,
            Slash,
            Asterisk,
            Int("5".to_string()),
            Semicolon,
            Int("5".to_string()),
            Lt,
            Int("10".to_string()),
            Gt,
            Int("5".to_string()),
            Semicolon,
            If,
            Lparen,
            Int("5".to_string()),
            Lt,
            Int("10".to_string()),
            Rparen,
            Lbrace,
            Return,
            True,
            Semicolon,
            Rbrace,
            Else,
            Lbrace,
            Return,
            False,
            Semicolon,
            Rbrace,
            Int("10".to_string()),
            Eq,
            Int("10".to_string()),
            Semicolon,
            Int("10".to_string()),
            NotEq,
            Int("9".to_string()),
            Semicolon,
            Eof,
        ];
        let actual = lexer::Lexer::new(String::from(input));
        for (expected, actual) in expected.into_iter().zip(actual) {
            assert_eq!(expected, actual);
        }
    }
}
