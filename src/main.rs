mod leberkas;
mod kasse;
mod mocking_example;
fn main() {
    leberkas::topping(String::from("senf"));
    leberkas::topping(String::from("Ketchup"));
}
