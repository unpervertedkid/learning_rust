use std::ops::Add;

struct Millimetres(u32);
struct Meters(u32);

impl Add<Meters> for Millimetres {
    type Output = Millimetres;    

    fn add(self, other: Meters) -> Millimetres {
        Millimetres(self.0 + (other.0 * 1000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_meters_to_millimetres() {
        let millimetres = Millimetres(1000);
        let metres = Meters(1);

        let new_millimetres = millimetres.add(metres);

        assert_eq!(2000, new_millimetres.0);
    }
}
