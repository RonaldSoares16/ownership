fn main() {
    let x = String::from("Hello");
    println!("{}", x);
    println!("{}", change(&x));
}

fn change(y:&String) -> String{
    let mut z = y.clone();
    z.push_str(" world");
    z
}
