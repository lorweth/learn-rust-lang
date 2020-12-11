use std::env;
use std::fs;

fn main() {
    // let filename: Vec<String> = env::args().collect(); // dọc chuỗi truyền vào phía sau cargo run

    // println!("{:?}", filename);

    let filename = "./poem.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
