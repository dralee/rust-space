/**
 * 自动化测试
 * 2024.03.29 by dralee
 */
pub mod tests; // 引入其他定义的test模块,会自动测试运行

#[cfg(test)]
mod test {
    
    #[test]
    fn it_works(){
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn same_str(){
        let s1 = String::from("hello");
        let s2 = "hello";
        assert_eq!(&s1, s2);
    }

    #[test]
    fn failure(){
        panic!("Make this test fail!");
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };
        assert!(smaller.can_hold(&larger));
    
    }

}