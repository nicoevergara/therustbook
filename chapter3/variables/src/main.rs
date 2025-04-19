use std::fmt;

fn main() -> () {
    let initial_temperature = Temperature {
        unit: TemperatureUnit::Celsius,
        degrees: 0f64
    };
    let temperature_in_fahrenheit = convert_temperature(initial_temperature, TemperatureUnit::Celsius);

    println!("{}", temperature_in_fahrenheit);

}

#[derive(Debug, PartialEq, Clone)]
struct Temperature {
    unit: TemperatureUnit,
    degrees: f64
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°{}", self.degrees, self.unit)
    }
}

impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit_symbol: &str = match self {
            TemperatureUnit::Celsius => "C",
            TemperatureUnit::Fahrenheit => "F",
        };
        write!(f, "{}", unit_symbol)
    }
}

fn convert_temperature(Temperature { unit, degrees}: Temperature, unit_to_convert_to: TemperatureUnit) -> Temperature {
    let new_degrees: f64 = match (unit, unit_to_convert_to.clone()) {
        (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => degrees * 1.8f64 + 32.0,
        (TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius) => (degrees - 32.0f64) / 1.8f64,
        (_, _) => degrees
    };

    Temperature {
        degrees: new_degrees,
        unit: unit_to_convert_to
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celcius_to_fahrenheit() {
        let temperature_freezing = Temperature {
            unit: TemperatureUnit::Celsius,
            degrees: 0f64
        };
        let result = convert_temperature(temperature_freezing, TemperatureUnit::Fahrenheit);

        assert_eq!(result, Temperature {
            unit: TemperatureUnit::Fahrenheit,
            degrees: 32f64
        });

        let temperature_boiling = Temperature {
            unit: TemperatureUnit::Celsius,
            degrees: 100f64
        };
        let result = convert_temperature(temperature_boiling, TemperatureUnit::Fahrenheit);

        assert_eq!(result, Temperature {
            unit: TemperatureUnit::Fahrenheit,
            degrees: 212f64
        });
    }

    #[test]
    fn fahrenheit_to_celcius() {
        let temperature_freezing = Temperature {
            unit: TemperatureUnit::Fahrenheit,
            degrees: 32f64
        };
        let result = convert_temperature(temperature_freezing, TemperatureUnit::Celsius);

        assert_eq!(result, Temperature {
            unit: TemperatureUnit::Celsius,
            degrees: 0f64
        });

        let temperature_boiling = Temperature {
            unit: TemperatureUnit::Fahrenheit,
            degrees: 212f64
        };
        let result = convert_temperature(temperature_boiling, TemperatureUnit::Celsius);

        assert_eq!(result, Temperature {
            unit: TemperatureUnit::Celsius,
            degrees: 100f64
        });

        let temperature_gasoline_freezing = Temperature {
            unit: TemperatureUnit::Fahrenheit,
            degrees: -100f64
        };
        let result = convert_temperature(temperature_gasoline_freezing, TemperatureUnit::Celsius);

        assert_eq!(result.to_string(), "-73.3°C");
    }

    #[test]
    fn same_unit_noop() {
        let temperature_freezing = Temperature {
            unit: TemperatureUnit::Fahrenheit,
            degrees: 32f64
        };
        let result = convert_temperature(temperature_freezing, TemperatureUnit::Fahrenheit);

        assert_eq!(result, Temperature {
            unit: TemperatureUnit::Fahrenheit,
            degrees: 32f64
        });

        let temperature_boiling = Temperature {
            unit: TemperatureUnit::Celsius,
            degrees: 100f64
        };
        let result = convert_temperature(temperature_boiling, TemperatureUnit::Celsius);

        assert_eq!(result, Temperature {
            unit: TemperatureUnit::Celsius,
            degrees: 100f64
        });
    }

    #[test]
    fn display_trait() {
        let temperature_freezing = Temperature {
            unit: TemperatureUnit::Fahrenheit,
            degrees: 32f64
        };

        assert_eq!(temperature_freezing.to_string(), "32.0°F");

        let temperature_boiling = Temperature {
            unit: TemperatureUnit::Celsius,
            degrees: 100f64
        };

        assert_eq!(temperature_boiling.to_string(), "100.0°C");

        let temperature_boiling = Temperature {
            unit: TemperatureUnit::Celsius,
            degrees: -10f64
        };

        assert_eq!(temperature_boiling.to_string(), "-10.0°C");
    }
}