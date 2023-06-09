/*
    ❯ rustc struct_rectangle.rs && ./struct_rectangle
    Area: 12.0
    Square: Rectangle { top_left: Point { x: 1.0, y: 4.0 }, bottom_right: Point { x: 4.0, y: 1.0 } }
 */

// https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

/*
main() below commented out as this code is now called from `main.rs`
to learn/demonstrate how to use Modules `mod`
 */
// fn main() {
//     let rectangle = Rectangle {
//         top_left: Point { x: 1.0, y: 4.0 },
//         bottom_right: Point { x: 5.0, y: 1.0 },
//     };
//     let rect_area = rect_area(rectangle);
//     println!("Area: {0:?}", rect_area);

//     let square_top_left = Point { x: 1.0, y: 4.0 };
//     let _square = square(square_top_left);
//     println!("Square: {0:?}", _square);
// }

// Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
pub fn rect_area(rectangle: Rectangle) -> f32 {
    return (rectangle.top_left.y - rectangle.bottom_right.y)
        * (rectangle.bottom_right.x - rectangle.top_left.x);
}

// Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with
// its top left corner on the point, and a width and height corresponding to the f32.
pub fn square(point: Point) -> Rectangle {
    let square = Rectangle {
        top_left: point,
        bottom_right: Point {
            x: point.y,
            y: point.x,
        },
    };
    return square;
}
