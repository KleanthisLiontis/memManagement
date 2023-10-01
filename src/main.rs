fn main() {
    let a = 75;
    // let result = stack_only(a);
    // dbg!(result);
 
    println!("Small function test OG weight:{} and adjusted Mars weight:{}", a, you_on_mars(a));
}

fn stack_only(b: i32) -> i32 {
    let c = 3;
    return b + c + stack_and_heap();
}

fn stack_and_heap() -> i32 {
    let d = 5;
    let e = Box::new(7);
    return d + *e;
}

fn you_on_mars(_earth_weight: i32) -> f32 {
    return _earth_weight as f32/9.81 *3.711;
}
