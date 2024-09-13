use crate::token::Token;
use crate::utils::ParserError;
use serde::{Deserialize, Serialize};

/*
 * The data structures defined here should all be heap-allocated, i.e. encapsulated
 * with `Box`. When creating/modifying the data structures `Rc` or `Arc` should be used.
 *
 * Each structure should also account for whether an error was encountered during parsing.
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub id: Option<Token>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Integer(Token),
    Float(Token),
    String(Token),
    Character(Token),
    Error(ParserError),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArrayAccess {
    pub level: u32, // Reasonable limit for array accesses.
    pub index: Box<Expression>,
    pub next: Box<ArrayAccess>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    pub id: Box<Identifier>,
    pub args: Vec<Box<Expression>>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Primary {
    Literal(Box<Literal>),
    Identifier(Box<Identifier>),
    Group(Box<Expression>),
    ArrayAccess(Box<Identifier>, Box<ArrayAccess>),
    FunctionCall(FunctionCall),
    Error(ParserError),
}

/*
 * Whatever operator precedence is used will be implemented in the Parser.
 * This allows for a simplified and streamlined approach.
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    Binary(String, Box<Expression>, Box<Expression>),
    Unary(String, Box<Expression>),
    Error(ParserError),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Operation(Box<Operator>),
    Primary(Box<Primary>),
    Error(ParserError),
}

/*
 * Generics
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GenericVariants {
    Identifier(Box<Identifier>),
    Implements(Box<Identifier>, Box<Identifier>),
    Error(ParserError),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericParameters {
    pub generics: Vec<Box<GenericVariants>>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TypeVariant {
    Primitive(String), // e.g., "i32", "f64", etc.
    Structure(Box<Identifier>, Option<Box<GenericParameters>>),
    Enumeration(Box<Identifier>, Option<Box<GenericParameters>>),
    Array(Box<TypeVariant>, Box<Expression>), // Array type with size
    Reference(Box<TypeVariant>),              // Pointer/Reference type
    Generic(Box<Identifier>),                 // Generic type
    Interface(Box<Identifier>, Option<Box<GenericParameters>>), // Traits/Interfaces
    Error(ParserError),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub variant: Box<TypeVariant>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
}

/*
 * Statements
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub id: Box<Identifier>,
    pub expr: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub if_block: Box<Block>,
    pub elif_statements: Option<Vec<Box<ElifStatement>>>,
    pub else_block: Option<Box<Block>>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElifStatement {
    pub condition: Box<Expression>,
    pub block: Box<Block>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub state: u8, // whether volatile or const. 0 for none, 1 for volatile, 2 for const.
    pub var_type: Box<Type>,
    pub id: Box<Identifier>,
    pub init: Box<Expression>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchStatement {
    pub case_clauses: Vec<CaseClause>,
    pub default_clause: Option<Box<Block>>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaseClause {
    pub cases: Vec<Literal>,
    pub case_block: Box<Block>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub expr: Box<Expression>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlockStringLiteralVariant {
    StringLiteral(Box<Literal>),
    Identifier(Box<Identifier>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LLVMBlock {
    pub statements: Vec<BlockStringLiteralVariant>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ASMBlock {
    pub statements: Vec<BlockStringLiteralVariant>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    If(IfStatement),
    Loop(Box<Block>),
    Assign(Assignment),
    Var(VariableDeclaration),
    Match(MatchStatement),
    Break,
    Continue,
    FunctionCall(FunctionCall),
    LLVM(LLVMBlock),
    ASM(ASMBlock),
    Error(ParserError),
}

/*
 * Common
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedFields {
    pub fields: Vec<(Box<Type>, Box<Identifier>)>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TupleFields {
    pub fields: Vec<Box<Type>>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Variant {
    Named(Box<Identifier>, Box<NamedFields>),
    Tuple(Box<Identifier>, Box<TupleFields>),
    Unit(Box<Identifier>),
}

/*
 * Enumerations
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumDeclaration {
    pub id: Box<Identifier>,
    pub generics: Option<Box<GenericParameters>>,
    pub variants: Option<Vec<Variant>>,
    pub error: Option<ParserError>,
}

/*
 * Structures
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructDeclaration {
    pub variant: Box<Variant>, // Encapsulate the struct inside a variant
    pub error: Option<ParserError>,
}

/*
 * Functions
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub id: Box<Identifier>,
    pub is_pub: bool,
    pub is_const: bool,
    pub generics: Option<Box<GenericParameters>>,
    pub parameters: Option<Vec<(Box<Type>, Box<Identifier>)>>,
    pub block: Box<Block>,
    pub error: Option<ParserError>,
}

/*
 * Defininitions
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterfaceDeclaration {
    pub id: Box<Identifier>,
    pub generics: Option<Box<GenericParameters>>,
    pub methods: Vec<Box<FunctionDeclaration>>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterfaceImplementation {
    pub intf_id: Box<Identifier>,
    pub for_id: Box<Identifier>,
    pub generics: Box<GenericParameters>,
    pub methods: Vec<Box<FunctionDeclaration>>,
    pub error: Option<ParserError>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Declaration {
    Enum(Box<EnumDeclaration>),
    Struct(Box<StructDeclaration>),
    Function(Box<FunctionDeclaration>),
    Interface(Box<InterfaceDeclaration>),
    Error(ParserError),
}

/*
 * The head or root of the syntax tree.
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AST {
    pub declarations: Vec<Box<Declaration>>,
}
