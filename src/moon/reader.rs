
/// A `Reader` is a single process parsing an input source.
/// A lisp dialect allows us to only have a single unified
/// Reader (Parser + Lexer), where as other languages typically
/// have separate constructs for these which make things like
/// macros a lot harder.
///
/// We don't use iterators here, we just store the index position
/// and the current, and previous values.
///
/// Rust supports unicode, so does Moon. Each index is a character, **not**
/// a byte.
pub struct Reader<'a> {

    // Current character position in the stream.
    pos: int,

    // Input stream of characters. This is currently just a string, however, it would be
    // better to have a `Stream` trait being implemented by various protocols
    // (strings, statics, files, stdin, etc...)
    stream: &'a str,

    current: char,
    previous: char
}

impl<'a> Reader<'a> {
    pub fn new(stream: &'a str) -> Reader<'a> {
        Reader {
            pos: 0,
            stream: stream,
            current: ' ',
            previous: ' '
        }
    }
}