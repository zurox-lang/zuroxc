#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(u128, Option<String>),
    Float(f64, Option<String>),
    String(String),
    Char(char),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    BitNot,
    Ref,
    Deref,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    And,
    Or,
    Xor,
    ShiftLeft,
    ShiftRight,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericParameter {
    Simple(String),
    Extends(String, String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Float32,
    Float64,
    Float80,
    Float128,
    Char,
    Bool,
    Struct(String, Option<Vec<GenericParameter>>),
    Enum(String),
    Array(Box<Type>, Box<Literal>),
    Identifier(String, Option<Vec<GenericParameter>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    UnaryOp(Box<UnaryOp>, Box<Expression>),
    BinaryOp(Box<Expression>, Box<BinaryOp>, Box<Expression>),
    Group(Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    If(
        Box<Expression>,
        Box<Block>,
        Vec<(Box<Expression>, Box<Block>)>,
        Option<Box<Block>>,
    ),
    Loop(Box<Block>),
    VariableDeclaration(Option<String>, Box<Type>, String, Option<Box<Expression>>),
    Expression(Box<Expression>),
    Return(Option<Box<Expression>>),
    Match(
        Box<Expression>,
        Vec<(Vec<Literal>, Box<Block>)>,
        Option<Box<Block>>,
    ),
    Break,
    Continue,
    Import(String),
    Asm(Vec<AsmStatement>),
    Llvm(Vec<LlvmStatement>),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AsmStatement {
    pub statement: String,
    pub operands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LlvmStatement {
    pub statement: String,
    pub operands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Function(Box<FunctionDeclaration>),
    Enum(Box<EnumDeclaration>),
    Struct(Box<StructDeclaration>),
    Asm(Box<AsmStatement>),
    Llvm(Box<LlvmStatement>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<(Box<Type>, String)>,
    pub return_type: Option<Box<Type>>,
    pub body: Box<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    pub name: String,
    pub generic_parameters: Option<Vec<GenericParameter>>,
    pub fields: Vec<(Box<Type>, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclaration {
    pub name: String,
    pub generic_parameters: Option<Vec<GenericParameter>>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumVariant {
    Named(String, Vec<(Box<Type>, String)>),
    Tuple(Vec<Box<Type>>),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}
