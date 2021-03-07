//! Factory method creational design pattern allows
//! creating objects without having to specify the
//! exact type of the object that will be created.

trait Shape {
    fn draw(&self);
}

enum ShapeType {
    Rectangle,
    Circle,
}

struct Rectangle {}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("draw a rectangle!");
    }
}

struct Circle {}

impl Shape for Circle {
    fn draw(&self) {
        println!("draw a circle!");
    }
}

trait ProductFactory {
    fn create_product(s: &ShapeType) -> Box<dyn Shape>;
}

struct ShapeFactory;
impl ProductFactory for ShapeFactory {
    fn create_product(s: &ShapeType) -> Box<dyn Shape> {
        match s {
            ShapeType::Circle => Box::new(Circle {}),
            ShapeType::Rectangle => Box::new(Rectangle {}),
        }
    }
}

fn main() {
    let shape = ShapeFactory::create_product(&ShapeType::Circle);
    shape.draw(); // output: draw a circle!

    let shape = ShapeFactory::create_product(&ShapeType::Rectangle);
    shape.draw(); // output: draw a rectangle!
}
