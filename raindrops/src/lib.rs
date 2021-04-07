pub fn raindrops(n: u32) -> String {
    let fac_3 = n % 3 == 0;
    let fac_5 = n % 5 == 0;
    let fac_7 = n % 7 == 0;

    if !fac_3 && !fac_5 && !fac_7 {
        return n.to_string();
    }

    let mut outvec = vec![];

    if fac_3 {
        outvec.push("Pling")
    }

    if fac_5 {
        outvec.push("Plang")
    }

    if fac_7 {
        outvec.push("Plong")
    }

    outvec.join("").to_string()
}
