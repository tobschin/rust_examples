
use mockall::*;
use mockall::predicate::*;


#[automock]
trait Gebaeck {
    fn new(gewicht : u8, getreide : String) -> Self;
    fn backen(&mut self);
    fn describe(&self);
    fn gebacken(&self) -> bool;
    fn ist_es_fertig(&self) -> String;
}


struct Semmel {
    gewicht : u8,
    getreide : String,
    gebacken : bool,

}

impl Gebaeck for Semmel {
    fn new(gewicht:u8,getreide:String) -> Semmel {
        Semmel { gewicht: gewicht, getreide: getreide, gebacken: false }
    }

    fn backen(&mut self){
        self.gebacken=true;
    }

    fn gebacken(&self) -> bool {
        return self.gebacken;
    }

    fn ist_es_fertig(&self) -> String {
        if self.gebacken() {
            return String::from("ja")
        } else {
            return String::from("nein");
        }
    }

    fn describe(&self) {
        println!("{} und aus {:?}", self.gewicht, self.getreide);
    }
}

#[cfg(test)]
mod tests {
    use super::{Semmel, Gebaeck, MockGebaeck};


    #[test]
    fn test_new() {
        let semmel : Semmel = Semmel::new(100, String::from("Roggen"));
        semmel.describe()
    }

    // geht noch nicht
    #[test]
    fn test_mock_ist_es_fertig() {
        let mut mock: MockGebaeck = MockGebaeck::new(12, String::from("asdf"));
        mock.expect_gebacken().return_const(true);
        println!("{:?}", mock.ist_es_fertig());
        assert_eq!(mock.ist_es_fertig(), String::from("ja"));

    }
}
