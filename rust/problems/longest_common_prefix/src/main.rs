use std::cmp;

fn main() {

    let mut str = Vec::new();
    str.push("aba".to_string());
    str.push("ab".to_string());
    let result = longest_common_prefix(str);
    println!("{}", result);
}

fn longest_common_prefix(str: Vec<String>) -> String {
   let mut result = str[0].clone();
   let mut i = 1;
   while i < str.len() {
       let mut j = 0;
       let min = cmp::min(result.len(), str[i].len());
       while j < min {
           if result.as_bytes()[j] != str[i].as_bytes()[j] {
               break;
           }
           j += 1;
       }
       result = result[..j].to_string();
       i += 1;
   }
   result
}
