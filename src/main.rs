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

  pub fn infix_to_postfix(input: &str) {
    let mut char_array: Vec<char> = input.chars().collect();
    let mut op_stack: Vec<char> = Vec::new();  // Use Vec as a stack

    // // reversing array.. Is this necessary?
    // char_array.reverse();

    // // First phase: Push all characters into the stack
    // char_array.iter().for_each(|&entry| {
    //     op_stack.push(entry);
    // });

    for entry in &char_array {
        match entry {
            '+' | '-' | '/' | '*' => {
                // Handle operators
                println!("Operator: {}", entry);
            },
            ')' => {
                // Handle closing parenthesis
                if let Some(popped_item) = op_stack.pop() {
                    match popped_item {
                        '(' => break,
                        _ => println!("{popped_item}")
                    }
                }
            },
            '(' => {
                // Handle opening parenthesis, pushing more into the stack if necessary
                println!("Opening parenthesis: {}", entry);
                op_stack.push(entry);
            },
            _ => {
                println!("{entry}");
            }
        }
    }

    // Second phase: Process the stack while mutating it
    while let Some(entry) = op_stack.pop() {
        // match entry {
        //     '+' | '-' | '/' | '*' => {
        //         // Handle operators
        //         println!("Operator: {}", entry);
        //     },
        //     ')' => {
        //         // Handle closing parenthesis
        //         if let Some(popped_item) = op_stack.pop() {
        //             match popped_item {
        //                 '(' => break,
        //                 _ => println!("{popped_item}")
        //             }
        //         }
        //     },
        //     '(' => {
        //         // Handle opening parenthesis, pushing more into the stack if necessary
        //         println!("Opening parenthesis: {}", entry);
        //         op_stack.push(entry);
        //     },
        //     _ => {
        //         println!("{entry}");
        //     }
        // }
    }

    println!("Stack processing complete.");
}

fn main() {
    // infix_to_postfix("todd");
    infix_to_postfix("(1+2)*2/4");
}
