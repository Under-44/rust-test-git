fn main() 
{
    println!("{}", two_literals_to_string("Danniel", "Radcliffe"));
    println!("{}", add_char_to_string(String::from("Danniel "), 'G'));
  
    let testing = String::from("Hello, world");
    println!("{}", add_char_to_string(testing, '!'));
}


///### adds two literals with format
///```
/// return String
///```
/// 
fn two_literals_to_string(string1: &str, string2: &str) -> String
{
    format!("{} - {}", string1, string2)
}


///### adds a char onto end of String
///```
///return String 
///```
fn add_char_to_string(mut my_string: String, my_char: char) -> String
{
    my_string.push(my_char);
    my_string   
}
