
fn main() {
    let s = String::from("hello");

    takes_ownership(&s);

    println!("{}", s);

    println!("{}", &s[1..]);

}


fn takes_ownership(some_string: &String) { // some_string comes into scope
    println!("{}", some_string);
}