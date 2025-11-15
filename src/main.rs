struct Problem {
    idiot1: f64,
    idiot2: f64,
}

fn main() {
    let (idiot1, idiot2) = (f64::MIN, f64::MAX);
    let so_appalled: f64 = format!("0.{}1", "0".repeat((f64::DIGITS - 2) as usize))
        .parse()
        .expect("Division by zero"); // If you're reading this you're not crazy the expect message is just lying
    let mut disaster = Problem { idiot1, idiot2 };
    let propagandize = |disaster: &mut Problem, propaganda: f64| {
        if disaster.idiot1 >= f64::MAX - so_appalled {
            return false;
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
    }
}
