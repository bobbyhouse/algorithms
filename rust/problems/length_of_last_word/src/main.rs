fn main() {
    let s = String::from("a");
    println!("{}: Result: {}", 1,length_of_last_word(s));

    let s = String::from(" a");
    println!("{}: Result: {}", 2,length_of_last_word(s));

    let s = String::from("a ");
    println!("{}: Result: {}", 3,length_of_last_word(s));

    let s = String::from(" a ");
    println!("{}: Result: {}", 4,length_of_last_word(s));

    let s = String::from(" aaaa");
    println!("{}: Result: {}", 5,length_of_last_word(s));

    let s = String::from("a aaa");
    println!("{}: Result: {}", 6,length_of_last_word(s));
}

fn length_of_last_word(s: String) -> i32 {
    let mut result = 0;
    let mut found = false;
    for c in s.chars().rev() {
        if c != ' ' && !found {
            found = true;
        } else if c == ' ' && found {
            break;
        }
        if found {
            result += 1;
        }
    } 
    return result;
}
