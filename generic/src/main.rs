use std::ops::Add;


fn get_sum<T:Add<Output = T>>(x : T , y : T) -> T {
return  x+y;
}
fn main() {
    println!("5 + 6 = {}" ,get_sum(5,6 ));
    println!("5.5 + 6.5 = {}" ,get_sum(5.5,6.5 ));
}
