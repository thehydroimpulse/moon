use gc::Gc;
use std::fmt::{Show,Formatter};
use std::fmt;
use std::rc::{Rc};
//use ast::{Ast, LetExprAst, BindingExprAst, NumberExprAst, BinaryExprAst};

/// A generic value within our virtual machine. A value
/// can be anything defined within this enum.
pub enum Value {
  Int(int),
  Str(~str),
  Pair(~Value, ~Value)
}

pub enum OpCode {
  XAdd(int, int),
  XSub(int, int),
  XMul(int, int),
  XDiv(int, int),
  XPush(~str, Value),
  XPop
}

pub enum Register {
  Reg1,
  Reg2,
  Reg3,
  Reg4,
  RegPc // Program/OpCode Counter
}

/// A virtual machine that holds needed objects (gc, stack, etc...) and
/// the current state. 
pub struct Vm {
  /// A global garbage collector.
  gc: Gc,

  /// Our small stack. For simplicity, we'll limit this to 
  /// a static value.
  stack: ~[Rc<Value>],

  registers: [Option<Value>, ..4],

  /// OpCodes
  opcodes: ~[OpCode],

  pc: int
}

impl Vm {
  pub fn new() -> Vm {
    Vm {
      gc: Gc::new(),
      stack: ~[],
      opcodes: ~[],
      registers: [None, None, None, None],
      pc: 0
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

  pub fn run(&mut self, string: &str) {

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

  #[test]
  fn should_run() {

  }

  #[test]
  fn should_parse_input() {
    
  }

}