use std::env::VarError;
use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(3);
    v.push(4);
    v.push(6);
    let third = v[2];
    let mut third1: i32 = v[2];
    third1 += 100;
    println!("{}", third1);

    match v.get(2) {
        Some(third1) => println!("{}", *third1),
        _ => println!("no third")
    }

    for i in v {
        println!("{}", i)
    }

    let mut v1 = vec![100, 32, 57];

    for i in &mut v1 {
        *i += 50;
    }
    println!("{:#?}",v1);

    let mut most_scores: HashMap<String,i32> = HashMap::new();
    most_scores.insert(String::from("hello"),10);
    most_scores.insert(String::from("world"),20);

    let teams = vec![String::from("lamer"),String::from("wandashan")];
    let initial_scores = vec![10,20];
    let scoresMap:HashMap<_,_> = teams.iter().zip(initial_scores).collect();
    
}