use std::cmp::PartialOrd;

fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    let result = largest(&nums);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'c', 'a'];
    let result = largest(&chars);
    println!("The largest char is {}", result);

    let strs = vec![String::from("hello"), String::from("world")];
    println!("strs[0] = {}", strs[0]);
    // let result = largest(&strs);
    // println!("The largest str is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
