pub mod yinyong_mod;
pub mod slice_mod;

fn main() {
    // cal();
    // let s2 = "heeeeee".to_string();
    // let (a, b) = cal_length(s2);
    // println!("{},{}", a, b);

    // let l = cal_lenghth_yy(&s2);
    // println!("{},{}", s2, l);
    // let g = &s2;
    // println!("{}", *g);
    let mut s = String::from("hello");
    yinyong_mod::change(&mut s);
    print!("{}", s);
    let my_string = String::from("hello world");

    // first_word 中传入 `String` 的 slice
    let word = slice_mod::first_word(&my_string[..]);

    println!("{}",word );
    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = slice_mod::first_word(&my_string_literal[..]);
    println!("{}",word );

    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = slice_mod::first_word(my_string_literal);
    println!("{}",word );
    
}
