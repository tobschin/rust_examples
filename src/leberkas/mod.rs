// example to write tests in outside file


use crate::kasse;

pub fn topping(mit : String) {
    if mit == String::from("Senf") {
        print!("Guadn");
    } else {
        panic!("Spinnst?");
    }
}

pub fn zahlen(preis : f32, zurueck : f32) -> f32 {
    return kasse::zahlen(preis, zurueck);
}

#[cfg(test)]
mod leberkas_test;