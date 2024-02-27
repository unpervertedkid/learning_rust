struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(23);
    let age_in_days = age.to_days();
    println!("Old enough: {}", old_enough(&age));
    println!("Old enough: {}", old_enough(&age_in_days.to_years()));
}
