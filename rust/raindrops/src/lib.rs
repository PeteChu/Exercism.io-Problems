pub fn raindrops(n: u32) -> String {
    let mut ans = String::new();
    ans += if n % 3 == 0 {"Pling"} else {""};
    ans += if n % 5 == 0 {"Plang"} else {""};
    ans += if n % 7 == 0 {"Plong"} else {""};
    return if ans == "" {n.to_string()} else {ans};
}
