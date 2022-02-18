pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let ve = vec![1, 2, 3, 4, 5, 6];
    let i = largest(&ve);
    println!("{}",i);
}