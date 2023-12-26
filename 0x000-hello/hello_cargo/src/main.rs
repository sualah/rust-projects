fn main() {
    let  x:i32 = 64;
    println!("Hello, world! {}", x);
    println!("my value {}", x * x);

    let new_str:&str = "Hello new string";

    println!("{}", new_str);

    let str_t1: String = String::from("new str");

    println!("string: {} has {} capacity and  {} length and is empty: {} ", str_t1, str_t1.capacity(), str_t1.len(), str_t1.is_empty());

    let str_t2 : String = str_t1.to_string() + " with addition.";

    println!("string: {} has {} capacity and  {} length and is empty: {} ", str_t2, str_t2.capacity(), str_t2.len(), str_t2.is_empty());

    let concat_str:String = format!(" {} {} ", str_t1, str_t2);
    println!("{}", concat_str.trim());

    /*
     --------------- Tuples ------------------
     ------------------------------------------
     ------------------------------------------
     */

     let my_information : (&str, i32) = ("Salary ", 40000);
     println!("I have a {} of ${} ", my_information.0, my_information.1);

     let (my_salary, salary_value) = my_information;
     println!("{} : {}", my_salary, salary_value);

     /*
     --------------- Arrays ------------------
     ------------------------------------------
     ------------------------------------------
     */

    let mut number_array:[i32;6] = [4,5,6,7,8,9];
    println!("{}", number_array[0]);
    println!("{:?}", number_array);
    
    number_array[4] = 3;
    println!("{:?}", number_array);

    let array_of_zeroes : [i32;10] = [0;10];
    println!("{:?}", array_of_zeroes);

    let  susbset_of_num_of_arrays:&[i32] = &number_array[0..3];

    println!("{:?}", susbset_of_num_of_arrays);

   let check_index:Option<&i32> =  number_array.get(1);
   println!("{:?}", check_index);

    /*
     --------------- Vectors ------------------
     ------------------------------------------
     ------------------------------------------
     */

    let mut number_vector:Vec<i32> = vec![4,5,6,7,8,9];
    println!("{}", number_vector[0]);
    println!("{:?}", number_vector);

    number_vector.push(10);
    number_vector.push(20);
    
    number_vector.remove(4);

    println!("{:?}", number_vector);

    println!("The value 10 exist in the vector: {}", number_vector.contains(&10));


     /*
     --------------- Functions ------------------
     ------------------------------------------
     ------------------------------------------
     */

    myfunc("Salifu", 6000);
    let multiplication = multiply_nums(5, 6);
    println!("{}", multiplication);
    let (multiplication1, addition, subtraction) = operations(5, 6);
    println!("multiplication: {} addition: {} subtraction: {}", multiplication1, addition, subtraction);

    let full_name:String = {
        let f_name = "Salifu";
        let l_name = "Abdullai";
        format!("{} {}", f_name, l_name)
    };

    println!("My full name is {}", full_name);


      /*
     --------------- inputs ------------------
     ------------------------------------------
     ------------------------------------------
     */

    let mut n: String = String::new();

    std::io::stdin().read_line(&mut n).expect("Failed to read input");

    let n:f64 = n.trim().parse().expect("Invalid input");

    println!("{:?}", n);
}


fn myfunc(name:&str, salary:i32) {
    println!("My name is {} and my salary is: {}", name, salary);
}

fn multiply_nums(num1:i32, num2:i32) -> i32 {
    return num1 * num2
}

fn operations(num1:i32, num2:i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num2 - num1)
}