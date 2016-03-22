
#[derive(Debug,PartialEq,Eq)]
pub enum Operation {
  Nop,
  PushInt,
  PopInt,
  Dup,
  Jmp,
  JmpEqZ,
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Print,
  Input,
  Swp,
  Debug
}

impl Operation {

  pub fn op(self, v: Option<i32>) -> Op {
    if let Some(arg) = v {
      self.op_1(arg)
    }  else {
      self.op_0()
    }
  }

  pub fn op_0(self) -> Op {
    Op::new_0(self)
  }

  pub fn op_1(self, arg: i32) -> Op {
    Op::new_1(self, arg)
  }
}

impl From<u8> for Operation {
  fn from(val: u8) -> Operation {
    use self::Operation::*;

    match val {
      0 => Nop,
      1 => PushInt,
      2 => PopInt,
      3 => Dup,
      4 => Jmp,
      5 => JmpEqZ,
      6 => Add,
      7 => Sub,
      8 => Mul,
      9 => Div,
      10 => Mod,
      11 => Print,
      12 => Input,
      13 => Swp,
      14 => Debug,
      _ => panic!()
    }
  }
}

#[derive(Debug)]
pub struct Op {
  operation: Operation,
  arg: Option<i32>
}

impl Op {

  pub fn new_0(op: Operation) -> Op {
    Op {
      operation: op,
      arg: None
    }
  }

  pub fn new_1(op: Operation, arg: i32) -> Op {
    Op {
      operation: op,
      arg: Some(arg)
    }
  }

  pub fn op(&self) -> &Operation {
    &self.operation
  }

  pub fn unwrap_arg(&self) -> i32 {
    self.arg.unwrap()
  }
}

use std::marker::PhantomData;

pub struct OpIter<'a, T: 'a>
where T: Iterator<Item = &'a usize> {
  iter: T,
  phantom: PhantomData<&'a T>
}

impl<'a, T> From<T> for OpIter<'a, T>
where T: Iterator<Item = &'a usize> {
  fn from(iter: T) -> OpIter<'a, T> {
    OpIter {
      iter: iter,
      phantom: PhantomData
    }
  }
}

impl<'a, T> Iterator for OpIter<'a, T>
where T: Iterator<Item = &'a usize> {
  type Item = Op;

  fn next(&mut self) -> Option<Op> {
    if let Some(op) = self.iter.next()
      .map(|op_val_usize| Operation::from(*op_val_usize as u8)) {

        let arg = match op {
          Operation::PushInt => Some(*self.iter.next().unwrap() as i32),
          Operation::JmpEqZ => Some(*self.iter.next().unwrap() as i32),
          _ => None
        };

        Some(Op {
          operation: op,
          arg: arg
        })

    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {

  use vm::op::*;

  #[test]
  fn test_op_from_u8() {
    let operation = Operation::from(8 as u8);

    assert_eq!(Operation::Mul, operation);
  }

  #[test]
  fn test_op_from_it() {
    let ops: Vec<usize> = vec![0, 1, 2, 2];

    let iter = ops.iter();

    let mut op_iter = OpIter::from(iter);

    let op1 = op_iter.next().unwrap();
    let op2 = op_iter.next().unwrap();
    let op3 = op_iter.next().unwrap();


    if let Some(_) = op_iter.next() {
      panic!();
    }

    assert_eq!(Operation::Nop, op1.operation);
    assert_eq!(Operation::PushInt, op2.operation);
    assert_eq!(Some(2), op2.arg);
    assert_eq!(Operation::PopInt, op3.operation);
  }
}