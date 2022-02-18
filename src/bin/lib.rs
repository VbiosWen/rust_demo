mod person;
mod refrence;

fn main() {
    let person1 = person::Person::new("test");
    let mut str = String::from("test");
    refrence::change(&mut str);
    println!("{}", str);
    println!("{:#?}", person1);
}