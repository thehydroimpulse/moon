use vm::Value;

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
}