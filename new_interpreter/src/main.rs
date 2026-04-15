enum Expression {
    Integer(i32),
    FixedPoint(i32, i32),
    Addition(Vec<Expression>),
}

fn evaluate_add_integers(expressions: &Vec<Expression>) -> Expression {
    let mut total = 0;
    for each in expressions {
        if let Expression::Integer(value) = each {
            total = total + value;
        } else {
            panic!("Non-integer provided in the vector");
        }
    }
    Expression::Integer(total)
}

fn add_fixed_point(expressions: &Vec<Expression>) -> Expression {
    let mut total_whole = 0;
    let mut total_frac = 0;
    for each in expressions {
        if let Expression::FixedPoint(whole, frac) = each {
            total_whole = total_whole + whole;
            total_frac = total_frac + frac;
        } else {
            panic!("Non-fixed point provided in this vector");
        }
    }
    Expression::FixedPoint(total_whole, total_frac)
}

fn evaluate_addition(expression: &Expression) -> Expression {
    if let Expression::Addition(expressions) = expression {
        match expressions[0] {
            Expression::Integer(_) => evaluate_add_integers(expressions),
            Expression::FixedPoint(_, _) => add_fixed_point(expressions),
            _ => panic!("Input not allowed...."),
        }
    } else {
        panic!("Not addition!");
    }
}

fn evaluate_integer(expression: &Expression) -> f64 {
    if let Expression::Integer(value) = expression {
        *value as f64
    } else {
        panic!("Expected an integer, got non-integer type!");
    }
}

fn evaluate_fixed_point(expression: &Expression) -> f64 {
    if let Expression::FixedPoint(whole, frac) = expression {
        return (*whole as f64) + ((*frac as f64) / 100.0);
    } else {
        panic!("Oh no, it's no fixed point!")
    }
}

fn evaluate(expression: &Expression) -> f64 {
    match expression {
        Expression::Addition(_) => evaluate(&evaluate_addition(expression)),
        Expression::Integer(_) => evaluate_integer(expression),
        Expression::FixedPoint(_, _) => evaluate_fixed_point(expression),
    }
}

fn main() {
    println!("Main function")
}

#[cfg(test)]
mod test {

    #[test]
    fn test_anything_works() {
        assert!(true, "It worked");
    }

    #[test]
    fn test_simple_addition() {
        // arrange
        let expr = crate::Expression::Addition(vec![
            crate::Expression::Integer(2),
            crate::Expression::Integer(2),
        ]);
        // act
        let val = crate::evaluate(&expr);
        // assert
        assert_eq!(val, 4.0, "Expressions not equal");
    }

    #[test]
    fn test_fixed_point_addition() {
        // arrange
        let expr = crate::Expression::Addition(vec![
            crate::Expression::FixedPoint(2, 50),
            crate::Expression::FixedPoint(2, 50),
        ]);
        // act
        let val = crate::evaluate(&expr);
        // assert
        assert_eq!(val, 5.0, "Expressions not equal");
    }
}
