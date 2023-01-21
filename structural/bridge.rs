trait Implementor {
    fn decorate(&self, msg: String) -> String;
}

struct ParenImpl;
impl Implementor for ParenImpl {
    fn decorate(&self, msg: String) -> String {
        format!("({msg})")
    }
}

struct BracketImpl;
impl Implementor for BracketImpl {
    fn decorate(&self, msg: String) -> String {
        format!("{{{msg}}}")
    }
}

struct Abstraction<'a> {
    implementer: &'a dyn Implementor,
}
impl<'a> Abstraction<'a> {
    fn new(i: &dyn Implementor) -> Abstraction {
        Abstraction { implementer: i }
    }

    fn convert(&self, msg: String) -> String {
        self.implementer.decorate(msg)
    }
}

struct RefinedAbstraction<'a> {
    abstraction: Abstraction<'a>,
}
impl<'a> RefinedAbstraction<'a> {
    fn new(i: &dyn Implementor) -> RefinedAbstraction {
        RefinedAbstraction {
            abstraction: Abstraction::new(i),
        }
    }

    fn convert(&self, msg: String) -> String {
        self.abstraction.convert(msg)
    }

    fn print_convert_msg(&self, msg: String) {
        println!("{}", self.abstraction.convert(msg));
    }
}

fn main() {
    let abst_p = RefinedAbstraction::new(&ParenImpl);
    let abst_b = RefinedAbstraction::new(&BracketImpl);

    println!("{}", abst_p.convert("YOYO".to_string()));
    abst_b.print_convert_msg("oops".to_string());
}
