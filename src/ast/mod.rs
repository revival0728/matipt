mod lexer;
pub mod parser;

/*
Ast Structure

Ast: Vec<Stmt> |
        Stmt_0
        Stmt_1
        Stmt_2
        ...

Stmt: FunStmt | Vec<Box<Expr>>

FunStmt: Vec<Box<Expr>>  // Use when script declares a function

Box<Expr>: BinOp | FunExpr | Idnt

BinOp |
    lhs: Box<Expr>
    rhs: Box<Expr>

FunExpr: Vec<Box<Expr>>  // Use When script makes a function call

Idnt: Identifier

*/

pub enum Expr {
    Add(BinOp),
    Sub(BinOp),
    Mul(BinOp),
    Div(BinOp),
    Pow(BinOp),
    Set(BinOp),
    Fun(FunExpr),
    Idnt(Idnt),
}

pub struct BinOp {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

pub enum Idnt {
    Var(Var),
    Raw(Raw),
}

pub type Var = u32;

pub type Raw = f32;

pub struct FunExpr {
    pub id: Var, // Var ID
    pub arg: Vec<Box<Expr>>,
}

pub enum Stmt {
    Fun(FunStmt),
    Raw(Vec<Box<Expr>>),
}

pub struct FunStmt {
    pub id: u32, // Var ID
    pub param: Vec<Var>,
    pub expr: Vec<Box<Expr>>,
}

impl FunStmt {
    pub fn new() -> FunStmt {
        FunStmt {
            id: 0,
            param: Vec::new(),
            expr: Vec::new(),
        }
    }
}

#[derive(Clone, Copy)]
enum OpSet {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Clone, Copy)]
enum ParSet {
    LPar,
    RPar,
    LBkt,
    RBkt,
}

#[derive(Clone, Copy)]
enum KeySet {
    Func,
}

#[derive(Clone, Copy)]
enum LexToken {
    Null,
    Op(OpSet),   // Operator ID
    Par(ParSet), // Paren or Bracket ID
    Key(KeySet), // Keyword ID
    Var(Var),    // Variable ID
    Raw(Raw),    // Raw value
    Endl,        // End of line
}

#[derive(Clone, Copy)]
struct LexResult {
    pub token: LexToken,
    pub info: (usize, usize), // (line number, word number)
}

impl LexResult {
    pub fn new(token: LexToken, ln: usize, wn: usize) -> LexResult {
        LexResult {
            token,
            info: (ln, wn),
        }
    }
}

#[derive(PartialEq, Debug)]
enum ParseState {
    Null,
    Expr,
    FunDecl,
}

#[derive(Debug)]
pub struct AstParseError {
    msg: &'static str,
    info: (usize, usize), // (line number, word number)
}

impl AstParseError {
    pub fn new(msg: &'static str, info: (usize, usize)) -> AstParseError {
        AstParseError { msg, info }
    }
}

impl std::fmt::Display for AstParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AstParseError({}:{}): {}",
            self.info.0, self.info.1, self.msg
        )
    }
}

impl std::error::Error for AstParseError {}

pub type Result = std::result::Result<Vec<Stmt>, AstParseError>;
