use std::{ops::Add, str::FromStr};
#[derive(Debug, Clone, PartialEq)]
pub struct TemperatureEntry {
    pub city_name: String,
    pub temperature: f32,
    pub min_temperature: f32,
    pub max_temperature: f32,
    pub count: u64,
}

impl FromStr for TemperatureEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        str::split_once(s, ';')
            .and_then(|(city_name, temp)| {
                let temp = temp.parse::<f32>().ok()?;
                Some(TemperatureEntry {
                    city_name: city_name.to_owned(),
                    temperature: temp,
                    min_temperature: temp,
                    max_temperature: temp,
                    count: 1,
                })
            })
            .ok_or(())
    }
}

impl Add<&TemperatureEntry> for TemperatureEntry {
    type Output = Self;

    fn add(self, other: &TemperatureEntry) -> Self::Output {
        Self {
            city_name: self.city_name,
            temperature: self.temperature + other.temperature,
            min_temperature: self.min_temperature.min(other.min_temperature),
            max_temperature: self.max_temperature.max(other.max_temperature),
            count: self.count + other.count,
        }
    }
}
