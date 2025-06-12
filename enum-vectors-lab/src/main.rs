#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle { a: f64, b: f64, c: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle { a, b, c } => {
                let p = (a + b + c) / 2.0;
                (p * (p - a) * (p - b) * (p - c)).sqrt()
            }
        }
    }
}

fn largest_area(shapes: Vec<Shape>) -> Shape {
    shapes
        .into_iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
        .unwrap()
}
//Формула Герона: S = √(p(p-a)(p-b)(p-c)), где p - полупериметр (p = (a+b+c)/2), а, b, c - стороны треугольника.
fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Square(3.0),
        Shape::Triangle {
            a: 5 as f64,
            b: 6 as f64,
            c: 7 as f64,
        },
    ];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle { a, b, c } => {
                let p = (a + b + c) / 2.0;
                (p * (p - a) * (p - b) * (p - c)).sqrt()
            }
        })
        .sum();

    println!("Total area: {} sq. units", total_area);
    let largest = largest_area(shapes);
    println!(
        "The Largest shape by area is: {:?} with area {:.2}",
        largest,
        largest.area()
    )
}
