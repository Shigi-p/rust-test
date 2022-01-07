pub fn enums_test() {
    println!("This is function for enums test.");

    enum WebEvent {
        // An `enum` may either be `unit-like`,
        // `enum`要素型はユニット風でもよい
        PageLoad,
        PageUnload,
        // like tuple structs,
        // タプル風でもよい
        KeyPress(char),
        Paste(String),
        // or c-like structures.
        // C言語スタイルの構造体風でもよい
        Click { x: i64, y: i64 },
    }

    // A function which takes a `WebEvent` enum as an argument and
    // returns nothing.
    // 引数として`WebEvent`列挙型をとり、何も返さない関数
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // Destructure `c` from inside the `enum`.
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // Destructure `Click` into `x` and `y`.
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    // `to_owned()`は文字列スライスから所有権のある`String`を作成する
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

pub fn use_test() {
    // An attribute to hide warnings for unused code.
    #![allow(dead_code)]
    enum Status {
        Rich,
        Poor,
    }
    enum Work {
        Civilian,
        Soldier,
    }
    // Explicitly `use` each name so they are available without
    // manual scoping.
    // use crate::Status::{Poor, Rich};
    use Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    // use crate::Work::*;
    use Work::*;

    // Equivalent to `Status::Poor`.
    let status = Rich;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

pub fn c_like_test() {
    // An attribute to hide warnings for unused code.
    #![allow(dead_code)]

    // enum with implicit discriminator (starts at 0)
    enum Number {
        Zero,
        One,
        Two,
    }

    // enum with explicit discriminator
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("one is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

pub fn testcase_linked_list_test() {
    use List::*;

    // enum List {
    //     // Cons: Tuple struct that wraps an element and a pointer to the next node
    //     Cons(u32, Box<List>),
    //     // Nil: A node that signifies the end of the linked list
    //     Nil,
    // }

    // charを持つようなlistを作るテスト
    enum List {
        // Cons: Tuple struct that wraps an element and a pointer to the next node
        Cons(char, Box<List>),
        // Nil: A node that signifies the end of the linked list
        Nil,
    }

    // Methods can be attached to an enum
    impl List {
        // Create an empty list
        fn new() -> List {
            // `Nil` has type `List`
            Nil
        }
        // Consume a list, and return the same list with a new element at its front
        // fn prepend(self, elem: u32) -> List {
        //     // `Cons` also has type List
        //     Cons(elem, Box::new(self))
        // }

        // charを持つようなlistを作るテスト
        fn prepend_str(self, elem: char) -> List {
            // `Cons` also has type List
            Cons(elem, Box::new(self))
        }

        // Return the length of the list
        fn len(&self) -> u32 {
            // `self` has to be matched, because the behavior of this method
            // depends on the variant of `self`
            // `self` has type `&List`, and `*self` has type `List`, matching on a
            // concrete type `T` is preferred over a match on a reference `&T`
            // after Rust 2018 you can use self here and tail (with no ref) below as well,
            // rust will infer &s and ref tail.
            // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
            // このメソッドは、`self`の状態によって振る舞いが
            // 変化するため、matchをする必要がある。
            // `self`の型は`&List`であるので、`*self`は`List`になる。マッチングは
            // リファレンス(`&T`)ではなく実体(`T`)に対して行うのが好ましい。
            match *self {
                // Can't take ownership of the tail, because `self` is borrowed;
                // instead take a reference to the tail
                // `self`をすでに借用しているので、tailの所有権を取ることができない。
                // 代わりに参照を使用する。
                Cons(_, ref tail) => 1 + tail.len(),
                // Base Case: An empty list has zero length
                // 空リストならば長さは0
                Nil => 0,
            }
        }

        // Return representation of the list as a (heap allocated) string
        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    // `format!` is similar to `print!`, but returns a heap
                    // allocated string instead of printing to the console
                    format!("{}, {}", head, tail.stringify())
                }
                Nil => {
                    format!("Nil")
                }
            }
        }
    }

    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    // list = list.prepend(1);
    // list = list.prepend(2);
    // list = list.prepend(3);

    //charを持つようなlistを作るテスト
    list = list.prepend_str('a');
    list = list.prepend_str('b');
    list = list.prepend_str('c');

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
