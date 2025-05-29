use std::collections::HashMap;

enum SpreadSheet {
    Int(i32),
    Float(f32),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadSheet::Int(3),
        SpreadSheet::Float(3.14),
        SpreadSheet::Text(String::from("some text")),
    ];

    for i in &row {
        match i {
            SpreadSheet::Int(val) => println!("int: {val}"),
            SpreadSheet::Float(val) => println!("float: {val}"),
            SpreadSheet::Text(val)=> println!("text: {val}"),           
        };
    }


    let row2 = Vec::from([1, 2, 3, 4, 5]);
    let i = &row2[3];
    println!("i = {i}");

    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s = {s}");

    let ss = String::from("hello");
    for c in ss.bytes() {
        println!("c = {c}");
    }

    for c in ss.chars() {
        println!("c = {c}");
    }

    let ss = String::from("नमस्ते");
    for c in ss.bytes() {
        println!("c = {c}");
    }

    for c in ss.chars() {
        println!("c = {c}");
    }

    let mut m = HashMap::new();
    m.insert(String::from("Blue"), 10);
    m.insert(String::from("Yellow"), 50);

    println!("blue = {}", m["Blue"]); 
    println!("yellow = {}", m["Yellow"]); 


    for (key, value) in &m {
        println!("{key}, {value}");
    }

    m.entry(String::from("Blue")).or_insert(50);

}