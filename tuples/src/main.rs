fn main() {
let my_tuple:(u32 , String , f64) = (25 , "Gopal".to_string(), 1000.00);
println!("Name : {}" , my_tuple.1);
let ( v1 , v2, v3) = my_tuple;
println!("AGE : {}" , v1);
println!("Money : {}" , v3);
println!("Name : {}" , v2);


}
