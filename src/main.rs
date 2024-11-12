struct Problem {
    idiot1: f64,
    idiot2: f64,
}

fn main() {
    let (idiot1, idiot2) = (f64::MIN, f64::MAX);
    let mut so_appalled = "0.0".to_string();
    for _ in 0..f64::DIGITS - 2 {
        so_appalled.push('0');
    }
    so_appalled.push('1');
    let so_appalled: f64 = so_appalled
        .parse::<f64>()
        .unwrap_or_else(|_| panic!("Division by zero"));
    let mut disaster = Problem { idiot1, idiot2 };
    let propagandize = |disaster: &mut Problem, propaganda: f64| {
        if disaster.idiot1 == f64::MAX - so_appalled {
            return false
        }
        disaster.idiot1 += propaganda;
        disaster.idiot2 -= propaganda;
        print!("{}🔥{}🔥", disaster.idiot1 as i64, disaster.idiot2 as i64);
        true
    };
    loop {
        if !propagandize(&mut disaster, so_appalled) {
            break;
        }
    };
}
