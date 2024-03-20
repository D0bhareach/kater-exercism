#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

/*
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut result: Option<i32> = None;
    let inp_ln = inputs.len();
    let stack:RefCell<Vec<i32>> = RefCell::new(Vec::with_capacity(inp_ln));
    
    let values = || {
        let mut s = stack.borrow_mut();
        let res = if s.len().ge(&2) {
            Some((s.pop().unwrap(), s.pop().unwrap()))
        } else {
            s.clear();
            None
        };
        res
    };

    for arm in inputs.iter().enumerate() {
       match arm {
            ((1..), CalculatorInput::Add) => {
                if let Some((y, x)) = values() {
                stack.borrow_mut().push(x + y);
                }
                else {
                    break;
                }
            },
            ((1..), CalculatorInput::Subtract) => {
                if let Some((y, x)) = values(){
                stack.borrow_mut().push(x - y);
                }
                else {
                    break;
                }
            },
            ((1..), CalculatorInput::Multiply) => {
                if let Some((y, x)) = values(){
                stack.borrow_mut().push(x * y);
                }
                else {
                    break;
                }
            },
            ((1..), CalculatorInput::Divide) => {
                if let Some((y, x)) = values(){
                stack.borrow_mut().push(x / y);
                }
                else {
                    break;
                }
            },
            (_, CalculatorInput::Value(n)) => {
                stack.borrow_mut().push(*n);
            },
            _ => break,

       } 
    };

    {
    let s = stack.borrow();
    if s.len().eq(&1) {
    if let Some(n) = s.get(0) {
        result = Some(*n)
    }
    }
}
    result
}
*/
// this brilliant solution is from comunity I failed use provided API.
use std::ops:: {Add, Sub, Div, Mul};
fn w(v: &mut Vec<i32>, f: impl Fn(i32, i32) -> i32)-> Option<i32> {
    v.pop().and_then(|y| v.pop().map(|x| f(x,y)))
}
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    // for each of inputs perfrom operation
    inputs.iter().try_fold(vec![], |mut acc, n| {
        match n {
            // note usage of referenced func. add(self, rhs: Rhs) -> Self::Output
            // takes two argument, and return result. map closure will wrap it in Option
            CalculatorInput::Add => w(& mut acc, i32::add),
            CalculatorInput::Subtract => w(& mut acc, i32::sub),
            CalculatorInput::Multiply => w(&mut acc, i32::mul),
            CalculatorInput::Divide => w(& mut acc, i32::div),
            CalculatorInput::Value(v) => Some(*v),
        }
        // if Options is not None this operaton return None
    }.map(|x| {
        acc.push(x);
        acc
    }))
    .and_then(|o| {
        match o.as_slice() {
            [v] => Some(*v),
            _ => None
        }
    })
}