#[derive(Clone, Copy, Debug)]
pub enum Unit {
    Celsius,
    Fahrenheit,
}

#[derive(Clone, Copy)]
pub struct Temperature {
    degree: f64,
    unit: Unit,
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 9.0 / 5.0 + 32.0
}

impl Temperature {
    pub fn new(degree: f64, unit: Unit) -> Self {
        Self { degree, unit }
    }

    pub fn degree(&self) -> f64 {
        self.degree
    }

    pub fn unit(&self) -> Unit {
        self.unit
    }

    pub fn to(&self, target: Unit) -> Temperature {
        match (self.unit, target) {
            (Unit::Celsius, Unit::Celsius) => *self,
            (Unit::Fahrenheit, Unit::Fahrenheit) => *self,
            (Unit::Fahrenheit, Unit::Celsius) => {
                Temperature::new(fahrenheit_to_celsius(self.degree), target)
            }
            (Unit::Celsius, Unit::Fahrenheit) => {
                Temperature::new(celsius_to_fahrenheit(self.degree), target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }

    #[test]
    fn test_new_temperature_stores_degree() {
        let temp = Temperature::new(25.0, Unit::Celsius);
        assert_eq!(temp.degree(), 25.0);
    }

    #[test]
    fn test_new_temperature_stores_unit() {
        let temp = Temperature::new(25.0, Unit::Celsius);
        assert!(matches!(temp.unit(), Unit::Celsius));
    }

    #[test]
    fn test_converted_from_celsius_changes_unit() {
        let temp = Temperature::new(0.0, Unit::Celsius);
        let converted = temp.to(Unit::Fahrenheit);

        assert!(matches!(converted.unit(), Unit::Fahrenheit));
    }

    #[test]
    fn test_converted_from_fahrenheit_changes_unit() {
        let temp = Temperature::new(32.0, Unit::Fahrenheit);
        let converted = temp.to(Unit::Celsius);

        assert!(matches!(converted.unit(), Unit::Celsius));
    }

    #[test]
    fn test_converted_from_celsius_changes_degree() {
        let temp = Temperature::new(0.0, Unit::Celsius);
        let converted = temp.to(Unit::Fahrenheit);

        assert_eq!(converted.degree(), 32.0);
    }

    #[test]
    fn test_converted_from_fahrenheit_changes_degree() {
        let temp = Temperature::new(32.0, Unit::Fahrenheit);
        let converted = temp.to(Unit::Celsius);

        assert_eq!(converted.degree(), 0.0);
    }

    #[test]
    fn test_to_same_unit_keeps_celsius_unchanged() {
        let temp = Temperature::new(25.0, Unit::Celsius);
        let converted = temp.to(Unit::Celsius);

        assert_eq!(converted.degree(), 25.0);
        assert!(matches!(converted.unit(), Unit::Celsius));
    }

    #[test]
    fn test_to_same_unit_keeps_fahrenheit_unchanged() {
        let temp = Temperature::new(77.0, Unit::Fahrenheit);
        let converted = temp.to(Unit::Fahrenheit);

        assert_eq!(converted.degree(), 77.0);
        assert!(matches!(converted.unit(), Unit::Fahrenheit));
    }
}
