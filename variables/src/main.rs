fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.01414;
    let age = "25";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned!");
    age = age + 1 ; 

    println!("I am {} and I want {}" , age , ONE_MIL);
}
