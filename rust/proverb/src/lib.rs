pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::from("");
    }

    let last_index = list.len();
    let first_word = list[0];

    let mut sentences = Vec::new();

    for i in 0..(last_index) {
        if i < last_index - 1 {
            let word = list[i];
            let next_word = list[i+1];
            sentences.push(format!("For want of a {} the {} was lost.", word, next_word));
        } else {
            sentences.push(format!("And all for the want of a {}.", first_word));
        }
    }

    return sentences.join("\n");
}