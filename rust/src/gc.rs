use vm::Value;
use std::rc::Rc;

#[experimental]
pub struct Gc {
  /// Describe the GC roots that we have. These are stack values.
  roots: ~[Value]
}

#[experimental]
impl Gc {
  pub fn new() -> Gc {
    Gc {
      roots: ~[]
    }
  }

  pub fn register_root(&mut self, root: Rc<Value>) {

  }
}


#[cfg(test)]
mod test {
  use super::Gc;

  #[test]
  fn should_have_empty_roots() {
    let gc = Gc::new();
    assert_eq!(gc.roots.len(), 0);
  }

}