#[allow(unused_variables)]
// name struct
struct User {
    active:bool,
    username:String,
    sign_in_count:i32,
}

// tuple like struct
struct Coordinates(i32, i32, i32);

struct UnitStruct;

struct Square {
    width:i32,
    height:i32
}

impl Square {
    fn area(&self) -> i32 {
        self.width*self.height
    }
}

struct MyString<'a> {
    text: &'a str,
}

#[derive(Debug)]
struct Car {
    mpg:i8,
    color:String,
    top_speed:i16,
}

impl Car {
    fn set_mpg(&mut self, mpg:i8) {
        self.mpg = mpg
    }
    fn set_color(&mut self, color:String) {
        self.color = color
    }
    fn set_top_speed(&mut self, top_speed: i16) {
        self.top_speed = top_speed
    }
}

fn main() {
    let mut car = Car {
        mpg: 0,
        color: String::from("none"),
        top_speed: 0,
    };
    car.set_mpg(10);
    car.set_top_speed(50);
    car.set_color(String::from("Red"));
    println!("{:?}", car);
    let u1 = User { active: true, username: String::from("vikash"), sign_in_count: 0 };
    println!("{}", u1.username);
    let cord = Coordinates(1, 2, 3);
    // 1..5 .. is sure struct value range -> Range{Start: 1, end: 5}
    let sq = Square{width: 4, height: 5};
    println!("{}", sq.area());
    enum_patterns();
}

enum Pet {
    dog,
    cat,
    fish
}

impl Pet {
    fn what_i_am(self)-> &'static str {
        match self {
            Pet::dog => "I am dog",
            Pet::cat => "I am cat",
            Pet::fish => "I am fish"
        }
    }
}
fn enum_patterns() {
    let dog = Pet::dog;
    println!("{}", dog.what_i_am());
}
