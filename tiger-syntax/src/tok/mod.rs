pub use self::lexer::Lexer;

pub type Span<T> = (usize, T, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error;

pub const KEYWORDS: &'static [(&'static str, Tok<'static>)] = &[
    ("nil", Tok::Nil),
    ("break", Tok::Break),
    ("if", Tok::If),
    ("then", Tok::Then),
    ("else", Tok::Else),
    ("type", Tok::Type),
    ("function", Tok::Function),
    ("var", Tok::Var),
    ("while", Tok::While),
    ("for", Tok::For),
    ("to", Tok::To),
    ("do", Tok::Do),
    ("let", Tok::Let),
    ("in", Tok::In),
    ("end", Tok::End),
    ("array", Tok::Array),
    ("of", Tok::Of),
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    /// Symbols start with a letter ([a-zA-Z]), followed by any more
    /// letters, underscores, or digits. Symbols are case sensitive, and
    /// keywords cannot be used as symbols.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// foo
    /// foo_bar1
    /// FOO
    /// ```
    Symbol(&'input str),
    /// An integer literal is one or more digits from 0-9.
    Int(&'input str),
    /// TODO: Doc.
    String(&'input str),

    Nil,
    Break,
    If,
    Then,
    Else,
    Type,
    Function,
    Var,
    While,
    For,
    To,
    Do,
    Let,
    In,
    End,
    Array,
    Of,

    Assign,
    Hash,  // TODO: This is a hack for now.
    Colon,
    Semi,
    Comma,
    Dot,
    LParen,
    RParen,
    LBrack,
    RBrack,
    LBrace,
    RBrace,
    Eq,
    Neq,
    And,
    Or,
    Plus,
    Minus,
    Times,
    Divide,
    Lt,
    Le,
    Gt,
    Ge,
}

mod lexer;
