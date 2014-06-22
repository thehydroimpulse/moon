use lexer::Lexer;
use lexer::Token;
use ast::Ast;
use span::Span;

/// The parser is responsible for taking a stream of generic tokens
/// that come from the lexer and turn them into a correct Abstract Synax
/// Tree (AST). The AST will be responsible for having the domain-logic
/// to compile the specialized nodes into something we can execute
/// (either JIT or interpreted.).
#[deriving(PartialEq, Show)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    ast: AstRoot
}


impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            lexer: Lexer::new(input),
            ast: AstRoot::new()
        }
    }
}
