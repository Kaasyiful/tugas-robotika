fn main() {
    use std::collections::HashMap;

    let mut objects: HashMap<String, String> = HashMap::new();

    objects.insert(String::from("Blue"), String::from("Sky"));
    objects.insert(String::from("Green"), String::from("Grass"));
    objects.insert(String::from("Yellow"), String::from("Sun"));

    let green_object = objects.get("Green");
    println!("The objects that is green is: {:?}", green_object);

    objects.remove("Yellow");

    println!("The objects that is yellow is: {:?}", objects.get("Yellow"));
}
