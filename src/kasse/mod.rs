
pub fn zahlen(preis : f32, gegeben : f32) -> f32 {
    let rueckgeld = gegeben - preis;
    if rueckgeld < 0.0 {
        panic!("Host koa Puiva ned?")
    }
    return rueckgeld;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_zahlen_should_panic() {
        zahlen(39.99, 1.30);
    }

    #[test]
    fn test_zahlen() {
        assert_eq!(1.0, zahlen(2.0, 3.0))
    }

}