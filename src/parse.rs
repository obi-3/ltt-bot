use crate::lexer;
use lexer::*;
use std::fmt;
use Operator::*;
use Token::*;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone)]
pub struct Tree {
    pub token: Token,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

pub struct Parser {
    pub lexer: Lexer,
    // 現在解析中のトークン
    curr_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let curr_token = lexer.get_token();
        Parser { lexer, curr_token }
    }

    pub fn parse(&mut self) -> Result<Option<Box<Tree>>, ParseError> {
        let root: Option<Box<Tree>> = self.eval_expr()?;

        if self.curr_token == Token::End {
            Ok(root)
        } else {
            Err(ParseError {
                message: "ParseError: Failed to create a parse tree.".to_string(),
            })
        }
    }

    fn eval_expr(&mut self) -> Result<Option<Box<Tree>>, ParseError> {
        let mut root: Option<Box<Tree>> = self.eval_term()?;

        match self.curr_token {
            Op(Or) | Op(Nor) | Op(Xor) => {
                let token: Token = self.curr_token.clone();
                self.curr_token = self.lexer.get_token();
                root = Some(Box::new(Tree {
                    token,
                    left: root,
                    right: self.eval_expr()?,
                }));
            }
            _ => {}
        }

        Ok(root)
    }

    fn eval_term(&mut self) -> Result<Option<Box<Tree>>, ParseError> {
        let mut root: Option<Box<Tree>> = self.eval_factor()?;

        match self.curr_token {
            Op(And) | Op(Nand) | Op(Is) => {
                let token: Token = self.curr_token.clone();
                self.curr_token = self.lexer.get_token();
                root = Some(Box::new(Tree {
                    token,
                    left: root,
                    right: self.eval_term()?,
                }));
            }
            _ => {}
        }

        Ok(root)
    }

    fn eval_factor(&mut self) -> Result<Option<Box<Tree>>, ParseError> {
        let root: Option<Box<Tree>>;

        match self.curr_token {
            Op(Not) => {
                let token: Token = self.curr_token.clone();
                self.curr_token = self.lexer.get_token();
                root = Some(Box::new(Tree {
                    token,
                    left: self.eval_primary()?,
                    right: None,
                }));
            }
            _ => root = self.eval_primary()?,
        }

        Ok(root)
    }

    fn eval_primary(&mut self) -> Result<Option<Box<Tree>>, ParseError> {
        let root: Option<Box<Tree>>;

        match self.curr_token.clone() {
            True | False => {
                let token = self.curr_token.clone();
                self.curr_token = self.lexer.get_token();
                root = Some(Box::new(Tree {
                    token,
                    left: None,
                    right: None,
                }));
            }
            Var(..) => {
                let token: Token = self.curr_token.clone();
                self.curr_token = self.lexer.get_token();
                root = Some(Box::new(Tree {
                    token,
                    left: None,
                    right: None,
                }));
            }
            Lpar => {
                self.curr_token = self.lexer.get_token();
                root = self.eval_expr()?;
                if self.curr_token != Token::Rpar {
                    return Err(ParseError {
                        message: "ParseError: Incorrect parenthese".to_string(),
                    });
                }
                self.curr_token = self.lexer.get_token();
            }
            _ => {
                return Err(ParseError {
                    message: "ParseError: Unknown token".to_string(),
                })
            }
        }

        Ok(root)
    }
}
