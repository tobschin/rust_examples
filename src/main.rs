mod leberkas;
mod kasse;
mod backerei;
fn main() {
    leberkas::topping(String::from("senf"));
    leberkas::topping(String::from("Ketchup"));
}
