struct Person {
    name: String,
    level: u32,
    attendance: bool
}

struct Grades (char, char, char, u32);

fn main() {
    println!("Hello, {} {}!", "Husain", "Kaasyiful");

    let mut age = 20;
    let birthyear = 2004;

    println!("I was {} years old", age);

    let birthyear = birthyear + 2;
    age = 22;

    println!("I am {} years old", age);
    println!("I was born in {}", birthyear);


    let brother_age: u32 = 18;
    println!("My brother is {} years old", brother_age);

    let float: f32 = 4.0;

    println!("1 * 2 = {}", 1 * 2);

    let is_bigger_num = 1 > 2;
    println!("Is 1 bigger than 2? {}", is_bigger_num);

    let first_letter = 'H';
    let last_letter = 'n';
    let name: &str = "Husain";

    println!("My name is {}, with {} as the first character and {} as the last character of may name", name, first_letter, last_letter);

    let prop_data = ("Toy1", 20, false);
    println!("The name is {}, the number is {}, and its status is {}", prop_data.0, prop_data.1, prop_data.2);

    let person1 = Person {
        name: String::from("Husain"),
        attendance: true,
        level: 4
    };

    println!("{}, is on level {}, and the attendance is {}", person1.name, person1.level, person1.attendance);

    let grades = Grades('A', 'A', 'A', 4);
    
    println!("The grades are {}, {}, {}, and the GPA is {}", grades.0, grades.1, grades.2, grades.3);
}
