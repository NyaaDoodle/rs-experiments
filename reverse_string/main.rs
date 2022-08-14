fn solution(phrase: &str) -> String {
    let str: String = phrase.to_owned();
    let mut rev_str = String::new();
    for c in str.chars().rev() {
        rev_str.push(c);
    }
    return rev_str;
}

fn main() {
    println!("{}", solution("hewwo world"));
}