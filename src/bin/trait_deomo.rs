pub trait summary {
    fn summarize(&self) -> String;
}

pub struct NewsAtricle {
    headLine: String,
    location: String,
    author: String,
    content: String,
}

impl NewsAtricle {
    fn new() -> NewsAtricle {
        NewsAtricle {
            headLine: String::from("hah"),
            location: String::from("wakaka"),
            author: String::from("wangbagaozi"),
            content: String::from("hah"),
        }
    }

}

impl summary for NewsAtricle {
    fn summarize(&self) -> String {
        format!("{},by{},({})", self.headLine, self.location, self.author)
    }
}

fn main() {
    let atricle = NewsAtricle::new();
    let s = atricle.summarize();
    print!("{}", s);
}