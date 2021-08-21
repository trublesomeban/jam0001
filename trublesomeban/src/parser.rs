use std::iter::Peekable;

use nexer::lexer::{self, Token};

use self::ast::ExprNode;

mod ast;

pub enum ErrorType {
    ParseError(Error),
    LexError(lexer::Error),
}

pub enum OpType {
    Unary,
    Binary,
}

pub struct Error {}

pub struct Parser<'a> {
    ast: Vec<ast::ExprNode<'a>>,
    ops: &'a [(&'a str, u8, OpType)],
    iter: Peekable<lexer::TokenStream<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: lexer::TokenStream<'a>, ops: &'a [(&'a str, u8, OpType)]) -> Self {
        Self {
            iter: tokens.peekable(),
            ops,
            ast: vec![],
        }
    }

    pub fn run(&mut self) -> Result<&'a Vec<ast::ExprNode>, ErrorType> {
        while let Some(token) = self.iter.next() {
            self.parse(match token {
                Err(err) => return Err(ErrorType::LexError(err)),
                Ok(tok) => tok,
            });
        }
        Ok(&self.ast)
    }

    fn parse(&mut self, token: Token) -> Result<ast::ExprNode, ErrorType> {
        match token {
            // lexer::Token::Sym(sym) => {
            //     if sym == ":=" {
            //         self.bind()
            //     }
            // }
            lexer::Token::Lit(val) => match self.val(val) {
                Err(e) => Err(ErrorType::ParseError(Error {})),
                Ok(val) => Ok(ExprNode::Val(val)),
            },
            _ => Err(ErrorType::ParseError(Error {})),
        }
    }

    fn val(&mut self, literal: lexer::Literal) -> Result<ast::ValueNode, ErrorType> {
        if let lexer::Literal::Str(s) = literal {
            Ok(ast::ValueNode::Str(s.to_string()))
        } else if let lexer::Literal::Int(i) = literal {
            Ok(ast::ValueNode::Int(i.parse().unwrap()))
        } else if let lexer::Literal::Float(f) = literal {
            Ok(ast::ValueNode::Float(f.parse().unwrap()))
        } else {
            Err(ErrorType::ParseError(Error {}))
        }
    }

    // fn atom(&mut self, ident: lexer::Identifier) -> Result<ast::ValueNode, ErrorType> {}

    fn bind(&'a mut self) -> Result<ast::ExprNode, ErrorType> {
        let val = match match self.iter.next() {
            None => return Err(ErrorType::ParseError(Error {})),
            Some(expr) => expr,
        } {
            Err(err) => return Err(ErrorType::LexError(err)),
            Ok(val) => self.parse(val)?,
        };
        let ident = match match self.ast.last() {
            None => return Err(ErrorType::ParseError(Error {})),
            Some(expr) => expr,
        } {
            ast::ExprNode::Atom(atom) => atom,
            _ => return Err(ErrorType::ParseError(Error {})),
        };
        Ok(ast::ExprNode::Bind(ast::BindingNode { ident: *ident, val: Box::new(val) }))
    }
}
