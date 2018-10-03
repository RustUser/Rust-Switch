mod switch;

use switch::*;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point {
            x,
            y,
        };
    }
}

fn main() {
    let array: [i32; 4] = [5, 4, 3, 2];

    let mut output: Vec<Point> = Vec::new();

    let mut switch = switch::Switch::new();

    for i in 0..array.len() {
        if switch.get() {
            output.push(Point::new(array[i - 1], array[i]));
        }
    }

    println!("We took an input (a one dimensional array of integers) of: {:?}.", array);
    println!("And turned it into an array of points: {:?}.", output);
}