use hello::hello;

fn main() {
    let str = String::from(hello::greetings_from_rust());
    println!("{}", str);
}
