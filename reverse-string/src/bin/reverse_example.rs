use reverse_string::reverse;

fn main() {
    let s = String::from("hello");
    println!("{}", s);
    println!("{}", reverse(&s));
    
}