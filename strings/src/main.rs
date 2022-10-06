fn main() {
let mut st1 = String::new();
st1.push('A');
st1.push_str(" gopal");
for gopal in st1.split_whitespace() {
    println!("{}" , gopal);
}
let st2 = st1.replace("A", "Another World");
println!("{}" , st2);

let st3 = String::from(" x s d e r g h b c c c d s ");
let mut v1:Vec<char> = st3.chars().collect();
v1.sort();
// duplicate string remove
v1.dedup();
for char in v1 {
    println!("{}" , char);
}
let st4 = "Hey Folks! I am Gopal Gurram";
let mut st5 = st4.to_string();
println!("{}" , st5);
// let bytes_array1 = st5.as_bytes();
let st6  = &st5[0..6];
println!("String length : {}" , st6.len());
st5.clear();

let st6 = String::from("Just some Random");
let st7 = String::from("Words");
let st8 = st6 + &st7;
// st7 is reference , it still exits but st6 will not exit anymore
println!("{}" , st8);
for char in st8.bytes() {
    println!("{}" , char);
}


}
