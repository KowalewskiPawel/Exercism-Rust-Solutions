#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.len() == 0 {
        return None;
    }
    let mut new_stack: Vec<i32> = Vec::new();

    for input in inputs {
        match *input {
            CalculatorInput::Value(x) => {
                new_stack.push(x);
            }
            CalculatorInput::Add => {
                if new_stack.len() >= 2 {
                    let a = new_stack.pop();
                    let b = new_stack.pop();
                    new_stack.push(a.unwrap() + b.unwrap());
                } else {
                    return None;
                }
            }
            CalculatorInput::Subtract => {
                if new_stack.len() >= 2 {
                    let a = new_stack.pop();
                    let b = new_stack.pop();
                    new_stack.push(b.unwrap() - a.unwrap());
                } else {
                    return None;
                }
            }
            CalculatorInput::Multiply => {
                if new_stack.len() >= 2 {
                    let a = new_stack.pop();
                    let b = new_stack.pop();
                    new_stack.push(a.unwrap() * b.unwrap());
                } else {
                    return None;
                }
            }
            CalculatorInput::Divide => {
                if new_stack.len() >= 2 {
                    let a = new_stack.pop();
                    let b = new_stack.pop();
                    new_stack.push(b.unwrap() / a.unwrap());
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }
    if new_stack.len() > 1 {
        return None;
    }
    Some(new_stack[0])
}
