use logos::Span;
use super::super::parsing::lexer::Operation;

pub type BoxedAst = Box<Ast>;

pub enum Ast {
  Empty,
  
  // Trait definition
  // 1: Span, 2: Trait name, 3: Contents of the curly braces
  TraitDef(Span, String, BoxedAst),
  // Struct definition
  // 1: Span, 2: Struct Name, 3: List of variable (name, type) pairs
  StructDef(Span, String, Vec<(String, BoxedAst)>),
  // Struct implementation block
  // 1: Span, 2: Struct name, 3: Optional trait name, 4: Contents of curly braces
  StructImplementBlock(Span, String, Option<String>, BoxedAst),

  // Sum type definition
  // 1: Span, 2: Type name, 3: List of Tag (name, list of content types) pairs
  SumDef(Span, String, Vec<(String, Vec<BoxedAst>)>),
  // Function declaration
  // 1: Span, 2: Name, 3: List of parameter (name, type) pairs, 4: Return type, 5: Contents
  FuncDec(Span, String, Vec<(String, BoxedAst)>, BoxedAst, BoxedAst),
  // Variable declaration
  // 1: Span, 2: Name, 3: Type, 4: Optional value
  VarDec(Span, String, BoxedAst),
  // Variable assignment
  // 1: Span, 2: Name, 3: Value
  VarAssign(Span, String, BoxedAst),

  // Match statement
  // 1: Span, 2: Statement to match, 3: List of (pattern, match code) pairs
  Match(Span, BoxedAst, Vec<(BoxedAst, BoxedAst)>),
  // If statement
  // 1: Span, 2: Conditional, 3: Contents, 4: Optional else
  If(Span, BoxedAst, BoxedAst, Option<BoxedAst>),
  // While loops
  // 1: Span, 2: Conditional, 3: Contents
  While(Span, BoxedAst, BoxedAst),
  // C-style for loops
  // 1: Span, 2: (Initial, Conditional, Increment) tuple, 3: Contents
  ForC(Span, (BoxedAst, BoxedAst, BoxedAst), BoxedAst),
  // For each loops
  // 1: Span, 2: (variable type and name, collection to iterate over) tuple, 3: Contents
  ForEach(Span, (BoxedAst, BoxedAst), BoxedAst),

  // Infix operator
  // 1: Span, 2: Operation identifier, 3: Left side value, 4: Right side value
  Infix(Span, Operation, BoxedAst, BoxedAst),
  // Prefix operator
  // 1: Span, 2: Operation identifier, 3: Value to operate on
  Prefix(Span, Operation, BoxedAst),
  // Postfix operator
  // 1: Span, 2: Operation identifier, 3: Value to operate on
  Postfix(Span, Operation, BoxedAst),

  // Value
  // 1: Span, 2: Value
  Value(Span, AstValue),
}

pub enum AstValue {
  StriLit(String),
  CharLit(char),
  InteLit(i64),
  WordLit(u64),
  FloaLit(f64),
  Identif(String),
}

impl Ast {
  fn get_span(&self) -> Span {
    match self {
      Self::TraitDef(s, _, _)
      | Self::StructDef(s, _, _)
      | Self::StructImplementBlock(s, _, _, _)
      | Self::SumDef(s, _, _)
      | Self::FuncDec(s, _, _, _, _)
      | Self::VarDec(s, _, _)
      | Self::VarAssign(s, _, _)
      | Self::Match(s, _, _)
      | Self::If(s, _, _, _)
      | Self::While(s, _, _)
      | Self::ForC(s, _, _)
      | Self::ForEach(s, _, _)
      | Self::Infix(s, _, _, _)
      | Self::Prefix(s, _, _)
      | Self::Postfix(s, _, _)
      | Self::Value(s, _) => s.clone(),

      Self::Empty => panic!("Attempted to get span of empty AST node."),
    }
  }
}

pub fn panic() {Ast::Empty.get_span();}