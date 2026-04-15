#[derive(Clone, Copy, Debug)]
pub enum Unit {
    Celsius,
    Fahrenheit,
}

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

    pub fn converted(&self) -> Temperature {
        match self.unit {
            Unit::Celsius => Temperature::new(celsius_to_fahrenheit(self.degree), Unit::Fahrenheit),
            Unit::Fahrenheit => Temperature::new(fahrenheit_to_celsius(self.degree), Unit::Celsius),
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
        let converted = temp.converted();

        assert!(matches!(converted.unit(), Unit::Fahrenheit));
    }

    #[test]
    fn test_converted_from_fahrenheit_changes_unit() {
        let temp = Temperature::new(32.0, Unit::Fahrenheit);
        let converted = temp.converted();

        assert!(matches!(converted.unit(), Unit::Celsius));
    }

    #[test]
    fn test_converted_from_celsius_changes_degree() {
        let temp = Temperature::new(0.0, Unit::Celsius);
        let converted = temp.converted();

        assert_eq!(converted.degree(), 32.0);
    }

    #[test]
    fn test_converted_from_fahrenheit_changes_degree() {
        let temp = Temperature::new(32.0, Unit::Fahrenheit);
        let converted = temp.converted();

        assert_eq!(converted.degree(), 0.0);
    }

    #[test]
    fn test_converted_twice_returns_original_unit() {
        let temp = Temperature::new(10.0, Unit::Celsius);
        let converted_twice = temp.converted().converted();

        assert!(matches!(converted_twice.unit(), Unit::Celsius));
    }

    #[test]
    fn test_converted_twice_returns_original_degree() {
        let temp = Temperature::new(10.0, Unit::Celsius);
        let converted_twice = temp.converted().converted();

        assert_eq!(converted_twice.degree(), 10.0);
    }
}
