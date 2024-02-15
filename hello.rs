fn main() {
    let hello: String = "Hello".to_owned(); // typed declaration using .to_owned()
    let world = String::from("World"); // implicit declaration and using String::from
    println!("{}", format!("{} {}!", hello, world));
}