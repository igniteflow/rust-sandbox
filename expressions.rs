// prevents a "path statement with no effect" warning being raised for line `y;`
#![allow(path_statements)]

fn main() {
    let x = {
        // returns x because it is not followed by a semi-colon
        let x = 2 * 5;
        x
    };

    let y = {
        // returns () and not y because y is followed by a semi-colon
        let y = 3 * 4;

        y;
    };

    println!("x {:?} y {:?}", x, y);
}
