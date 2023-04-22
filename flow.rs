#![allow(unused_labels)]
fn main() {
    fizz_buzz();
    pick_up_sticks();
    the_outer_limits();
    bottles_of_beer_on_the_wall();
    let waves = while_the_waves_washed_the_shores();
    println!("Immutable waves {}", waves);
    to_be_or_not_to_be();
}

fn fizz_buzz() {
    for i in 1..101i32 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        };
    };
}

fn pick_up_sticks() {
    let mut i = 1i32;
    loop {
        if i < 3 {
            println!("{}", i);
            i += 1;
        } else {
            println!("Buckle my shoe");
            break;
        }
    }
}

fn the_outer_limits() {
    // Loops can be annotated.  The name must be passed to `break` and `continue`.
    'outer: loop {
        println!("Outer");
        'inner: loop {
            println!("Inner");
            break 'outer;
        }
    }
    println!("Done");
}

fn bottles_of_beer_on_the_wall() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 99 {
            // counter is assigned to result
            break counter;
        }
    };
    println!("{} bottles of beer on the wall", result);
}

fn while_the_waves_washed_the_shores() -> i32 {
    let mut wave = 0;
    while wave < 10 {
        if wave == 5 {
            return wave;
        }
        wave += 1;
    }
    wave
}

fn to_be_or_not_to_be() {
    let to_be = true;
    let result = match to_be {
        true => "To be",
        false => "Not to be"
    };
    println!("{}", result);
}