
trait Overview {
    fn overview(&self) -> String;
}

trait AnotherTrait {

}

struct Course {
    heading:String,
    author: String,
}

impl Drop for Course {
    fn drop(&mut self) {
        println!("Droping the course author {}", self.author);
    }
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.heading)
    }
}
fn main() {
    let c1 = Course{ heading: String::from("headline"), author: String::from("vikash")};
    println!("{}", c1.overview());
    call_overview(&c1);
    call_generic_overview(&c1);
    //drop(c1);
}

fn call_overview(item: &impl Overview) {
    println!("{}",item.overview())
}

fn call_generic_overview<T : Overview>(item: &T) {
    println!("{}", item.overview())
}
/*
fn call_multiple_overview(item1: &impl Overview, item2: &impl Overview) {

}

fn call_multiple_overview<T : Overview>(item1: &T, item2: &T) {

}


fn call_Single_overview(item1: &impl Overview + AnotherTrait) {

}
fn call_multiple_overview<T : Overview + AnotherTrait>(item1: &T, item2: &T) {

}

 */

