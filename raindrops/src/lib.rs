pub fn raindrops(num: i64) -> String {
    let is_pling = |n| n % 3 == 0;
    let is_plang = |n| n % 5 == 0;
    let is_plong = |n| n % 7 == 0;

    let mut output = String::new();

    if is_pling(num) {
        output.push_str("Pling");
    }
    if is_plang(num) {
        output.push_str("Plang");
    }
    if is_plong(num) {
        output.push_str("Plong");
    }
    if output.is_empty() {
        let s = format!("{}", num);
        output.push_str(&s);
    }
    output
}
