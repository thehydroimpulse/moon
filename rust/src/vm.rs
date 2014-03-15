use gc::Gc;
use std::fmt::{Show,Formatter};
use std::fmt;

/// A generic value within our virtual machine. A value
/// can be anything defined within this enum.
pub enum Value {
  Int(u32),
  Str(~str),
  Pair(~Value, ~Value)
}

impl Eq for Value {
  fn eq(&self, other: &Value) -> bool {
    match *self {
      Int(i) => {
        match *other {
          Int(ii) => i == i,
          _ => fail!("Invalid comparison.")
        }
      },
      Str(ref r) => {
        match *other {
          Str(ref s) => r == s,
          _ => fail!("Invalid comparison.")
        }
      },
      _ => fail!("Missing further `Eq` implementations.")
    }
  }
}

impl Show for Value {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match *self {
      Int(i) => write!(f.buf, "({})", i),
      Str(ref s) => write!(f.buf, "({})", s),
      _ => write!(f.buf, "({})", *self )
    }
  }
}

/// A virtual machine that holds needed objects (gc, stack, etc...) and
/// the current state. 
pub struct Vm {
  /// A global garbage collector.
  gc: Gc,

  /// Our small stack. For simplicity, we'll limit this to 
  /// a static value.
  stack: ~[Value],

  /// Stack Size
  stack_size: u64
}

impl Vm {
  pub fn new() -> Vm {
    Vm {
      gc: Gc::new(),
      stack: ~[],
      stack_size: 0
    }
  }

  pub fn push(&mut self, val: Value) {
    self.stack.push(val);
    self.stack_size += 1;
  }

  pub fn pop(&mut self) -> Option<Value> {
    self.stack_size -= 1;
    self.stack.pop()
  }
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn empty_stack_default() {
    let vm = Vm::new();
    assert_eq!(vm.stack_size, 0);
  }

  #[test]
  fn should_push_item() {
    let mut vm = Vm::new();
    let val = Str(~"Hello World");
    vm.push(val);
    assert_eq!(vm.stack_size, 1);
    assert_eq!(vm.stack[0], Str(~"Hello World"));
  }

}