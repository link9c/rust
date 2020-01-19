pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

pub fn slice_a() {
    let a: (&str, usize) = ("kk", 10);
    let s = a.0;

    println!("{},{:?}", s, a);
}
