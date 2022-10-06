fn main() {
get_sum(5, 45);
println!("{}" , get_sum_2(15, 25));
let (val1 , val2) = get_sum_3(5);
println!("Nums : {} {}" , val1 , val2);
let num_list = vec![1,2,3,4,54,7];
println!("Sum of list = {}" , sum_list(&num_list));
}
fn get_sum (x:i32 , y:i32) {
println!(" {} + {} = {}" , x , y , x+y);
}
fn get_sum_3(x:i32) -> (i32 , i32) {
    return (x+1 , x+3);
}
fn get_sum_2(x:i32 , y:i32) -> i32 {
    x + y
}

fn sum_list (list : &[i32]) -> i32 {
let mut sum = 0 ;
for &val in list.iter() {
    sum += &val;
}
sum
}