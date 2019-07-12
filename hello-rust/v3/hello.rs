
fn main() {
	println!("{}",hello("Rust"))
}

fn hello(input: &str) -> String {
	let hello = "Hello, ".to_string() + &input;
    hello
}
