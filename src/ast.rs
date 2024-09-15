use crate::token::Token;
use crate::utils::ParserError;
use serde::{Deserialize, Serialize};

/*
 * The data structures defined here should all be heap-allocated, i.e. encapsulated
 * with `Box`. When creating/modifying the data structures `Rc` or `Arc` should be used.
 *
 * Each structure should also account for whether an error was encountered during parsing.
 */

/// Represents an identifier in the syntax tree. An identifier may have an
/// associated error from the parsing process.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    /// The token representing the identifier.
    pub id: Option<Token>,
    /// Optional error encountered while parsing the identifier.
    pub error: Option<ParserError>,
}

/// Represents various literal values such as integers, floats, strings,
/// or characters. In case of a parsing error, the `Error` variant is used.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Integer(Token),
    Float(Token),
    String(Token),
    Character(Token),
    /// Captures an error during the parsing of a literal.
    Error(ParserError),
}

/// Represents an array access operation in the syntax tree.
/// Contains an expression for indexing, and allows for chained accesses.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArrayAccess {
    /// Specifies the depth of array access.
    pub level: u32,
    /// Expression for the current index.
    pub index: Box<Expression>,
    /// Recursive next access for multidimensional arrays.
    pub next: Box<ArrayAccess>,
    /// Optional error encountered while parsing the array access.
    pub error: Option<ParserError>,
}

/// Represents a function call in the syntax tree, including the function
/// identifier and arguments. Supports parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    /// The identifier of the function being called.
    pub id: Box<Identifier>,
    /// A vector of expressions representing function arguments.
    pub args: Vec<Box<Expression>>,
    /// Optional error encountered while parsing the function call.
    pub error: Option<ParserError>,
}

/// Represents primary expressions such as literals, identifiers, groups,
/// array accesses, or function calls. Parsing errors are represented using
/// the `Error` variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Primary {
    Literal(Box<Literal>),
    Identifier(Box<Identifier>),
    Group(Box<Expression>),
    ArrayAccess(Box<Identifier>, Box<ArrayAccess>),
    FunctionCall(FunctionCall),
    /// Captures an error during parsing of primary expressions.
    Error(ParserError),
}

/// Represents an operator in an expression. This includes binary and unary
/// operations. Errors are captured via the `Error` variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    /// A binary operation with an operator and two expressions.
    Binary(String, Box<Expression>, Box<Expression>),
    /// A unary operation with an operator and a single expression.
    Unary(String, Box<Expression>),
    /// Captures an error during parsing of an operator.
    Error(ParserError),
}

/// Represents an expression in the syntax tree. An expression can either
/// be an operation, a primary value, or an error.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Operation(Box<Operator>),
    Primary(Box<Primary>),
    /// Captures an error during the parsing of an expression.
    Error(ParserError),
}

/// Represents different variants of generics in the syntax tree. This includes
/// identifiers or implementations with types. Parsing errors are represented
/// using the `Error` variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GenericVariants {
    Identifier(Box<Identifier>),
    Implements(Box<Identifier>, Box<Identifier>),
    /// Captures an error during parsing of a generic variant.
    Error(ParserError),
}

/// Represents a collection of generic parameters in a declaration. Parsing
/// errors are optional.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericParameters {
    /// A vector of generic variants.
    pub generics: Vec<Box<GenericVariants>>,
    /// Optional error encountered while parsing the generic parameters.
    pub error: Option<ParserError>,
}

/// Represents different type variants such as primitives, structures,
/// enumerations, arrays, references, generics, or interfaces. Errors are
/// represented using the `Error` variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TypeVariant {
    Primitive(String),
    Structure(Box<Identifier>, Option<Box<GenericParameters>>),
    Enumeration(Box<Identifier>, Option<Box<GenericParameters>>),
    Array(Box<TypeVariant>, Box<Expression>),
    Reference(Box<TypeVariant>),
    Generic(Box<Identifier>),
    Interface(Box<Identifier>, Option<Box<GenericParameters>>),
    /// Captures an error during the parsing of a type variant.
    Error(ParserError),
}

