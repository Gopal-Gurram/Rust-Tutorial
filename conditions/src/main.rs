use std::cmp::Ordering;

fn main() {
let age = 26;
if(age >= 18) && (age <=25) {
    println!("Not good Age to get married!");
} else if (age >=25) || (age <=30) {
    println!("Perfect Age to get married!");
} else {
    println!("Better to not get married!")
}
 // Ternary operator
let  my_vote_age = 17 ;
let can_vote = if my_vote_age >=18  {
    true
} else {
    false
};
println!("you are eligible to vote: {} " , can_vote);

// match

let age2 = 15 ;

match  age2 {
    1..=18 =>  println!("This is the best age to enjoy life"),
    19 | 25 => println!("This is the very important age to build your career."),
    26..=30 => println!("This is the age to get married"),
    _ => print!("You have to enjoy the family life!")
};

let my_age = 18 ;
let voting_age = 18 ;
match my_age.cmp(&voting_age) {
    Ordering::Less => println!("Can't Vote"),
    Ordering::Greater => println!("Can Vote"),
    Ordering::Equal => println!("Gained the opertunity to vote!."),
}
}
