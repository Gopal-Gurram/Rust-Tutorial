
//Stack : Stores a values in last in first out format
// Data on the stack must have a defined fixed size

// Heap : When putting data on the heap you 
// request a certain amount of space.
// The OS finds space availabe and returns 
// an address for that space called a pointer.
 
// RULES
  // 1. Each value has a variable that's called it's owner.
  // 2. There is only one owner at a time.
  // 3. When the owner goes out of scope the value disappears.
fn main() {

    let mut str1 = String::from("Gopal , Nice to meet You!");
    // let str3 = print_return_string(str1);
    // println!("{}" , str3);
    change_string(&mut str1);
    
    
    

}
fn print_string(x:String) {
    println!("A String {}" , x);
}
fn print_return_string(x:String) -> String {
    println!("A String {}" , x);
    x
}
fn change_string(name : &mut String) {
    name.push_str("is Happy?");
    println!("Message : {}" , name);
}

