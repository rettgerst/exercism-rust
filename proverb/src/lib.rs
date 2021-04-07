pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let mut losses = (1..list.len())
        .map(|ln| format!("For want of a {} the {} was lost.", list[ln - 1], list[ln]))
        .collect::<Vec<String>>();

    let want = format!("And all for the want of a {}.", list[0]).to_string();

    let mut lines = vec![];

    lines.append(&mut losses);

    lines.push(want);

    lines.join("\n")
}
