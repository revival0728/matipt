use crate::ast::lexer::Lexer;
use crate::ast::{
    AstParseError, Expr, FunStmt, KeySet, LexResult, LexToken, ParSet, ParseState, Result, Stmt,
};

pub struct AstParser {
    lexer: Lexer,
    op_prec: [u8; 256], // [opid!(OP) -> prec]
    op_rank: [u8; 256], // [prec -> opid!(OP)]
}

impl AstParser {
    pub fn new() -> AstParser {
        let mut lexer = Lexer::new();
        let mut op_prec: [u8; 256] = [0; 256];
        let mut op_rank: [u8; 256] = [0; 256];

        macro_rules! opid {
            ($str:literal) => {
                lexer.get_opid($str).unwrap() as usize
            };
        }
        op_prec[opid!("=")] = 1;
        op_prec[opid!("+")] = 2;
        op_prec[opid!("-")] = 2;
        op_prec[opid!("*")] = 3;
        op_prec[opid!("/")] = 3;
        op_prec[opid!("**")] = 4;

        let mut rank = 0_usize;
        for (op, _) in op_prec.iter().enumerate() {
            op_rank[rank] = op as u8;
            rank += 1;
        }

        AstParser {
            lexer,
            op_prec,
            op_rank,
        }
    }

    #[cfg(test)]
    pub fn get_op_prec(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        for i in self.op_prec {
            vec.push(i);
        }
        vec
    }

    fn parse_expr(&mut self, lex_result: &Vec<LexResult>, lb: usize, rb: usize) -> Box<Expr> {
        // [lb, rb)
        todo!()
    }

    pub fn parse(&mut self, code: &str) -> Result {
        let mut pa_result: Vec<Stmt> = Vec::new();
        let lex_result = self.lexer.lex(code);

        // 0 -> Null
        // 1 -> Op(u8),   // Operator Hash
        // 2 -> Par(u8),  // Paren or Bracket ID
        // 3 -> Key(u8),  // Keyword ID
        // 4 -> Var(Var), // Variable ID
        // 5 -> Raw(Raw), // Raw value
        // 6 -> Endl

        // parse Stmt
        let mut prev_tk = LexToken::Null;
        let mut state = ParseState::Null;
        let mut expr_lb = 0_usize;

        for (idx, lexr) in lex_result.iter().enumerate() {
            macro_rules! return_err {
                ($msg:literal) => {
                    return Err(AstParseError::new($msg, lexr.info))
                };
            }
            macro_rules! push_expr_to_last_stmt {
                () => {{
                    let expr = self.parse_expr(&lex_result, expr_lb, idx);
                    match pa_result.last_mut().unwrap() {
                        Stmt::Raw(expr_list) => {
                            expr_list.push(expr);
                        }
                        Stmt::Fun(func) => {
                            func.expr.push(expr);
                        }
                    }
                }};
            }
            match prev_tk {
                LexToken::Null => match lexr.token {
                    LexToken::Par(_) | LexToken::Var(_) | LexToken::Raw(_) => {
                        state = ParseState::Expr;
                        expr_lb = idx;
                        pa_result.push(Stmt::Raw(Vec::<Box<Expr>>::new()));
                    }
                    LexToken::Key(KeySet::Func) => {
                        state = ParseState::FunDecl;
                        pa_result.push(Stmt::Fun(FunStmt::new()));
                    }
                    _ => return_err!("Expected variable, literals or keyword"),
                },
                LexToken::Op(_) => match lexr.token {
                    LexToken::Par(ParSet::LPar) | LexToken::Var(_) | LexToken::Raw(_) => {}
                    _ => return_err!("Expected left paren, variable or literal"),
                },
                LexToken::Par(ParSet::LPar) => match lexr.token {
                    LexToken::Par(ParSet::RPar) | LexToken::Var(_) | LexToken::Raw(_) => {}
                    _ => return_err!("Expected right paren, variable or literal"),
                },
                LexToken::Par(ParSet::RPar) => match lexr.token {
                    LexToken::Op(_) | LexToken::Var(_) | LexToken::Raw(_) => {}
                    LexToken::Endl => {
                        push_expr_to_last_stmt!();
                        state = ParseState::Null;
                    }
                    _ => return_err!("Expected operator, variable or literal"),
                },
                LexToken::Par(ParSet::LBkt) => match lexr.token {
                    LexToken::Endl | LexToken::Par(ParSet::RBkt) => {}
                    _ => return_err!("Expected new line or right bracket"),
                },
                LexToken::Par(ParSet::RBkt) => panic!(
                    "prev_tk should not be LexToken::Par(ParSet::RBkt), should be LexToken::Null"
                ),
                LexToken::Var(_) => match state {
                    ParseState::Null => panic!("should be in ParseState::Expr"),
                    ParseState::Expr => match lexr.token {
                        LexToken::Op(_) | LexToken::Par(_) => {}
                        LexToken::Endl => {
                            push_expr_to_last_stmt!();
                            state = ParseState::Null;
                        }
                        _ => return_err!("Expected operator, left or right paren"),
                    },
                    ParseState::FunDecl => match lexr.token {
                        LexToken::Par(ParSet::LPar) => {}
                        _ => return_err!(
                            "Error when declaring function. Correct way: func NAME(P1, P2, P3, ...) {}"
                        ),
                    },
                },
                LexToken::Raw(_) => match lexr.token {
                    LexToken::Op(_) | LexToken::Par(ParSet::RPar) => {}
                    LexToken::Endl => {
                        push_expr_to_last_stmt!();
                        state = ParseState::Null;
                    }
                    _ => return_err!("Expected operator or right paren"),
                },
                LexToken::Endl => match lexr.token {
                    LexToken::Key(KeySet::Func) => {
                        state = ParseState::FunDecl;
                        pa_result.push(Stmt::Fun(FunStmt::new()));
                    }
                    LexToken::Par(ParSet::RBkt) => {
                        prev_tk = LexToken::Null;
                        continue; // prevent from setting prev_tk naturally
                    }
                    _ => {}
                },
                LexToken::Key(KeySet::Func) => match lexr.token {
                    LexToken::Var(vid) => {
                        assert_eq!(
                            state,
                            ParseState::FunDecl,
                            "should be in ParseState::FuncDecl"
                        );
                        match pa_result.last_mut().unwrap() {
                            Stmt::Fun(func) => {
                                func.id = vid;
                            }
                            _ => {
                                panic!("when state is FunDecl, last of pa_result should me FunStmt")
                            }
                        }
                    }
                    _ => return_err!("Expected function name"),
                },
            }
            prev_tk = lexr.token;
        }
        Ok(pa_result)
    }
}
