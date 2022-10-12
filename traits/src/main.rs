fn main() {
const PI :f32 = 3.141592;

// let rec = Rectangle {
//     length : 20 , height : 20.25
// };
trait  Shape {
    fn new(length : f32 , height : f32) -> Self;
    fn area(&self) -> f32;
}
struct Rectangle {length : f32 , height:f32}
struct Circle {length : f32 , height:f32}
 
impl  Shape for Rectangle {
    fn new(length :f32 , height : f32) -> Rectangle {
        return Rectangle {length , height};
    }
    fn area(&self) -> f32 {
        return  self.length * self.height;
    }
}
impl  Shape for Circle {
    fn new(length :f32 , height : f32) -> Circle {
        return Circle {length , height};
    }
    fn area(&self) -> f32 {
        return  (self.length / 2.0) .powf(2.0) * PI;
    }
}

let rec = Rectangle { length:10.0 , height:10.0};
let circ = Circle { length:10.0 , height:10.0};

println!("Rect Area = {}" , rec.area());
println!("Circ Area = {}" , circ.area());
}
