pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];

    for char in string.chars() {
        match char {
            '(' => brackets.push(char),
            '[' => brackets.push(char),
            '{' => brackets.push(char),
            '}' => {
                if brackets.len() == 0 {
                    return false;
                }
                let top = brackets.last().unwrap();
                if *top == '{' {
                    brackets.pop();
                } else {
                    return false;
                }
            }
            ']' => {
                if brackets.len() == 0 {
                    return false;
                }
                let top = brackets.last().unwrap();
                if *top == '[' {
                    brackets.pop();
                } else {
                    return false;
                }
            }
            ')' => {
                if brackets.len() == 0 {
                    return false;
                }
                let top = brackets.last().unwrap();
                if *top == '(' {
                    brackets.pop();
                } else {
                    return false;
                }
            }
            _ => (),
        }
    }

    brackets.len() == 0
}
