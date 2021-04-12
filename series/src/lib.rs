pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![];
    }

    let chars = digits.chars().collect::<Vec<_>>();

    let mut series = vec![];

    for i in 0..chars.len() - len + 1 {
        let str = &chars[i..i + len];

        series.push(str.iter().collect())
    }

    series.into_iter().collect::<Vec<String>>()
}
