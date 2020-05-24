// maps to jlox-java file jlox/TokenType.java
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  // Single Character Tokens
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier,
  STRING { literal: String },
  Number { literal: f64 },

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof
}
