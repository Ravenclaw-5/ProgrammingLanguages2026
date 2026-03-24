#[derive(Clone, Debug)]
pub enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Subtract(Vec<Expression>),
    Number(i32),
}

pub fn evaluate_addition(add: &Expression) -> i32 {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        iter.fold(0, |total, next| total + evaluate(next))
    } else {
        panic!("Addition not provided");
    }
}

pub fn evaluate_multiplication(mult: &Expression) -> i32 {
    if let Expression::Multiply(expressions) = mult {
        let iter = expressions.iter();
        iter.fold(1, |total, next| total * evaluate(next))
    } else {
        panic!("Multiplication not provided");
    }
}

pub fn evaluate_subtraction(sub: &Expression) -> i32 {
    if let Expression::Subtract(expressions) = sub {
        let mut iter = expressions.iter();
        let first = iter.next().unwrap();
        iter.fold(evaluate(first), |total, next| total - evaluate(next))
    } else {
        panic!("Subtraction not provided");
    }
}

fn evaluate(expression: &Expression) -> i32 {
    match expression {
        Expression::Add(_) => evaluate_addition(expression),
        Expression::Multiply(_) => evaluate_multiplication(expression),
        Expression::Subtract(_) => evaluate_subtraction(expression),
        Expression::Number(val) => *val,
    }
}

fn main() {
    let mut expressions = Vec::new();
    expressions.push(Expression::Number(3));
    expressions.push(Expression::Number(4));
    expressions.push(Expression::Number(5));
    let add = Expression::Add(expressions);
    let multiply = Expression::Multiply(vec![add, Expression::Number(2)]);
    let result = evaluate(&multiply);
    println!("The result is {result}");
}

#[cfg(test)]
mod tests {
    use crate::{Expression, evaluate_addition, evaluate_subtraction};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_basic_addition() {
        // arrange
        let values = vec![Expression::Number(2), Expression::Number(2)];

        // act
        let sum = evaluate_addition(&Expression::Add(values));

        // assert
        assert_eq!(sum, 4);
    }

    #[test]
    fn test_basic_addition_not5() {
        // arrange
        let values = vec![Expression::Number(2), Expression::Number(2)];

        // act
        let sum = evaluate_addition(&Expression::Add(values));

        // assert
        assert_ne!(sum, 5);
    }

    #[test]
    fn test_basic_subtraction() {
        // arrange
        let values = vec![Expression::Number(2), Expression::Number(2)];

        // act
        let difference = evaluate_subtraction(&Expression::Subtract(values));

        // assert
        assert_eq!(difference, 0);
    }
}
