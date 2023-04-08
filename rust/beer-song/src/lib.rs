pub fn verse(n: u32) -> String {
    let bottle_left_msg: String = match n.wrapping_sub(1) {
        0 => "Take it down and pass it around, no more bottles of beer on the wall.".to_string(),
        1 => "Take one down and pass it around, 1 bottle of beer on the wall.".to_string(),
        n => format!(
            "Take one down and pass it around, {} bottles of beer on the wall.",
            n
        ),
    };

    match n {
        0 => format!(
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
        ),
        1 => format!(
            "{} bottle of beer on the wall, {} bottle of beer.\n{}\n",
            n, n, bottle_left_msg
        ),
        _ => format!(
            "{} bottles of beer on the wall, {} bottles of beer.\n{}\n",
            n, n, bottle_left_msg
        ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut lyric: String = String::new();

    for i in (end..start + 1).rev() {
        if i == end {
            lyric += &format!("{}", verse(i));
        } else {
            lyric += &format!("{}\n", verse(i));
        }
    }

    lyric
}
