use lexer;

/// A generic trait that's implemented for each Ast node. We can't use
/// enums for the Ast because they are a union type, thus, they
/// are as large as the largest variant. This isn't a good fit for
/// a primitive that's used a **lot** throughout the compiler.
pub trait Ast {
    fn compile(&self);
}

pub struct AstRoot {
    nodes: Vec<Box<Ast>>
}

impl AstRoot {
    pub fn new() -> AstRoot {
        AstRoot {
            nodes: Vec::new()
        }
    }
}

impl Ast for AstRoot {
    fn compile(&self) {}
}

#[cfg(test)]
mod test {
    use super::*;
}
