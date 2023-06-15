use std::{error::Error, thread::current};

use crate::{
    ast::{Expr, Stmt, Range},
    reporter::ErrorReporter,
    token::{self, Kind, Token},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct Parser<'r> {
    reporter: &'r ErrorReporter,
    tokens: Vec<Token>,
    current: usize,
}

/// public
impl<'r> Parser<'r> {
    pub fn new(tokens: Vec<Token>, reporter: &'r ErrorReporter) -> Self {
        Self {
            reporter,
            tokens,
            current: 0,
        }
    }

    pub fn stmts(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = vec![];
        loop {
            if self.match_any(vec![Kind::NewLine]) {
                continue;
            }
            if let Some(tk) = self.peek() {
		match tk.kind() {
		    Kind::Eof | Kind::RightBrace => {
			break;
		    },
		    _ => {
			let st = self.stmt()?;
			stmts.push(st);
		    }
		}
            }
        }
        Ok(stmts)
    }
    fn stmt(&mut self) -> Result<Stmt> {
        if self.is_match_all(vec![Kind::Name, Kind::Equal]) {
            self.assignment()
        // } else if self.is_match_all(vec![Kind::Name , Kind::LeftParen])  {
        //     self.print_stmt()
        }else if self.match_any(vec![Kind::If]) {
	    self.if_stmt()
	} else {
            let e = self.exp()?;
            let st = Stmt::expr().expr(e).build();
            Ok(st)
        }
    }

    // fn if_stmt(&mut self) -> Result<Stmt>{
    // 	let cond = self.exp()?;
	
    // 	self.expect(Kind::LeftBrace,"Expected `{`")?;
    // 	self.expect(Kind::NewLine,"Expected `\n`")?;
    // 	let then = self.stmts()?;
    // 	self.expect(Kind::RightBrace,"Expected `}`")?;
    // 	let mut  builder = Stmt::if_stmt().condition(cond).then(then);
    // 	if self.match_any(vec![Kind::Else]) {
    // 	    self.expect(Kind::LeftBrace,"Expected `{`")?;
    // 	    self.expect(Kind::NewLine,"Expected `\n`")?;
    // 	    let else_block= self.stmts()?;
    // 	    self.expect(Kind::RightBrace,"Expected `}`")?;
    // 	    builder = builder.else_(else_block);
    // 	}
    // 	let ifstmt = builder.build();
    // 	Ok(ifstmt)
    // }

    fn if_stmt(&mut self) -> Result<Stmt> {
	let cond = self.exp()?;
	self.expect(Kind::LeftBrace,"Expected `{`")?;	
	let then = self.block()?;
	
	self.expect(Kind::Else,"Expected `else` branch")?;
	
	self.expect(Kind::LeftBrace,"Expected `{`")?;		
	let els = self.block()?;
	
	Ok(Stmt::if_stmt().condition(cond).then(then).else_(els).build())
    }
      

    fn assignment(&mut self) -> Result<Stmt> {
        let name = self
            .match_all(vec![Kind::Name, Kind::Equal])
            .unwrap()
            .remove(0);
        let binding = self.exp()?;
        let st = Stmt::assignment().name(name).binding(binding).build();
        Ok(st)
    }

    fn print_stmt(&mut self) -> Result<Stmt> {
        todo!()
    }

    /// expression:
    pub fn exp(&mut self) -> Result<Expr> {
	if self.match_any(vec![Kind::LeftBrace]) {
	    self.block()
	}else {
            self.condition()
	}

    }

    pub fn block(&mut self) -> Result<Expr> {
	let open = self.previous().unwrap();
	while self.match_any(vec![Kind::NewLine]) {
	}
	let mut sts = self.stmts()?;
	while self.match_any(vec![Kind::NewLine]) {
	}
	let close = self.expect(Kind::RightBrace,"Expected `}`")?;	
	let res = sts.pop();
	let body = sts;
	
	let result: Option<Box<Expr>> = res.and_then(|s| {
	    match s.stmt {
		crate::ast::StmtData::Expr(e) => {
		    Some(e.into())
		},
		_ => {
		    None
		}
	    }
	});
	let range = (open.range().0,close.range().1);
	Ok(Expr{
	    data: crate::ast::ExprData::Block{
		body,
		result,
	    },
	    range,
	})
    }

    // if expr:
    // <expr1> if <condition> else <expr2>
    fn condition(&mut self) -> Result<Expr> {
	let mut e = self.logical()?;
	if self.match_any(vec![Kind::If]) {
	    let cond = self.logical()?;
	    self.expect(Kind::Else,"Expected `else`")?;
	    let els = self.logical()?;
	    e =   Expr::condition().condition(cond).then(e).else_(els).build();
	}

	Ok(e)
    }


    /// logical = equality { (and | or) equality }*
    fn logical(&mut self) -> Result<Expr> {
	let mut e1 = self.equality()?;

        while self.match_any(vec![Kind::And , Kind::Or]) {
            let op = self.previous().unwrap();
            let e2 = self.equality()?;
            e1 = Expr::binary().left(e1).op(op).right(e2).build();
        }
        Ok(e1)
    }

