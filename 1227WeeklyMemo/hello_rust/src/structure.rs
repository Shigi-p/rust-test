pub fn structure() {
    use std::fmt;

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // A unit struct
    struct Unit;

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    #[allow(dead_code)]
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: Point,
        bottom_right: Point,
    }

    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // ----- activity
    impl fmt::Display for Rectangle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "top_left is {{ point.x is {}, point.y is {} }}\n", self.top_left.x, self.top_left.y)?;
            write!(f, "bottom_right is {{ point.x is {}, point.y is {} }}", self.bottom_right.x, self.bottom_right.y)
        }
    }
    fn rect_area(rectangle: Rectangle) -> f32 {
        let rect_horizontal = rectangle.bottom_right.x - rectangle.top_left.x;
        let rect_vertical = rectangle.top_left.y - rectangle.bottom_right.y;

        // println!("rect_horizontal is {:?}", rect_horizontal);
        // println!("rect_vertical is {:?}", rect_vertical);

        rect_horizontal * rect_vertical
    }

    let _new_rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: 3.0, y: 13.5 },
        bottom_right: Point {x: 5.2, y: 10.8},
    };

    // println!("_new_rectangle.top_left is {:?}", _new_rectangle.top_left);
    // println!("_new_rectangle.bottom_right is {:?}", _new_rectangle.bottom_right);
    
    println!("rect_area(_new_rectangle) is {}", rect_area(_new_rectangle));

    // -------------------

    fn square(from_point: Point, length: f32) ->  Rectangle{
        let top_left = Point{ x: from_point.x, y: from_point.y + length};
        let bottom_right = Point{ x: from_point.x + length, y: from_point.y};

        Rectangle{top_left: top_left, bottom_right: bottom_right}
    }

    let _new_point = Point{ x: 0.0, y: 0.0 };
    let _new_square = square(_new_point, 10.0);
    println!("_new_square.top_left is {:?}", _new_square.top_left);
    println!("_new_square.bottom_right is {:?}", _new_square.bottom_right);
    println!("rect_area(_new_square) is {:?}", rect_area(_new_square));
    // activity -----
}