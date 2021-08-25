fn main() {
    let t1 = 0.0;
    let t2 = 212.0;
    let t3 = 98.6;
    let t4 = 100.0;

    let c1 = to_f(t1);
    let c2 = to_c(t2);
    let c3 = to_c(t3);
    let c4 = to_f(t4);

    println!(
        "0C = {}F, 212F = {}C, 98.6F = {}C, 100C = {} F",
        c1, c2, c3, c4
    );
}

fn to_f(c: f64) -> f64 {
    ((9.0 / 5.0) * c) + 32.0
}

fn to_c(f: f64) -> f64 {
    (5.0 / 9.0) * (f - 32.0)
}