    /// Eq / NotEq
    fn equality(&mut self) -> Result<Expr> {
	let mut e1 = self.term()?;

        while self.match_any(vec![Kind::EqualEqual, Kind::BangEqual,
				  Kind::Greater,Kind::GreaterEqual,Kind::Less,Kind::LessEqual]) {
            let op = self.previous().unwrap();
            let e2 = self.term()?;
            e1 = Expr::binary().left(e1).op(op).right(e2).build();
        }
        Ok(e1)
    }
    
    fn term(&mut self) -> Result<Expr> {
        let mut e1 = self.factor()?;

        while self.match_any(vec![Kind::Plus, Kind::Minus]) {
            let op = self.previous().unwrap();
            let e2 = self.factor()?;
            e1 = Expr::binary().left(e1).op(op).right(e2).build();
        }
        Ok(e1)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut e1 = self.unary()?;
        while self.match_any(vec![Kind::Star, Kind::Slash]) {
            let op = self.previous().unwrap();
            let e2 = self.unary()?;
            e1 = Expr::binary().left(e1).op(op).right(e2).build();
        }
        Ok(e1)
    }

    // unary          → ( "not" | "-" ) unary | call ;
    fn unary(&mut self) -> Result<Expr> {
        if self.match_any(vec![Kind::Minus, Kind::Bang]) {
            let op = self.previous().unwrap();
            let expr = self.function_call()?;
            let unary = Expr::unary().op(op).operand(expr).build();
            return Ok(unary);
        }
        Ok(self.function_call()?)
    }

    // call   → primary ( "(" arguments? ")" )* ;
    fn function_call(&mut self) -> Result<Expr> {
        let mut callee = self.primary()?;

        while self.match_any(vec![Kind::LeftParen]) {
            let args = self.arguments()?;
            self.expect(Kind::RightParen, "Expected `)`")?;
            callee = Expr::call().func(callee).args(args).build();
        }

        Ok(callee)
    }

    fn arguments(&mut self) -> Result<Vec<Expr>> {
        let mut first = true;
        let mut args = vec![];
        loop {
            match self.peek() {
                Some(tk) if tk.kind() == Kind::RightParen => {
                    break;
                }
                None => break,
                _ => {
                    if !first {
                        self.expect(Kind::Comma, "Expected `,`")?;
                    } else {
                        first = false;
                    }
                    args.push(self.exp()?);
                }
            }
        }

        Ok(args)
    }

    fn primary(&mut self) -> Result<Expr> {
        if let Some(tk) = self.peek() {
            match tk.kind() {
                Kind::Integer | Kind::Float | Kind::Name| Kind::True | Kind::False => {
                    self.advance();
                    return Ok(Expr::atom(tk));
                }
                Kind::LeftParen => {
                    self.advance();
                    let r = self.exp()?;
                    self.expect(Kind::RightParen, "Expected `)`.")?;
                    return Ok(r);
                }
                _ => Err(self.reporter.error_token("Unexpected Token.", &tk).unwrap_err()),
            }
	    
        } else {
            self.reporter
                .error_token("Unexpected EOF.", &self.previous().unwrap())?;
            panic!("Unexpected EOF.");
        }
    }
}

/// private:
impl<'r> Parser<'r> {
    fn previous(&self) -> Option<Token> {
        if self.current == 0 || self.current > self.tokens.len() {
            None
        } else {
            self.tokens.get(self.current - 1).cloned()
        }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    fn advance(&mut self) -> Option<Token> {
        let tk = self.peek();
        self.current += 1;
        tk
    }

    fn expect(&mut self, kind: Kind, msg: &str) -> Result<Token> {
        if let Some(token) = self.peek() {
            if token.kind() == kind {
                self.advance();
		return Ok(token)
            } else {
		return Err(self.reporter.error_token(msg, &token).unwrap_err())
            }
        } 
        Err( self.reporter
             .error_token("Unexpected EOF.", &self.previous().unwrap()).unwrap_err())
    }

    #[inline]
    fn is_match_all(&mut self, kinds: Vec<token::Kind>) -> bool {
        let start = self.current;
        kinds.iter().enumerate().all(|(i, k)| {
            if let Some(tk) = self.tokens.get(start + i) {
                tk.kind() == *k
            } else {
                false
            }
        })
    }

    fn match_all(&mut self, kinds: Vec<token::Kind>) -> Option<Vec<Token>> {
        let start = self.current;
        let len = kinds.len();

        if self.is_match_all(kinds) {
            self.current += len;
            Some(self.tokens[start..start + len].to_owned())
        } else {
            None
        }
    }
    fn match_any(&mut self, kinds: Vec<token::Kind>) -> bool {
        let res = kinds.iter().any(|k| self.is_match(*k));

        if res {
            self.current += 1;
        }
        res
    }

    fn is_match(&self, kind: token::Kind) -> bool {
        self.peek().map_or(false, |tk| tk.kind() == kind)
    }
}
