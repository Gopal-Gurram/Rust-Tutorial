fn main() {
enum Day {
    Monday,
    Tuesday,
    Wendsday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}
impl  Day {
    fn is_weekend(&self) -> bool {
        match self {
  Day::Saturday | Day::Sunday => true,
        _ => false
        }       
    }
}

let today:Day = Day::Monday;
match today {
    Day::Monday => println!("I love Monday"),
    Day::Tuesday => println!("Biryani Day"),
    Day::Wendsday=> println!("Burger Day"),
    Day::Thursday => println!("Pizza Day"),
    Day::Friday => println!("Almost Weekend"),
    Day::Saturday => println!("Weekend"),
    Day::Sunday => println!("Weekend"),
}
println!("Is today the weekend {}" , today.is_weekend());
}
