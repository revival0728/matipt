use crate::ast::{LexResult, LexToken, Raw, Var};
use std::{
    collections::{HashMap, LinkedList},
    hash::{DefaultHasher, Hash, Hasher},
};

pub struct Lexer {
    hasher: DefaultHasher,
    var_map: HashMap<u64, Var>,
    vid_ceil: Var,
}

impl Lexer {
    pub fn new() -> Lexer {
        let hasher = DefaultHasher::new();
        let var_map: HashMap<u64, Var> = HashMap::new();
        let vid_ceil: Var = 0;

        Lexer {
            hasher,
            var_map,
            vid_ceil,
        }
    }

    pub fn get_opid(&mut self, s: &str) -> Option<u8> {
        Some(match s {
            "+" => 0,
            "-" => 1,
            "*" => 2,
            "/" => 3,
            "**" => 4,
            _ => return None,
        })
    }

    pub fn get_kid(&mut self, s: &str) -> Option<u8> {
        Some(match s {
            "func" => 0,
            _ => return None,
        })
    }

    pub fn get_vid(&mut self, s: &str) -> Var {
        s.hash(&mut self.hasher);
        let h = self.hasher.finish();
        if self.var_map.contains_key(&h) {
            return self.var_map[&h];
        }
        let vid = self.var_map.insert(h, self.vid_ceil).unwrap();
        self.vid_ceil += 1;
        vid
    }

    pub fn get_pid(&self, s: &str) -> u8 {
        match s {
            "(" => 0,
            ")" => 1,
            "{" => 2,
            "}" => 3,
            _ => panic!("get_pid(): the argument is not a paren or a bracket."),
        }
    }

    pub fn lex(&mut self, code: &str) -> LinkedList<LexResult> {
        let mut lex_result: LinkedList<LexResult> = LinkedList::new();

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
            for (wn, s) in line.trim().split(' ').enumerate() {
                macro_rules! push_result {
                    ($lex_token:expr) => {
                        lex_result.push_back(LexResult::new($lex_token, ln, wn));
                    };
                }
                if is_op!(s) {
                    push_result!(LexToken::Op(self.get_opid(s).unwrap()));
                    continue;
                }
                if is_keyw!(s) {
                    push_result!(LexToken::Key(self.get_opid(s).unwrap()));
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
        }

        lex_result
    }
}
