use logos::Logos;

fn convert_chars(s: &str, off_b: usize, off_e: usize) -> String {
  let mut iter = s[off_b..s.len() - off_e].chars();
  let mut s = String::new();

  while let Some(c) = iter.next() {
      if c == '\\' {
          match iter.next().unwrap() {
              '\\' => s.push('\\'),
              '\"' => s.push('\"'),
              '\'' => s.push('\''),
              'n' => s.push('\n'),
              'r' => s.push('\r'),
              't' => s.push('\t'),
              '0' => s.push('\0'),
              c => {
                  s.push('\\');
                  s.push(c)
              }
          };
      } else {
          s.push(c);
      }
  }

  s
}

#[derive(Debug, PartialEq)]
pub enum CompareOperation {
  Equ,
  Neq,
  Gre,
  Geq,
  Les,
  Leq,
}

#[derive(Debug, PartialEq)]
pub enum BitwiseOperation {
  Orr,
  And,
  Xor,
  Nor,
  Lsh,
  Rsh,
  Not,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
  Agn,
  Add,
  Sub,
  Mul,
  Div,
  Exp,
  Mod,
  Ple,
  Pri,
  Btw(BitwiseOperation),
  Cmp(CompareOperation),
}

#[derive(Debug, PartialEq)]
pub enum Punctuation {
  Period,
  Commaa,
  Colonn,
  DubCol,
  SemCol,
  LParen,
  RParen,
  LBrack,
  RBrack,
  LBrace,
  RBrace,
}

#[derive(Debug, PartialEq)]
pub enum FlowKeyword {
  If,
  Then,
  Else,
  For,
  While,
  Match,
  Break,
  Continue,
  Return,
}

#[derive(Debug, PartialEq)]
pub enum TypeKeyword {
  Struct,
  SumTyp,
  AliasT,
  Global,
  Public,
  Mutabl,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
  FunDec,
  VarDec,
  AsCast,
  TypeOf,
  LoopIn,
  FlowKeyw(FlowKeyword),
  TypeKeyw(TypeKeyword),
}

#[derive(Logos, Debug, PartialEq)]
pub enum VisconLexer {
  #[token(":=", |_| Operation::Agn)]
  #[token("+",  |_| Operation::Add)]
  #[token("-",  |_| Operation::Sub)]
  #[token("*",  |_| Operation::Mul)]
  #[token("/",  |_| Operation::Div)]
  #[token("**", |_| Operation::Exp)]
  #[token("%",  |_| Operation::Mod)]
  #[token("<:", |_| Operation::Ple)]
  #[token(":>", |_| Operation::Pri)]

  #[token("|",  |_| Operation::Btw(BitwiseOperation::Orr))]
  #[token("&",  |_| Operation::Btw(BitwiseOperation::And))]
  #[token("^",  |_| Operation::Btw(BitwiseOperation::Xor))]
  #[token("<<", |_| Operation::Btw(BitwiseOperation::Lsh))]
  #[token(">>", |_| Operation::Btw(BitwiseOperation::Rsh))]
  #[token("!",  |_| Operation::Btw(BitwiseOperation::Not))]
  
  #[token("==", |_| Operation::Cmp(CompareOperation::Equ))]
  #[token("!=", |_| Operation::Cmp(CompareOperation::Neq))]
  #[token(">",  |_| Operation::Cmp(CompareOperation::Gre))]
  #[token(">=", |_| Operation::Cmp(CompareOperation::Geq))]
  #[token("<",  |_| Operation::Cmp(CompareOperation::Les))]
  #[token("<=", |_| Operation::Cmp(CompareOperation::Leq))]
  Operation(Operation),



  #[token("=",   |_| Operation::Agn)]
  #[token("+=",  |_| Operation::Add)]
  #[token("-=",  |_| Operation::Sub)]
  #[token("*=",  |_| Operation::Mul)]
  #[token("/=",  |_| Operation::Div)]
  #[token("**=", |_| Operation::Exp)]
  #[token("%=",  |_| Operation::Mod)]

  #[token("|=",  |_| Operation::Btw(BitwiseOperation::Orr))]
  #[token("&=",  |_| Operation::Btw(BitwiseOperation::And))]
  #[token("^=",  |_| Operation::Btw(BitwiseOperation::Xor))]
  #[token("<<=", |_| Operation::Btw(BitwiseOperation::Lsh))]
  #[token(">>=", |_| Operation::Btw(BitwiseOperation::Rsh))]
  AssignOperation(Operation),


  
  #[token(".",  |_| Punctuation::Period)]
  #[token(",",  |_| Punctuation::Commaa)]
  #[token(":",  |_| Punctuation::Colonn)]
  #[token("::", |_| Punctuation::DubCol)]
  #[token(";",  |_| Punctuation::SemCol)]

