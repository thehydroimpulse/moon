use vm::Value;
use std::rc::Rc;

#[experimental]
pub struct Gc {
  /// Describe the GC roots that we have. These are stack values.
  roots: ~[Rc<Value>]
}

#[experimental]
impl Gc {
  pub fn new() -> Gc {
    Gc {
      roots: ~[]
    }
  }

  pub fn register_root(&mut self, root: Rc<Value>) {
    self.roots.push(root);
  }
}


#[cfg(test)]
mod test {
  use super::Gc;
  use vm::Value;
  use vm::Int;
  use std::rc::Rc;

  #[test]
  fn should_have_empty_roots() {
    let gc = Gc::new();
    assert_eq!(gc.roots.len(), 0);
  }

  #[test]
  fn should_add_root() {
    let val = Int(55);
    let mut gc = Gc::new();
    gc.register_root(Rc::new(val));
  }

}