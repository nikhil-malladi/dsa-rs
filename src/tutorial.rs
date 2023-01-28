use std::io;
use std::cmp::Ordering;
use std::ops::Add;

fn main(){
    // println!("Hello World!");
    // println!("How are you");
    // let mut name: String = String::new();
    // let greeting: &str = "Nice to Meet you";
    // io::stdin().read_line(&mut name)
    //     .expect("Didn't Receive Input");
    // println!("Hello {}, {}",name.trim_end(),greeting);

    // println!("MAX u32: {}",u32::MAX);
    // println!("MAX u64: {}",u64::MAX);
    // println!("MAX f32: {}",f32::MAX);
    // println!("MAX f64: {}",f64::MAX);
    // let num: i32 = rand::thread_rng().gen_range(1..100);
    // println!("num is {}",num);


    // let age: i32 = 88;
    // println!("Age: {}",age);

    // if (age>0) && (age<=18) {
    //     println!("Teenage")
    // }
    // else if (age>18) && (age<65) {
    //     println!("Working Age")
    // }
    // else {
    //     println!("Divine Age")
    // }

    // let mut inp: String = String::new();
    // io::stdin().read_line(&mut inp);
    // let age = inp.trim().parse::<i32>().unwrap();
    // const VOTING_AGE: i32 = 18;
    // let can_vote: bool = if age>18 {
    //     true
    // }
    // else {
    //     false
    // };
    // let votinge_age:i32 = 18;
    // println!("Can Vote : {}",can_vote);
    // match age {
    //     1..=18 => println!("Youth"),
    //     21 | 50 => println!("Suttle Age"),
    //     18..=100 => println!("Working"),
    //     _ => println!("Age Limit Exceeded")
    // };
    // match age.cmp(&VOTING_AGE){
    //     Ordering::Less => println!("Not eligible to vote"),
    //     Ordering::Greater => println!("congrats, eligible to vote"),
    //     Ordering::Equal => println!("RIght age to Vote")
    // };

    // // Loops
    // let arr_1: [i32;9] = [1,2,3,4,5,6,7,8,9];
    // let mut idx: usize = 0;
    // loop {
    //     if arr_1[idx]%2==0{
    //         idx+=1;
    //         continue;
    //     }
    //     if arr_1[idx]==9{
    //         break;
    //     }
    //     println!("Value {}",arr_1[idx]);
    //     idx+=1;
    // }

    // while idx<arr_1.len() {
    //     println!("Val {}",arr_1[idx]);
    //     idx+=1;
    // }

    // for val in arr_1.iter() {
    //     println!("Val {}",val);
    // }

    // Tuple
    // let tpl: (u8,String,f64) = (4,"Earth".to_string(),5_00.43);
    // println!("{}",tpl.0);
    // let (val_1,val_2,val_3) = tpl;
    // println!("{} --> {} --> {}",val_1,val_2,val_3);

    // String
    // let mut s1: String = String::new();
    // s1.push('A');
    // s1.push_str(" New Place");
    // println!("{}",s1);

    // for word in s1.split_whitespace() {
    //     println!("{}",word);
    // };

    // let s2: String = s1.replace("A","The");
    // println!("{}",s1);
    // println!("{}",s2);

    // let s1: String = String::from("zyabcdef");
    // let mut v1: Vec<char> = s1.chars().collect();
    // v1.sort();
    // v1.dedup();

    // for c in v1 {
    //     println!("{}",c);
    // };
    // let s2: &str = "Randon String";
    // let mut s3: String = s2.to_string();
    // println!("{}",s3);

    // let b1 = s3.as_bytes();
    // let s4: &str = &s3[0..6];
    // println!("string length {} {}",s4.len(),s4);
    // // s3.clear();

    // let s5: String = s1 + &s3;
    // println!("{}",s5);
    // // println!("{}",s1);
    // println!("{}",s3);

    // Type Change
    // let a: u32 = 5;
    // let b: u32 = 4;
    // let c: i32 = (a as i32) + (b as i32);
    // println!("{}",c);
    // println!("{}",a);

    // Enum
    // enum Day{
    //     Monday,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    //     Saturday,
    //     Sunday
    // }

    // impl Day {
    //     fn is_weekend(&self) -> bool {
    //         match self {
    //             Day::Saturday | Day::Sunday => true,
    //             _ => false
    //         }
    //     }
    // }

    // let today: Day = Day::Monday;
    // match today {
    //     Day::Monday => println!("Monday"),
    //     Day::Tuesday => println!("Tuesday"),
    //     Day::Wednesday => println!("Wednesday"),
    //     Day::Thursday => println!("Thursday"),
    //     Day::Friday => println!("Friday"),
    //     Day::Saturday => println!("Saturday"),
    //     Day::Sunday => println!("Sunday"),
    // }

    // println!("Today is a weekend : {}",today.is_weekend());


    // Vectors
    // let v1: Vec<i32> = Vec::new();
    // let mut v2: Vec<i32> = vec![1,2,3,4];
    // v2.push(3);
    // for i in v2.iter() {
    //     println!("{}",i);
    // }

    // let second: &i32 = &v2[1];

    // match v2.get(1) {
    //     Some(second) => println!("Second Value is {}",second),
    //     None => println!("No Second Value"),
    // }

    // for i in &mut v2 {
    //     *i *=2
    // }

    // for i in &v2 {
    //     println!("{}",i);
    // }

    // println!("Len: {}",v2.len());
    // println!("Pop: {:?}",v2.pop());


    // Functions
    // fn print_str(s: &mut String) -> () {
    //     println!("{}",s);
    // }
    
    // fn return_str(s: String) -> String {
    //     s
    // }
    
    // fn change_str(s: &mut String) -> () {
    //     s.push_str(" World with Rust");
    // }
    
    // fn main() {
    //     let mut s1: String = String::from("Hello");
    //     print_str(&mut s1);
    //     // let s3: String = return_str(s1);
    //     change_str(&mut s1);
    //     print_str(&mut s1);
    //     // let v1: i32 = 10;
    //     // let v2: i32 = v1;
    //     // println!("{}",v1);
    // }

    // Hashmap
    // let mut h1: HashMap<&str,&str> = HashMap::new();
    // h1.insert("Mountain","Tree");
    // h1.insert("Sea","Horse");

    // for (k,v) in h1.iter() {
    //     println!("key is {} and value is {}",k,v);
    // }
    // println!("{} is the length",h1.len());

    // if h1.contains_key("Sea") {
    //     let sea = h1.get("Sea");

    //     match sea {
    //         Some(_) => println!("Its a Seahorse"),
    //         None => println!("There is no sea")
    //     }
    // }

    // Ownership
    // struct User {
    //     name: String,
    //     age: i32,
    //     address: String
    // }

    // let mut u1: User = User { 
    //     name: String::from("Nikhil"),
    //     age:32,
    //     address:String::from("flat 2c-006")
    // };
    // println!("{}",u1.age);
    // u1.name = String::from("Nikhil Malladi");
    // println!("{}",u1.name);

    // Traits
    // const PI: f32 = 3.14159265359;
    // struct Rectangle {
    //     length: f32,
    //     width: f32
    // }

    // struct Circle {
    //     length: f32,
    //     width: f32
    // }

    // trait Shape {
    //     fn new(length: f32,width: f32) -> Self;
    //     fn area(&self) -> f32;
    // }

    // impl Shape for Rectangle {
    //     fn new(length: f32,width: f32) -> Rectangle {
    //         Rectangle {
    //             length:length,
    //             width:width
    //         }
    //     }
    //     fn area(&self) -> f32 {
    //         self.length * self.width
    //     }
    // }

    // impl Shape for Circle {
    //     fn new(length: f32,width: f32) -> Circle {
    //         Circle {
    //             length:length,
    //             width:width
    //         }
    //     }

    //     fn area(&self) -> f32 {
    //         (&self.length/2.0).powi(2) * PI
    //     }
    // }

    // let r1: Rectangle = Rectangle::new(10.4,5.5);
    // let r_area: f32 = r1.area();
    // println!("Area of Rectangle: {}",r_area);

    // let a1: Circle = Circle::new(6.4,6.4);
    // let a_area: f32 = a1.area();
    // println!("Area of Circle: {}",a_area);
}