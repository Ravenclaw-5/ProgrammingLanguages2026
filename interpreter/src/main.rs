#[derive(Clone, Debug)]
pub enum Expression {
    Add(Vec<Expression>),
    Multiply(Vec<Expression>),
    Subtract(Vec<Expression>),
    Divide(Vec<Expression>),
    Variable(String),
    Number(f64),
}

pub struct Environment {
    key: String,
    value: Expression,
}

impl Environment {
    fn value_for_key(self: &Environment, key: &String) -> &Expression {
        if &self.key == key {
            &self.value
        } else {
            panic!("Key not found in environment");
        }
    }

    fn new() -> Environment {
        Environment {
            key: String::from(""),
            value: Expression::Number(0.0),
        }
    }
}

pub fn evaluate_addition(add: &Expression, environment: &Environment) -> f64 {
    if let Expression::Add(expressions) = add {
        let iter = expressions.iter();
        iter.fold(0.0, |total, next| total + evaluate(next, environment))
    } else {
        panic!("Addition not provided");
    }
}

pub fn evaluate_multiplication(mult: &Expression, environment: &Environment) -> f64 {
    if let Expression::Multiply(expressions) = mult {
        let iter = expressions.iter();
        iter.fold(1.0, |total, next| total * evaluate(next, environment))
    } else {
        panic!("Multiplication not provided");
    }
}

pub fn evaluate_subtraction(sub: &Expression, environment: &Environment) -> f64 {
    if let Expression::Subtract(expressions) = sub {
        let mut iter = expressions.iter();
        let first = iter.next().unwrap();
        iter.fold(evaluate(first, environment), |total, next| {
            total - evaluate(next, environment)
        })
    } else {
        panic!("Subtraction not provided");
    }
}

pub fn evaluate_division(div: &Expression, environment: &Environment) -> f64 {
    if let Expression::Divide(expressions) = div {
        let mut iter = expressions.iter();
        let first = iter.next().unwrap();
        iter.fold(evaluate(first, environment), |total, next| {
            total / evaluate(next, environment)
        })
    } else {
        panic!("Division not provided");
    }
}

fn evaluate(expression: &Expression, environment: &Environment) -> f64 {
    match expression {
        Expression::Add(_) => evaluate_addition(expression, environment),
        Expression::Multiply(_) => evaluate_multiplication(expression, environment),
        Expression::Subtract(_) => evaluate_subtraction(expression, environment),
        Expression::Divide(_) => evaluate_division(expression, environment),
        Expression::Variable(key) => {
            let expr = environment.value_for_key(key);
            evaluate(expr, environment)
        }
        Expression::Number(val) => *val,
    }
}

fn main() {
    let mut expressions = Vec::new();
    expressions.push(Expression::Number(3.0));
    expressions.push(Expression::Number(4.0));
    expressions.push(Expression::Number(5.0));
    let add = Expression::Add(expressions);
    let multiply = Expression::Multiply(vec![add, Expression::Number(2.0)]);
    let result = evaluate(&multiply, &Environment::new());
    println!("The result is {result}");
}

#[cfg(test)]
mod tests {
    use crate::{
        Environment, Expression, evaluate_addition, evaluate_division, evaluate_multiplication,
        evaluate_subtraction,
    };

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_basic_addition() {
        // arrange
        let values = vec![Expression::Number(2.0), Expression::Number(2.0)];

        // act
        let sum = evaluate_addition(&Expression::Add(values), &Environment::new());

        // assert
        assert_eq!(sum, 4.0);
    }

    #[test]
    fn test_basic_addition_not5() {
        // arrange
        let values = vec![Expression::Number(2.0), Expression::Number(2.0)];

        // act
        let sum = evaluate_addition(&Expression::Add(values), &Environment::new());

        // assert
        assert_ne!(sum, 5.0);
    }

    #[test]
    fn test_basic_subtraction() {
        // arrange
        let values = vec![Expression::Number(2.0), Expression::Number(2.0)];

        // act
        let difference = evaluate_subtraction(&Expression::Subtract(values), &Environment::new());

        // assert
        assert_eq!(difference, 0.0);
    }

    #[test]
    fn homework_test() {
        // arrange
        let values = vec![
            Expression::Number(3.0),
            Expression::Number(4.0),
            Expression::Number(5.0),
            Expression::Number(evaluate_addition(
                &Expression::Add(vec![Expression::Number(2.0), Expression::Number(2.0)]),
                &Environment::new(),
            )),
        ];

        // act
        let result = evaluate_multiplication(&Expression::Multiply(values), &Environment::new());

        // assert
        assert_eq!(result, 240.0);
    }

    #[test]
    fn test_new_environment() {
        // arrange

        // act
        let new_env = crate::Environment::new();

        // assert
        let expr = new_env.value;
        if let crate::Expression::Number(value) = expr {
            assert_eq!(value, 0.0);
        } else {
            assert_eq!(1, 0)
        }
    }

    #[test]
    fn test_value_for_key() {
        // arrange
        let mut new_env = crate::Environment::new();
        new_env.key = String::from("foo");
        new_env.value = crate::Expression::Number(2.0);

        // act
        let expr = new_env.value_for_key(&String::from("foo"));

        // assert
        if let crate::Expression::Number(value) = expr {
            assert_eq!(*value, 2.0);
        } else {
            assert_ne!(1, 1);
        }
    }

    #[test]
    fn test_addition_with_variable() {
        // arrange
        let mut new_env = crate::Environment {
            key: String::from("X"),
            value: crate::Expression::Number(6.0),
        };
        let vec = vec![
            crate::Expression::Number(7.0),
            crate::Expression::Variable(String::from("X")),
        ];
        let add = crate::Expression::Add(vec);

        // act
        let value = crate::evaluate(&add, &Environment::new());

        // assert
        assert_eq!(value, 7.0)
    }

    #[test]
    fn test_division() {
        // arrange
        let values = vec![Expression::Number(2.0), Expression::Number(2.0)];

        // act
        let dividend = evaluate_division(&Expression::Divide(values), &Environment::new());

        // assert
        assert_eq!(dividend, 1.0);
    }
}
