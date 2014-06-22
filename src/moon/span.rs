/// A `Span` keeps track of a position in the source
/// stream. This allows us to later accurately look
/// back to the original location and display errors or
/// do something else with the source.
///
/// Spans keeps track for a column and line number.
#[deriving(PartialEq, Show)]
pub struct Span {
    line: uint,
    col: uint
}


impl Span {

    /// Create a new Span given a line number and column.
    pub fn new(line: uint, col: uint) -> Span {
        Span {
            line: line,
            col: col
        }
    }
}
