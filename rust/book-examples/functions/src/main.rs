fn main() {
    let x: i32 = 5;
    five(x);
    println!("result: {x}");
}

fn five(num: i32) {
    let _ = num + 5;
}