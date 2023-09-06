
//#![allow(unused_variables)]

use std::arch::x86_64::_mulx_u32;

use std::io::{ErrorKind, Error, Read};
use std::fs::File;

fn read_file(fileName:&str)->Result<String, Error> {
    let mut file_handler = match File::open(fileName) {
        Ok(f) => {
            Result::Ok(f)
        }
        Err(err) => {
            match err.kind() {
                 ErrorKind::NotFound => {
                    match File::create(fileName) {
                        Ok(f) => {
                            Result::Ok(f)
                        }
                        Err(err1) => {
                            Result::Err(err1)
                        }
                    }
                }
                _ => {
                    Result::Err(err)
                }
            }
        }
    };
    match &mut file_handler {
        Ok(file) => {
            let mut fileData = String::new();
            file.read_to_string(&mut fileData)?;
            Result::Ok(fileData)
        }
        Err(_) => {
            let result = Result::Err(Error::new(ErrorKind::NotFound, "not found the file"));
            result
        }
    }
    
}

fn main() {
    let unused:i32 = -1;
    let used:u32 = 1;
    let location:[f32;2] = [0.1, 0.2];
    let array:[i32; 10000] = [4; 10000];

    for x in array {
        print!("{}",x);
    }
    let name_with_location:(&str, f32, f32) = ("vikash", 0.2, 0.3);
    let(name, latitude, longitude) = name_with_location;
    println!("{},{},{}", name, latitude, longitude);
    println!("{}", name_with_location.0);
    println!("{}, {}", unused, used);
    println!("Hello, world!");
    println!("Vikash first rust programme");
    let nm:&str = "vcikash";
    let mut vnm:String = String::from("vikash kumar");
    let string_name_slice = &vnm;
    let string_name_slice2 = vnm.as_str();
    println!("{}, {}", string_name_slice, string_name_slice2);
   // vnm = String::from("vikash");
   // println!("{}, {}", string_name_slice, string_name_slice2);
    println!("{}-- {}", nm, vnm);

    let mut slogan = String::new();
    slogan.push_str("we hit the ground");
    slogan.push(' ');
    slogan = slogan + "every time";
    println!("{}",format!("{}", slogan));
    let nominator = 6.7;
    let deno : u32 = 3;
    let _deno_in_flot = deno as f32;
    let result = nominator / _deno_in_flot;
    let char_in_ascii =  char::from_u32(deno);
    println!("{}", char_in_ascii.unwrap());

    let scope_test = "scope test outer";

    {
        //let scope_test = 1;
        println!("{}", scope_test);
        let scope_test = "inner scope";
        println!("{}", scope_test);
    }
    println!("{}", scope_test);

    if scope_test == "scope test outer" {
        println!("Hello you come here!")
    }else if scope_test == "inner scope" {
        println!("you changed a lot")
    } else {
        println!("No body come here")
    }

    match scope_test {
        "scope test" => println!("{}", scope_test),
        _ => println!("nothing is found")
    }

    let letter = scope_test.chars().nth(4);
    
    let _ndb_frequency = 384;
    
    match _ndb_frequency {
        200..=500 => {
            println!("valid result")
        }
        _=> println!("invalid result")
    }

    match _ndb_frequency {
        ndb if _ndb_frequency >= 200 && _ndb_frequency < 500 => {
            println!("valid frequency {}", ndb)
        }
        _ => println!("invalid freq")
    }
    let rom = Fr::ROM(32);
    let vor = Fr::VOR(String::from("vor"), 43);
    let vor_rom = Fr::VORROM(String::from("vorrom"), 45);
    let random = Fr::RANDOM {name:String::from("vikash"), latitude:0.23, longitude:4.34};
    display_fr(&rom);
    display_fr(&vor);
    display_fr(&vor_rom);
    display_fr(&random);

    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            continue;
        }
        println!("counter is {}", counter);
        if counter == 10 {
            break;
        }
    }
    let numbers = [1,2,3,4,5,6,7,8,9,10];
    for num in numbers.iter() {
        println!("print number {}", num)
    }

    let mut original = String::from("original value");
    println!("outer original value {}", original);

    {
        let next = &mut original;
        *next = String::from("next value");

        println!("inner next value {}", next);
        println!("inner original value {}", original);
    }
    println!("outer original value {}", original);
    let a = 10;
    let b = 30;
    println!("print {}", explicit_lifetime(&a, &b));
    println!("print single {}", single_return(&a));

    let airline = "Duck Airline";

    let write_message = |slogan:String| -> String {
        String::from(format!("{}. {}", airline, slogan))
    };

    println!("{}",write_message("We always hit the ground.".parse().unwrap()));

    let fileName = "vikash.txt";
    let result = read_file(fileName);
    match result {
        Ok(data) => {
            println!("{}", data)
        }
        Err(err) => {
            println!("error {:#?}", err)
        }
    }
}


fn explicit_lifetime<'a>(p1:&'a i32, p2:&'a i32) -> &'a i32 {
    let result = if p1 < p2 {
        p2
    } else {
        p1
    };
    result
}

fn single_return(p1:&i32) -> &i32 {
    let result = p1;
    result
}

fn display_fr(fr:&Fr) {
    match fr {
        Fr::ROM(fr) => {
            println!("display {}", fr)
        }
        Fr::VOR(id, fr) => {
            println!("name {}, fr {}", id, fr)
        }
        Fr::VORROM(id, fr) => {
            println!("name {}, fr {}", id, fr)
        }
        Fr::RANDOM {name, latitude, longitude} => {
            println!("name {}, lat {}, long {}", name, latitude, longitude)
        }
    }
}
enum Fr {
    ROM(u32),
    VOR(String, u32),
    VORROM(String, u32),
    RANDOM{name:String, latitude:f32, longitude:f32},
}

trait Data<T, U> {
    fn getKey(key:T) -> U;
    fn insert(Key:T, Value:U);
}