#[allow(dead_code)]

//unsigned integers : u8 , u16 . u32 , u64 , u128 , usize
//signed integers : i8 , i16 . i32 , i64 , i128 , isize
fn main() {
    println!(" Max u32 {} : ", u32::MAX);
    println!(" Max u64 {} : " ,u64::MAX);
    println!(" Max u128 {} : ", u128::MAX);
    println!(" Max usize {} : ", usize::MAX);
    println!(" Max f32 {} : " ,f32::MAX);
    println!(" Max f64 {} : ", f64::MAX);
    
    let num_1:f32 = 1.111111111111111;
    println!("f32 : {}" , num_1+0.111111111111111);
    
    let num_2:f64 = 1.111111111111111;
    println!("f64 : {}" , num_2+0.111111111111111);

    let num_3: u32= 4;
    let num_4: u32 = 5;
println!("4 + 5 = {}" , num_3 + num_4);
println!("4 - 5 = {}" , num_3 - num_4);
println!("4 * 5 = {}" , num_3 * num_4);
println!("4 / 5 = {}" , num_3 / num_4);
println!("4 % 5 = {}" , num_3 % num_4);

}
