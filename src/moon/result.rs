use std::str::SendStr;
use std::io;
use span::Span;

pub type MoonResult<T> = Result<T, MoonError>;

#[deriving(PartialEq, Show)]
pub struct MoonError {
    span: Span,
    kind: MoonErrorKind,
    message: SendStr
}

#[deriving(PartialEq, Show)]
pub enum MoonErrorKind {
    Noop
}

impl MoonError {
    pub fn new<T: IntoMaybeOwned<'static>>(span: Span,
                                          kind: MoonErrorKind,
                                          msg: T) -> MoonError {
        MoonError {
            span: span,
            kind: kind,
            message: msg.into_maybe_owned()
        }
    }
}
