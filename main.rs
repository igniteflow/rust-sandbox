/*
❯ rustc main.rs && ./main
Area: 12.0
Square: Rectangle { top_left: Point { x: 1.0, y: 4.0 }, bottom_right: Point { x: 4.0, y: 1.0 } }
 */

mod struct_rectangle;


fn main() {
    let rectangle = struct_rectangle::Rectangle {
        top_left: struct_rectangle::Point { x: 1.0, y: 4.0 },
        bottom_right: struct_rectangle::Point { x: 5.0, y: 1.0 },
    };
    let rect_area = struct_rectangle::rect_area(rectangle);
    println!("Area: {0:?}", rect_area);

    let square_top_left = struct_rectangle::Point { x: 1.0, y: 4.0 };
    let _square = struct_rectangle::square(square_top_left);
    println!("Square: {0:?}", _square);
}
