use std::{cmp::Ordering};

fn largest<T: Ord>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Eq, PartialEq)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Ord for Point<T> where
    T: Ord + Eq + Copy
    {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}
impl<T: Ord + Eq + Copy> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> std::fmt::Display for Point<T> where
    T: std::fmt::Display
    {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait Swap {
    fn swap(&mut self) {
        println!("Please implement swap function");
    }
}
impl<T> Swap for Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}

fn swap(v: &mut impl Swap) {
    v.swap();
}

fn main() {
    let nums = vec![1, 5, 4, 10, 20, 3];
    let largest_num = largest(&nums);
    println!("Largest num is {largest_num}");

    let mut points = vec![
        Point { x: 3, y: 10},
        Point { x: 5, y: 9 },
        Point { x: 3, y: 5},
        Point { x: 5, y: 3 },
    ];
    let largest_point = largest(&points);
    println!("Largest point is {largest_point}");

    for point in &mut points {
        point.swap();
        swap(point);
        point.swap();
    }
    let largest_point = largest(&points);
    println!("Largest point is {largest_point}");

    let weird_points = vec![
        Point { x: Point { x: 3, y: 5 }, y: Point { x: 10, y: 6 } }
    ];
    // Below intentionally doesn't work as doesn't fulfill ord constraints
    // let weird_point = largest(&weird_points);
    // println!("Largest weird point is {weird_point}");
}
