
use vm::op::*;

#[derive(Debug)]
pub struct VmState {
  stack: Vec<i32>,
  instructions: Vec<Op>,
  cur: usize,
  last_pop: Option<i32>
}

impl VmState {
  pub fn new(instructions: Vec<Op>) -> VmState {
    VmState {
      stack: Vec::new(),
      instructions: instructions,
      cur: 0,
      last_pop: None
    }
  }

  pub fn get_last_pop(&self) -> &Option<i32> {
    &self.last_pop
  }

  pub fn run(&mut self) {


    while self.cur < self.instructions.len() {
      let mut jmp_vector: i32 = 1;

      let cur_inst = &self.instructions[self.cur];

      match cur_inst.op() {
        &Operation::Nop => (),
        &Operation::PushInt => self.stack.push(cur_inst.unwrap_arg()),
        &Operation::PopInt => self.last_pop = self.stack.pop(),
        &Operation::Dup => {
          let last_value = self.stack[self.stack.len() -1];
          self.stack.push(last_value);
        },
        &Operation::Swp => {
          let v1 = self.stack.pop().unwrap();
          let v2 = self.stack.pop().unwrap();

          self.stack.push(v1);
          self.stack.push(v2);
        },
        &Operation::Jmp => jmp_vector = cur_inst.unwrap_arg() - (self.cur as i32),
        &Operation::JmpEqZ => {
          let v = self.stack.pop().unwrap();
          if v == 0 {
            jmp_vector = cur_inst.unwrap_arg() - (self.cur as i32)
          }
        },
        &Operation::Add => {
          let v1 = self.stack.pop().unwrap();
          let v2 = self.stack.pop().unwrap();
          self.stack.push(v1 + v2);
        },
        &Operation::Sub => {
          let v1 = self.stack.pop().unwrap();
          let v2 = self.stack.pop().unwrap();
          self.stack.push(v2 - v1);
        },
        &Operation::Mul => {
          let v1 = self.stack.pop().unwrap();
          let v2 = self.stack.pop().unwrap();
          self.stack.push(v1 * v2);
        },
        &Operation::Div => {
          let v1 = self.stack.pop().unwrap();
          let v2 = self.stack.pop().unwrap();
          self.stack.push(v2 / v1);
        },
        &Operation::Mod => {
          let v1 = self.stack.pop().unwrap();
          let v2 = self.stack.pop().unwrap();
          self.stack.push(v2 % v1);
        },
        &Operation::Print => println!("{}", self.stack.pop().unwrap()),
        i => panic!(format!("Op: {:?} not implemented", i))
      }

      let next_cur = (self.cur as i32 + jmp_vector) as usize;

      println!("cur: {} jmp: {} next_cur: {}", self.cur, jmp_vector, next_cur);

      self.cur = next_cur;
    }

  }
}

#[cfg(test)]
mod tests{

  use vm::exec::*;
  use vm::op::*;

  #[test]
  fn test_exec_simple() {

    let ops = vec![
      Op::new_1(Operation::PushInt, 5),
      Op::new_0(Operation::PopInt)
    ];

    let mut vm = VmState::new(ops);

    vm.run();

    if let Some(v) = *vm.get_last_pop() {
      assert_eq!(5, v)
    } else {
      panic!()
    }
  }

  #[test]
  fn test_print() {

    let ops = vec![
      Op::new_1(Operation::PushInt, 5),
      Op::new_0(Operation::Print)
    ];

    let mut vm = VmState::new(ops);

    vm.run();
  }

  #[test]
  fn test_add() {

    let ops = vec![
      Op::new_1(Operation::PushInt, 5),
      Op::new_1(Operation::PushInt, 5),
      Op::new_0(Operation::Dup),
      Op::new_0(Operation::Add),
      Op::new_0(Operation::Add),
      Op::new_0(Operation::PopInt)
    ];

    let mut vm = VmState::new(ops);

    vm.run();

    if let Some(v) = *vm.get_last_pop() {
      assert_eq!(15, v)
    } else {
      panic!()
    }

  }

  #[test]
  fn test_jmp() {
    let ops = vec![
      Op::new_1(Operation::PushInt, 5),
      Op::new_1(Operation::PushInt, 5),
      Op::new_0(Operation::Dup),
      Op::new_1(Operation::Jmp, 5),
      Op::new_0(Operation::Add),
      Op::new_0(Operation::Add),
      Op::new_0(Operation::PopInt)
    ];

    let mut vm = VmState::new(ops);

    vm.run();

    if let Some(v) = *vm.get_last_pop() {
      assert_eq!(10, v)
    } else {
      panic!()
    }

  }

  #[test]
  fn test_euclid() {

  }

}