/// Represents a type in the syntax tree, encapsulating the variant and
/// any parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Type {
    /// The variant of the type.
    pub variant: Box<TypeVariant>,
    /// Optional error encountered while parsing the type.
    pub error: Option<ParserError>,
}

/// Represents a block of statements in the syntax tree.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
    /// A list of statements in the block.
    pub statements: Vec<Statement>,
}

/// Represents an assignment statement, containing an identifier and an
/// expression.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    /// The identifier to assign a value to.
    pub id: Box<Identifier>,
    /// The expression representing the value being assigned.
    pub expr: Box<Expression>,
}

/// Represents an `if` statement in the syntax tree, including the condition,
/// the `if` block, optional `elif` statements, and an optional `else` block.
/// Errors are handled optionally.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub if_block: Box<Block>,
    pub elif_statements: Option<Vec<Box<ElifStatement>>>,
    pub else_block: Option<Box<Block>>,
    pub error: Option<ParserError>,
}

/// Represents an `elif` (else-if) statement in the syntax tree, containing
/// a condition and a block of statements. Optional errors are included.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElifStatement {
    pub condition: Box<Expression>,
    pub block: Box<Block>,
    pub error: Option<ParserError>,
}

/// Represents a variable declaration, including its state (e.g., volatile or
/// constant), type, identifier, and initializer expression. Optional parsing
/// errors are handled.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub state: u8,
    pub var_type: Box<Type>,
    pub id: Box<Identifier>,
    pub init: Box<Expression>,
    pub error: Option<ParserError>,
}

/// Represents a `match` statement, which includes case clauses,
/// an optional default clause, and optional parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchStatement {
    /// The clauses of the `match` statement.
    pub case_clauses: Vec<CaseClause>,
    /// An optional default clause.
    pub default_clause: Option<Box<Block>>,
    /// Optional error encountered while parsing the `match` statement.
    pub error: Option<ParserError>,
}

/// Represents a case clause in a `match` statement, including cases and
/// the associated block of statements.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaseClause {
    /// A list of literals representing the cases.
    pub cases: Vec<Literal>,
    /// The block of statements to execute for the matched case.
    pub case_block: Box<Block>,
    /// Optional error encountered while parsing the case clause.
    pub error: Option<ParserError>,
}

/// Represents a `return` statement in the syntax tree, containing an
/// expression and optional parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    /// The expression to return.
    pub expr: Box<Expression>,
    /// Optional error encountered while parsing the return statement.
    pub error: Option<ParserError>,
}

/// Represents a variant of a block string literal, which could either be
/// a string literal or an identifier.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlockStringLiteralVariant {
    StringLiteral(Box<Literal>),
    Identifier(Box<Identifier>),
}

/// Represents a block of LLVM code. Contains statements and optional parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LLVMBlock {
    /// A list of statements in the LLVM block.
    pub statements: Vec<BlockStringLiteralVariant>,
    /// Optional error encountered while parsing the LLVM block.
    pub error: Option<ParserError>,
}

/// Represents a block of inline assembly (ASM) code. Contains statements
/// and optional parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ASMBlock {
    /// A list of statements in the ASM block.
    pub statements: Vec<BlockStringLiteralVariant>,
    /// Optional error encountered while parsing the ASM block.
    pub error: Option<ParserError>,
}

/// Represents different kinds of statements in the syntax tree, including
/// conditional, loop, assignment, variable declaration, and more. Errors
/// are captured using the `Error` variant.
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
    /// Captures an error during parsing of a statement.
    Error(ParserError),
}

/// Represents named fields in structures or other data types.
/// Each field has a type and an identifier, with optional parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedFields {
    /// A list of fields, each represented by a type and identifier.
    pub fields: Vec<(Box<Type>, Box<Identifier>)>,
    /// Optional error encountered while parsing named fields.
    pub error: Option<ParserError>,
}

