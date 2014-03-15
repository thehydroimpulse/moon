use gc::Gc;
use std::fmt::{Show,Formatter};
use std::fmt;
use std::rc::{Rc};

/// A generic value within our virtual machine. A value
/// can be anything defined within this enum.
pub enum Value {
  Int(u32),
  Str(~str),
  Pair(~Value, ~Value)
}

/// A virtual machine that holds needed objects (gc, stack, etc...) and
/// the current state. 
pub struct Vm {
  /// A global garbage collector.
  gc: Gc,

  /// Our small stack. For simplicity, we'll limit this to 
  /// a static value.
  stack: ~[Rc<Value>]
}

impl Vm {
  pub fn new() -> Vm {
    Vm {
      gc: Gc::new(),
      stack: ~[]
    }
  }

  pub fn push(&mut self, val: Value) {
    let ptr = Rc::new(val);
    self.gc.register_root(ptr.clone());
    self.stack.push(ptr);
  }

  pub fn pop(&mut self) -> Option<Value> {
    Some((*self.stack.pop().unwrap()).clone())
  }
}

impl Eq for Value {
  fn eq(&self, other: &Value) -> bool {
    match *self {
      Int(i) => {
        match *other {
          Int(ii) => i == ii,
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

impl Clone for Value {
  fn clone(&self) -> Value {
    match *self {
      Int(i) => Int(i),
      Str(ref s) => Str(s.clone()),
      _ => fail!("Not implemented.")
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


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn empty_stack_default() {
    let vm = Vm::new();
    assert_eq!(vm.stack.len(), 0);
  }

  #[test]
  fn should_push_item() {
    let mut vm = Vm::new();
    let val = Str(~"Hello World");
    vm.push(val);
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(*vm.stack[0], Str(~"Hello World"));
  }

}