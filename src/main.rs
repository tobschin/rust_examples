mod leberkas;
mod kasse;
fn main() {
    leberkas::topping(String::from("senf"));
    leberkas::topping(String::from("Ketchup"));
    leberkas::zahlen(12.23, 14.0);
}
