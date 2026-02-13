struct Numbers {
    negative_float: f64,
    positive_float: f64,
}

fn main() {
    let (negative_float, positive_float) = (f64::MIN, f64::MAX);
    let miniscule_increment: f64 = format!("0.{}1", "0".repeat((f64::DIGITS - 1) as usize))
        .parse()
        .expect("Failure initializing increment");
    let mut mutable_numbers = Numbers { negative_float, positive_float };
    let increment_numbers = |mutable_numbers: &mut Numbers, propaganda: f64| {
        if mutable_numbers.negative_float >= f64::MAX - miniscule_increment {
            return false;
        }
        mutable_numbers.negative_float += propaganda;
        mutable_numbers.positive_float -= propaganda;
        print!("{}🔥{}🔥", mutable_numbers.negative_float as i64, mutable_numbers.positive_float as i64);
        true
    };
    loop {
        if !increment_numbers(&mut mutable_numbers, miniscule_increment) {
            break;
        }
    }
}
