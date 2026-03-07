#[derive(Clone, Copy)]
enum Primitive {
    Add,
    Subtract,
    Multiply,
    Number(i32),
}

fn evaluate(primitives: Vec<Primitive>) -> i32 {
    let first_element = &primitives[0];
    let mut iter = primitives.iter();
    iter.next();
    match first_element {
        Primitive::Add => iter.fold(0, |t, n| t + evaluate(vec![*n])),
        Primitive::Subtract => {
            iter.fold(evaluate(vec![primitives[1]]), |t, n| t - evaluate(vec![*n]))
                + evaluate(vec![primitives[1]])
        }
        Primitive::Multiply => iter.fold(1, |p, n| p * evaluate(vec![*n])),
        Primitive::Number(val) => *val,
    }
}

fn main() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Subtract);
    primitives.push(Primitive::Number(15));
    primitives.push(Primitive::Number(5));
    primitives.push(Primitive::Number(3));
    let result = evaluate(primitives);
    println!("The result is {result}!");
}
