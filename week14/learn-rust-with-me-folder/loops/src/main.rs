fn main() {
    let mut counter = 1;

    let stopped_at = loop {
        counter *= 3;

        if counter > 100 {
            break counter;
        }
    };

    println!("The counter stopped at: {}", stopped_at);

    let mut num = 0;
    while num < 10 {
        println!("Hello there!");
        num += 1;
    }

    let groceries = ["Apple", "Banana", "Cherry", "Date", "Eggplant"];

    for item in groceries.iter() {
        println!("The item is: {}", item);
    }

    for number in 1..7 {
        println!("Current number: {}", number);
    }
}
