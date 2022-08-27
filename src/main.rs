// i'm testing with &str and String and
// using git to get familier with syntax
fn main()
{    
    //these are just some little tests
    println!("{}", two_literals_to_string("Danniel", "Radcliffe"));
    println!("{}", add_char_to_string(&mut String::from("Danniel "), 'G'));
    println!("{}", add_char_to_string(&mut "Jason ".to_owned(), 'T'));

    let mut testing: String = String::from("Hello, world");
    println!("{}", add_char_to_string(&mut testing, '!'));
}


///### adds two literals with `format!` macro
///```
/// return String
///```
/// 
fn two_literals_to_string(string1: &str, string2: &str) -> String
{
    format!("{} - {}", string1, string2)
}


///### appends `char` onto end of `String`
/// this function is useless, since you
/// can do this functionality in one line
///```
///return String 
///```
fn add_char_to_string(my_string: &mut String, my_char: char) -> String
{
    my_string.push(my_char);
    my_string.to_string()
}
