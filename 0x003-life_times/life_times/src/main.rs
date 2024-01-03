fn main() {
    let some_int = 10;
    let additional_int = some_fun(&some_int);
    println!("{}", additional_int);
}


fn some_fun(i: &i32) -> &i32 {
    &i
}