  #[token("(", |_| Punctuation::LParen)]
  #[token(")", |_| Punctuation::RParen)]

  #[token("[", |_| Punctuation::LBrack)]
  #[token("]", |_| Punctuation::RBrack)]

  #[token("{", |_| Punctuation::LBrace)]
  #[token("}", |_| Punctuation::RBrace)]
  Punctuation(Punctuation),



  #[token("function", |_| Keyword::FunDec)]
  #[token("let",      |_| Keyword::VarDec)]
  #[token("as",       |_| Keyword::AsCast)]
  #[token("typeof",   |_| Keyword::TypeOf)]
  #[token("in",       |_| Keyword::LoopIn)]
  
  #[token("struct", |_| Keyword::TypeKeyw(TypeKeyword::Struct))]
  #[token("sum",    |_| Keyword::TypeKeyw(TypeKeyword::SumTyp))]
  #[token("type",   |_| Keyword::TypeKeyw(TypeKeyword::AliasT))]
  #[token("global", |_| Keyword::TypeKeyw(TypeKeyword::Global))]
  #[token("pub",    |_| Keyword::TypeKeyw(TypeKeyword::Public))]
  #[token("mut",    |_| Keyword::TypeKeyw(TypeKeyword::Mutabl))]

  #[token("if",       |_| Keyword::FlowKeyw(FlowKeyword::If))]
  #[token("then",     |_| Keyword::FlowKeyw(FlowKeyword::Then))]
  #[token("else",     |_| Keyword::FlowKeyw(FlowKeyword::Else))]
  #[token("for",      |_| Keyword::FlowKeyw(FlowKeyword::For))]
  #[token("while",    |_| Keyword::FlowKeyw(FlowKeyword::While))]
  #[token("continue", |_| Keyword::FlowKeyw(FlowKeyword::Continue))]
  #[token("break",    |_| Keyword::FlowKeyw(FlowKeyword::Break))]
  #[token("return",   |_| Keyword::FlowKeyw(FlowKeyword::Return))]
  Keyword(Keyword),



  #[regex(r#""([^\\"]|(\\[\S\s]))*""#, |lex| convert_chars(lex.slice(), 1, 1))]
  #[regex(r#"l"[^"]*""#, |lex| convert_chars(lex.slice(), 2, 1).parse())]
  StriLiteral(String),
  #[regex(r#"'([^\\"]|(\\[\S\s]))'"#, |lex| convert_chars(lex.slice(), 1, 1).chars().next().unwrap())]
  #[regex(r#"l'[^"]'"#, |lex| convert_chars(lex.slice(), 2, 1).chars().next().unwrap())]
  CharLiteral(char),

  #[regex(r#"[0-9]+"#, |lex| lex.slice().parse())]
  InteLiteral(i64),
  #[regex(r#"[0-9]+u"#, |lex| lex.slice().trim_end_matches('u').parse())]
  #[regex(r#"0x[0-9A-F]+"#, |lex| u64::from_str_radix(lex.slice().trim_start_matches("0x"), 16))]
  #[regex(r#"0b[01]+"#, |lex| u64::from_str_radix(lex.slice().trim_start_matches("0b"), 2))]
  WordLiteral(u64),
  #[regex(r"[0-9]+(\.[0-9]*([eE][+-]?[0-9]+)?|[eE][+-]?[0-9]+)", |lex| lex.slice().parse())]
  FloatLiteral(f64),

  #[regex(r#"true|false"#, |lex| "true" == lex.slice())]
  BoolLiteral(bool),



  #[regex(r#"[a-zA-Z]\w*"#, |lex| {lex.slice().parse()})]
  Identifier(String),



  #[regex(r#"//.*"#, logos::skip)]
  #[regex(r#"/\*([^*]*\*+)+/"#, logos::skip)]
  Comment,

  #[error]
  #[regex(r"[ \t\n\f]+", logos::skip)]
  Error,
}

pub fn lexically_analyze(
  input_text: &str,
) -> Vec<(VisconLexer, std::ops::Range<usize>)> {
  let mut token_list = Vec::new();
  let mut lex = VisconLexer::lexer(input_text);
  loop {
    match lex.next() {
      Some(token) => token_list.push((token, lex.span())),
      None => return token_list,
    }
  }
}
