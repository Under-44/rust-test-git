fn main() 
{
    let my_string = two_literals_to_string("Danniel", "Radcliffe");
    println!("{}", my_string);
}


///### adds two literals with format
///returns String type
fn two_literals_to_string(string1: &str, string2: &str) -> String
{
    format!("{} - {}", string1, string2)
}
