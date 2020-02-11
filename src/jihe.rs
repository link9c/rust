#![allow(dead_code)]

pub fn vec_func() {
    // let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];
    println!("{}", v[2]);
    assert_eq!(Some(&1), v.get(0));
    println!("{:?}", v.get(2))
}

pub fn iter_num_vec() {
    // 定义可变变量
    let mut v = vec![100, 32, 57];

    // i 为v的引用
    for i in &mut v {
        // 解引用+50
        *i += 50;
    }
    println!("{:?}", v)
}

pub fn iter_str_vec() {
    let mut s = vec!["a", "b", "c"];
    let mut cl = String::new();
    for sd in &mut s {
        let temp: String = "_".to_string() + sd;
        cl.push_str(&temp);
    }
    print!("{}", cl)
}
