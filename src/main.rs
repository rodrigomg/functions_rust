fn main() {
    another_function(90);
    mutiple_parameters(23,89.99);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn mutiple_parameters(a: i32, b: f32) {
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
}

/*fn statements_not_return_value(){
    let x = (let y = 6);
    println!("The value of x is: {}", x);
}*/
