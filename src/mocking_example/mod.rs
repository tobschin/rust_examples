use mockall::*

trait MyTrait {
    fn foo(&self) -> i32;
    fn bar(&self, x: i32) -> i32;
}

struct MyStruct {
    basenum: i32
}

#[automock]
impl MyTrait for MyStruct {
    

    fn bar(&self, x: i32) -> i32 {
        self.basenum + x
    }

    fn foo(&self) -> i32 {
        self.basenum
    }
}

fn function_to_test(myTrait : &dyn MyTrait) -> i32 {
    myTrait.foo()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let mut mock = MockMyStruct::new();
        mock.expect_foo()
            .return_const(3);
        assert_eq!(mock.foo(), 3)
        
    }

    #[test]
    fn test_blu() {
        let mut mock: MockMyStruct = MockMyStruct::new();
        mock.expect_foo()
            .return_const(3);
        assert_eq!(function_to_test(&mock), 3)
    }

}

