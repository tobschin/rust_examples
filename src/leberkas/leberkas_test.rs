    // example to write tests in outside file
    
    use super::*;

    #[test]
    #[should_panic]
    fn test_topping_should_panic() {
        topping(String::from("Ketchup"))
    }

    #[test]
    fn test_topping_should_go() {
        topping(String::from("Senf"));
    }

    #[test]
    fn test_zoin() {
        assert_eq!(zahlen(1.0, 2.0), 1.0);

    }
