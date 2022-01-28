use std::convert::From;

pub fn from_and_into_test() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    println!("my_str is {}", my_str);
    println!("my_string is {}", my_string);

    println!("{:->10}", '-');

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30i32);

    println!("My number is {:?}", num);
    println!("{:->10}", '-');

    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
