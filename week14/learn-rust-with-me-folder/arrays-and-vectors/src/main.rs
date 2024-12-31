fn main() {
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let days_num = [0; 7];

    println!("Today is {}", days[0]);

    let test_scores = vec![100, 80, 90];
    println!("The test scores are: {:?}", test_scores);

    let zeroes = vec![0; 5];
    println!("The zeroes are: {:?}", zeroes);

    let mut names = Vec::new();
    names.push("Husain");
    names.push("Kaasyiful");
    names.push("Rayhan");

    println!("The names are: {:?}", names);

    names.pop();
    println!("The names are: {:?}", names);

    let mut fruits = vec!["Apple", "Banana", "Cherry"];
    let banana = fruits[1];
    fruits[0] = "Blueberry";
    println!("The second fruit of {:?} is: {}", fruits, banana);
}