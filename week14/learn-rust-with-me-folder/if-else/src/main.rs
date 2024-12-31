fn main() {
    if 1 == 2 {
        println!("the numbers are equal");
    } else {
        println!("the numbers are not equal");
    }

    let x = true;
    let truth = if x {
        "it is true"
    } else {
        "it is false"
    };

    println!("The statement states that {}", truth);

    let score = 50;
    let invalid_score: bool;
    
    if score < 0 {
        invalid_score = true;
    } else if score == 0 {
        invalid_score = true;
    } else if score > 100 {
        invalid_score = true;
    } else {
        invalid_score = false;
    }

    println!("The score is invalid: {}", invalid_score);
}