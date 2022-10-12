fn main() {
let  arr_1 = [1,2,3,4,5];
for val in arr_1.iter(){
    println!("{}" , val);
}
let mut iter1 = arr_1.iter();
println!("1st : {:?}" , iter1.next());
}
