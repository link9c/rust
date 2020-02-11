// pub fn cal() {
//     let x = 3.5;
//     let y = 4.4;
//     let res: f64 = y / x;
//     println!("{}", res)
// }

// 移动
// pub fn cal_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// 引用
// pub fn cal_lenghth_yy(s: &String) -> usize {
//     s.len()
// }

// 可变引用
pub fn change(st: &mut String) {
    st.push_str("aaaa");
}
