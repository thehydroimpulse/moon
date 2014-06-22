use lexer::Lexer;
use lexer::Token;
use ast::Ast;
use span::Span;
use result::MoonResult;

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

    pub fn execute() -> MoonResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_empty_program() {
        let mut parser = Parser::new("");
        parser.execute();
        assert_eq!(parser.ast.nodes.len(), 0);
    }
}
