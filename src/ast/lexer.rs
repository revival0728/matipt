use crate::ast::{KeySet, LexResult, LexToken, OpSet, ParSet, Raw, Var};
use crate::hash::SSMap;

pub struct Lexer {
    var_map: SSMap<u32>,
    vid_ceil: Var,
}

impl Lexer {
    pub fn new() -> Lexer {
        let var_map = SSMap::<u32>::new();
        let vid_ceil: Var = 0;

        Lexer { var_map, vid_ceil }
    }

    pub fn get_opid(&mut self, s: &str) -> Option<OpSet> {
        Some(match s {
            "+" => OpSet::Add,
            "-" => OpSet::Sub,
            "*" => OpSet::Mul,
            "/" => OpSet::Div,
            "**" => OpSet::Pow,
            _ => return None,
        })
    }

    pub fn get_kid(&mut self, s: &str) -> Option<KeySet> {
        Some(match s {
            "func" => KeySet::Func,
            _ => return None,
        })
    }

    pub fn get_vid(&mut self, s: &str) -> Var {
        if self.var_map.contains_key_with_str(&s) {
            return self.var_map[s];
        }
        self.var_map.insert_with_str(s, self.vid_ceil);
        self.vid_ceil += 1;
        self.vid_ceil - 1
    }

    pub fn get_pid(&self, s: &str) -> ParSet {
        match s {
            "(" => ParSet::LPar,
            ")" => ParSet::RPar,
            "{" => ParSet::LBkt,
            "}" => ParSet::RBkt,
            _ => panic!("get_pid(): the argument is not a paren or a bracket."),
        }
    }

    pub fn lex(&mut self, code: &str) -> Vec<LexResult> {
        let mut lex_result = Vec::<LexResult>::new();

        macro_rules! is_op {
            ($ch:ident) => {{ self.get_opid($ch).is_some() }};
        }

        macro_rules! is_keyw {
            ($ch:ident) => {{ self.get_kid($ch).is_some() }};
        }

        macro_rules! is_par {
            ($ch:ident) => {{
                let mut r = false;
                if $ch.len() == 1 {
                    if $ch == "(" || $ch == ")" || $ch == "{" || $ch == "}" {
                        r = true;
                    }
                }
                r
            }};
        }

        macro_rules! is_num {
            ($ch:ident) => {
                $ch.parse::<Raw>().is_ok()
            };
        }

        for (ln, line) in code.trim().split('\n').enumerate() {
            for (wn, s) in line.trim().split_whitespace().enumerate() {
                macro_rules! push_result {
                    ($lex_token:expr) => {
                        lex_result.push(LexResult::new($lex_token, ln + 1, wn + 1));
                    };
                }
                if is_op!(s) {
                    push_result!(LexToken::Op(self.get_opid(s).unwrap()));
                    continue;
                }
                if is_keyw!(s) {
                    push_result!(LexToken::Key(self.get_kid(s).unwrap()));
                    continue;
                }
                if is_par!(s) {
                    push_result!(LexToken::Par(self.get_pid(s)));
                    continue;
                }
                if is_num!(s) {
                    push_result!(LexToken::Raw(s.parse::<Raw>().unwrap()));
                    continue;
                }
                push_result!(LexToken::Var(self.get_vid(s)));
            }
            lex_result.push(LexResult::new(LexToken::Endl, ln + 1, 0));
        }

        lex_result
    }
}