/// Represents tuple fields in a structure or other data type.
/// Each field is simply a type, with optional parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TupleFields {
    /// A list of fields, each represented by a type.
    pub fields: Vec<Box<Type>>,
    /// Optional error encountered while parsing tuple fields.
    pub error: Option<ParserError>,
}

/// Represents the variants of a structure or enumeration.
/// A variant can be named, a tuple, or a unit.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Variant {
    Named(Box<Identifier>, Box<NamedFields>),
    Tuple(Box<Identifier>, Box<TupleFields>),
    Unit(Box<Identifier>),
}

/// Represents an enumeration (enum) declaration in the syntax tree.
/// Includes an identifier, optional generics, variants, and optional
/// parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnumDeclaration {
    /// The identifier of the enum.
    pub id: Box<Identifier>,
    /// Optional generics for the enum.
    pub generics: Option<Box<GenericParameters>>,
    /// A list of optional variants for the enum.
    pub variants: Option<Vec<Variant>>,
    /// Optional error encountered while parsing the enum declaration.
    pub error: Option<ParserError>,
}

/// Represents a structure (struct) declaration in the syntax tree.
/// Encapsulates a variant and optional parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructDeclaration {
    /// The variant that defines the structure.
    pub variant: Box<Variant>,
    /// Optional error encountered while parsing the struct declaration.
    pub error: Option<ParserError>,
}

/// Represents a function declaration in the syntax tree, including its identifier,
/// visibility, constants, generics, parameters, and body. Parsing errors are optional.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    /// The identifier of the function.
    pub id: Box<Identifier>,
    /// Whether the function is public.
    pub is_pub: bool,
    /// Whether the function is constant.
    pub is_const: bool,
    /// Optional generics for the function.
    pub generics: Option<Box<GenericParameters>>,
    /// Optional parameters for the function, each represented by a type and an identifier.
    pub parameters: Option<Vec<(Box<Type>, Box<Identifier>)>>,
    /// The body of the function, represented as a block of statements.
    pub block: Box<Block>,
    /// Optional error encountered while parsing the function declaration.
    pub error: Option<ParserError>,
}

/// Represents an interface (trait) declaration in the syntax tree.
/// Includes an identifier, optional generics, a list of methods,
/// and optional parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterfaceDeclaration {
    /// The identifier of the interface.
    pub id: Box<Identifier>,
    /// Optional generics for the interface.
    pub generics: Option<Box<GenericParameters>>,
    /// A list of methods (function declarations) in the interface.
    pub methods: Vec<Box<FunctionDeclaration>>,
    /// Optional error encountered while parsing the interface declaration.
    pub error: Option<ParserError>,
}

/// Represents an implementation of an interface for a specific type.
/// Includes the interface identifier, type identifier, generics, methods,
/// and optional parsing errors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterfaceImplementation {
    /// The identifier of the interface being implemented.
    pub intf_id: Box<Identifier>,
    /// The identifier of the type implementing the interface.
    pub for_id: Box<Identifier>,
    /// The generics for the implementation.
    pub generics: Box<GenericParameters>,
    /// A list of method implementations.
    pub methods: Vec<Box<FunctionDeclaration>>,
    /// Optional error encountered while parsing the implementation.
    pub error: Option<ParserError>,
}

/// Represents a top-level declaration in the syntax tree, which could be
/// an enum, struct, function, or interface. Parsing errors are represented
/// using the `Error` variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Declaration {
    Enum(Box<EnumDeclaration>),
    Struct(Box<StructDeclaration>),
    Function(Box<FunctionDeclaration>),
    Interface(Box<InterfaceDeclaration>),
    /// Captures an error during parsing of a declaration.
    Error(ParserError),
}

/// Represents the Abstract Syntax Tree (AST) for a particular module.
/// It consists of a collection of top-level declarations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AST {
    /// A vector of top-level declarations.
    pub declarations: Vec<Box<Declaration>>,
}
