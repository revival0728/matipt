use crate::ast::lexer::Lexer;
use crate::ast::{Expr, LexResult, Result, Stmt};
use std::collections::LinkedList;

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

    fn parse_expr(&mut self, tokens: LinkedList<LexResult>) -> Box<Expr> {
        todo!()
    }

    pub fn parse(&mut self, code: &str) -> Result {
        let pa_result: Vec<Stmt> = Vec::new();
        let lex_result = self.lexer.lex(code);

        // 1 -> Op(u8),   // Operator Hash
        // 2 -> Par(u8),  // Paren or Bracket ID
        // 3 -> Key(u8),  // Keyword ID
        // 4 -> Var(Var), // Variable ID
        // 5 -> Raw(Raw), // Raw value
        let prev_tk = 0_u32;
        for token in lex_result {
            // match token {
            //     Op =>
            // }
        }

        Ok(pa_result)
    }
}
