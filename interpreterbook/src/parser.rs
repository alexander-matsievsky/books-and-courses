use crate::token::Token::*;
use crate::{ast, lexer, token};

pub struct Parser {
    curr_token: Option<token::Token>,
    lexer: lexer::Lexer,
    peek_token: Option<token::Token>,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Parser {
        let mut parser = Parser {
            curr_token: None,
            lexer,
            peek_token: None,
        };
        parser.next();
        parser.next();
        parser
    }

    pub fn parse(&mut self) -> ast::Program {
        let mut statements = vec![];
        while self.curr_token.is_some() {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement)
            }
        }
        ast::Program { statements }
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.curr_token {
            Some(token::Token::Let) => self.parse_let_statement(),
            Some(token::Token::Return) => self.parse_return_statement(),
            _ => {
                self.next();
                None
            }
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let token = match (self.curr_token.clone(), self.peek_token.clone()) {
            (Some(Let), Some(token @ Ident(_))) => {
                self.next();
                self.next();
                token
            }
            (Some(Let), _) => panic!("expected next token to be {}", Ident("?".to_string())),
            _ => return None,
        };
        let int = match (self.curr_token.clone(), self.peek_token.clone()) {
            (Some(Assign), Some(int @ Int(_))) => {
                self.next();
                self.next();
                int
            }
            (Some(Assign), _) => {
                panic!("expected next token to be {}", Int("?".to_string()));
            }
            _ => {
                panic!("expected next token to be {}", Assign);
            }
        };
        if let Some(Semicolon) = self.curr_token {
            self.next();
        }
        Some(ast::Statement::Let(token, ast::Expression::Int(int)))
    }

    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        let int = match (self.curr_token.clone(), self.peek_token.clone()) {
            (Some(Return), Some(int @ Int(_))) => {
                self.next();
                self.next();
                int
            }
            (Some(Return), _) => {
                panic!("expected next token to be {}", Int("?".to_string()));
            }
            _ => return None,
        };
        if let Some(Semicolon) = self.curr_token {
            self.next();
        }
        Some(ast::Statement::Return(ast::Expression::Int(int)))
    }
}

impl Iterator for Parser {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next();
        if self.curr_token.is_none() && self.peek_token.is_none() {
            None
        } else {
            Some(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token::*;
    use crate::{ast, lexer, parser};

    #[test]
    fn parser_1() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";
        let expected = ast::Program {
            statements: vec![
                ast::Statement::Let(
                    Ident("x".to_string()),
                    ast::Expression::Int(Int("5".to_string())),
                ),
                ast::Statement::Let(
                    Ident("y".to_string()),
                    ast::Expression::Int(Int("10".to_string())),
                ),
                ast::Statement::Let(
                    Ident("foobar".to_string()),
                    ast::Expression::Int(Int("838383".to_string())),
                ),
            ],
        };
        let lexer = lexer::Lexer::new(String::from(input));
        let mut parser = parser::Parser::new(lexer);
        let actual = parser.parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parser_2() {
        let input = "
            return 5;
            return 10;
            return 993322;
        ";
        let expected = ast::Program {
            statements: vec![
                ast::Statement::Return(ast::Expression::Int(Int("5".to_string()))),
                ast::Statement::Return(ast::Expression::Int(Int("10".to_string()))),
                ast::Statement::Return(ast::Expression::Int(Int("993322".to_string()))),
            ],
        };
        let lexer = lexer::Lexer::new(String::from(input));
        let mut parser = parser::Parser::new(lexer);
        let actual = parser.parse();
        assert_eq!(expected, actual);
    }
}
