fn main() {
    println!("Hello, world!");
    let x = vec![1, 2, 3];
    let y = x.clone(); // deep copy of the vector into the new variable.
    // copy
    let a = 1;
    let b = a;
    println!("{}, {}", a, b);
    println!("{:?}", x);

    let mut str1:String = "given".to_string();
    let mut str2:String;
    change_string(&mut str1);
    println!("{}", str1);
    let mut vec = vec![1, 2, 3];
    println!("{:?}", vec);
    check_val(&vec);
    vec.push(15);
    println!("{:?}", vec);
    let mut value = 2;
    add_two(value);
    println!("{}", value);
}

fn change_string(str: &mut String) {
    str.push_str(", world");
}

fn check_val(val: &Vec<i8>) -> bool {
    if val[0] == 1 {
        true
    } else {
        false
    }
}

fn add_two(val: i8) {
    val + 2;
}
