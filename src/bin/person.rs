// #![crate_name = "doc"]
/// A human being is represented there
pub struct Person {
    name: String,
}

impl Person {
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }
    pub fn say_hello(&self) {
        println!("Hello {}", self.name);
    }
}

fn main() {
    let join = Person::new("test");
    join.say_hello();
}