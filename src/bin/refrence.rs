fn main() {
    let s1 = String::from("test");
    let len = calculate_len(&s1);
    println!("{}", s1);
    println!("{}", len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2);

    let s3 = String::from("hello world");
    let i = first_word_index(&s3);
    println!("{}", i);

    let s6 = String::from("hello world");
    let sub = first_word(&s6);
    println!("{}",sub);
}

fn first_word(str: &str) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }
    return &str[..];
}

fn first_word_index(str: &String) -> usize {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    str.len()
}

pub fn calculate_len(str: &String) -> usize {
    str.len()
}


pub fn change(str: &mut String) {
    str.push_str(",world")
}