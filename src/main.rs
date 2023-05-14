trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

trait Printable {
    fn print_info(&self);
}

impl<T: Shape> Printable for T {
    fn print_info(&self) {
        println!(
            "Shape information: area = {}, perimeter = {}",
            self.area(),
            self.perimeter()
        );
    }
}

#[derive(Debug)]
struct Rectangle {
    height: f64,
    width: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
    fn perimeter(&self) -> f64 {
        self.radius * std::f64::consts::PI * 2.0
    }
}

fn main() {
    let rectangle = Rectangle {
        height: 10.0,
        width: 20.0,
    };
    let circle = Circle { radius: 10.0 };

    rectangle.print_info();
    circle.print_info();

    let shape = get_largest_area(rectangle, circle);
    println!("The largest area is {:#?}", shape.area());
}

fn get_largest_area(shape1: impl Shape + 'static, shape2: impl Shape + 'static) -> Box<dyn Shape> {
    if shape1.area() > shape2.area() {
        Box::new(shape1)
    } else {
        Box::new(shape2)
    }
}
