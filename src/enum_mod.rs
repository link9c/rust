#![allow(dead_code)]

use std::fmt::Debug;
use std::cmp::PartialEq;

#[derive(Debug)]

enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}
#[derive(PartialEq, Debug)] 
struct Point {
    x: u32,
    y: u32,
}
#[derive(Debug)]
enum Keys<T> {
    UpKey(T),
    DownKey(T),
    LeftKey(T),
    RightKey(T),
}



impl Direction {
    
  
    fn match_direction(&self) -> Keys<String> {
        match *self {
            
            Direction::Up(ref s) => {
                const P:Point = Point {x:10,y:10};
            
                if *s == P {
                    Keys::UpKey(String::from("press w"))
                } else{
                    Keys::UpKey(String::from("press ww"))
                }
                
        },
            Direction::Down(_) => Keys::DownKey(String::from("press s")),
            Direction::Left(_) => Keys::LeftKey(String::from("press a")),
            Direction::Right(_) => Keys::RightKey(String::from("press d")),
        }
    }
}

impl Keys<String> {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}



pub fn print_enum() {
    let u = Direction::Up(Point { x: 10, y: 10 });
    let k = u.match_direction();
    let d = k.destruct();
    println!("{:?}", k);
    println!("{}", d)
}
