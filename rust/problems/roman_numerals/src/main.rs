fn main() {
    let tests = ["LVIII", "III", "MCMXCIV"];

    tests.iter().for_each(|test| {
        println!("{} = {}", test, roman_numeral_to_integer(&test.to_string()));
    });
}

fn roman_numeral_to_integer(roman_numeral: &String) -> i32 {
    let result = roman_numeral.chars().rev().fold((0, 0), |acc, next| {
        let (last, sum) = acc;

        let current = match next {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        if current < last { (current, sum - current) } else { (current, sum + current) }
    });
    result.1
}