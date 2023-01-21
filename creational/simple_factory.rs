trait Product {
    fn turn_on(&self);
}

enum Type {
    ProductA,
    ProductB,
}

struct ProductA;
impl Product for ProductA {
    fn turn_on(&self) {
        println!("ProductA turned on");
    }
}

struct ProductB;
impl Product for ProductB {
    fn turn_on(&self) {
        println!("ProductB turned on");
    }
}

struct ProductFactory {}

impl ProductFactory {
    fn get_instance(kind: &Type) -> Box<dyn Product> {
        match kind {
            Type::ProductA => Box::new(ProductA {}),
            Type::ProductB => Box::new(ProductB {}),
        }
    }
}

fn main() {
    let pa = ProductFactory::get_instance(&Type::ProductA);

    let pb = ProductFactory::get_instance(&Type::ProductB);

    pa.turn_on();
    pb.turn_on();
}
