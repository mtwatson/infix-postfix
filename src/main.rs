struct Stack<T> {
    stack: Vec<T>,
}
  
impl<T> Stack<T> {
  fn new() -> Self {
    Stack { stack: Vec::new() }
  }

  fn length(&self) -> usize {
    self.stack.len()
  }

  fn pop(&mut self) -> Option<T> {
    self.stack.pop()
  }

  fn push(&mut self, item: T) {
    self.stack.push(item)
  }

  fn is_empty(&self) -> bool {
    self.stack.is_empty()
  }

  fn peek(&self) -> Option<&T> {
    self.stack.last()
  }
}

fn precedence(op: char) -> u8 {
  match op {
    '+' | '-' => 1,
    '*' | '/' => 2,
    _ => 0,
  }
}

pub fn infix_to_postfix(input: &str) {
  let char_array: Vec<char> = input.chars().collect();
  let mut op_stack: Stack<char> = Stack::new();  // Use Vec as a stack
  
  // precedence:
  //  * and / have higher than + and - 
  //  * has the same as / and + the same as - 
  for entry in &char_array {
      match entry {
          '+' | '-' | '/' | '*' => {
            let mut next_op = op_stack.peek();
            // Handle operators

            if op_stack.is_empty() || next_op == Some(&'(') {
              op_stack.push(*entry);
            } else {
                loop {
                  if let Some(value) = next_op {
                      let entry_precedence = precedence(*entry);
                      let stack_top_precendence = precedence(*value);
                      
                      if stack_top_precendence >= entry_precedence {
                        let popped_item = op_stack.pop().unwrap();
                        next_op = op_stack.peek();
                        println!("{popped_item} ");
                        // break;
                      } else {
                          op_stack.push(*entry);
                          break
                      }
                  } else {
                      op_stack.push(*entry);
                      break
                  }
                }
            }
          },
          ')' => {
              // Handle closing parenthesis
              while let Some(popped_item) = op_stack.pop() {
                  match popped_item {
                       '(' => break,
                    //   '(' => (),
                      _ => println!("{popped_item}")
                  }
              }
          },
          '(' => {
              // Handle opening parenthesis, pushing more into the stack if necessary
              op_stack.push(*entry);
          },
          _ => {
              println!("{entry}");
          }
      }
  }
  // Second phase: Process the stack while mutating it
  while let Some(entry) = op_stack.pop() {
    println!("{entry}");
  }
}

fn main() {
    println!("======================");
    infix_to_postfix("1+2*(3-4)*(5+6*7)-8");
    println!("======================");
}
