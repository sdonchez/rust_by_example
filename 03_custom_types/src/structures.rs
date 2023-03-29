// An attribute to hide warnings for unused code.
#![allow(dead_code)]

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
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// Function to compute the area of a Rectangle
fn rect_area(rect: Rectangle) -> f32
{
    let length: f32 = (rect.bottom_right.x - rect.top_left.x).abs();
    let width: f32 = (rect.bottom_right.y - rect.top_left.y).abs();

    let area: f32 = length * width;

    area
}

// Function to construct a square rectangle based on a point and a side length.
fn square(top_left: Point, side_length: f32) -> Rectangle
{
    let bottom_right = Point {x: top_left.x + side_length, 
        y: top_left.y + side_length};

    Rectangle{top_left, bottom_right}
}

fn main() {
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
    let bottom_right = Point { x: 5.2, y: 1.4 };

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

    let area: f32 = rect_area(_rectangle);
    println!("Rectangle Area: {}", area);

    let square_tl = Point { x: 1.5, y: 1.5 };
    let square_length = 5.5;

    let _square: Rectangle = square( square_tl, square_length );
    let sq_area: f32 = rect_area(_square);
    println!("Square Area: {}", sq_area);


    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}