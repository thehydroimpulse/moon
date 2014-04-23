
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
pub struct Reader {

}