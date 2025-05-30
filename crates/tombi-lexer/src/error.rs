#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    span: tombi_text::Span,
    range: tombi_text::Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidKey,
    InvalidBasicString,
    InvalidLiteralString,
    InvalidMultilineBasicString,
    InvalidMultilineLiteralString,
    InvalidNumber,
    InvalidOffsetDateTime,
    InvalidLocalDateTime,
    InvalidLocalDate,
    InvalidLocalTime,
    InvalidLineBreak,
    InvalidToken,
}

impl Error {
    #[inline]
    pub fn new(kind: ErrorKind, (span, range): (tombi_text::Span, tombi_text::Range)) -> Self {
        Self { kind, span, range }
    }

    #[inline]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[inline]
    pub fn span(&self) -> tombi_text::Span {
        self.span
    }

    #[inline]
    pub fn range(&self) -> tombi_text::Range {
        self.range
    }
}
