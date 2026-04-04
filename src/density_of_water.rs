#[derive(Debug, Default, PartialEq)]
pub enum WaterType {
    #[default]
    Fresh,
    Salt,
}

impl WaterType {
    pub fn label(&self) -> &str {
        match self {
            WaterType::Fresh => "Fresh",
            WaterType::Salt => "Salt",
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum WaterTemperature {
    #[default]
    Zero,
    Ten,
    Twenty,
    Thirty,
}

impl WaterTemperature {
    pub fn label(&self) -> &str {
        match self {
            WaterTemperature::Zero => "0°C",
            WaterTemperature::Ten => "10°C",
            WaterTemperature::Twenty => "20°C",
            WaterTemperature::Thirty => "30°C",
        }
    }
}

pub fn density(water: &mut WaterType, temp: &mut WaterTemperature) -> f64 {
    match (water, temp) {
        (WaterType::Fresh, WaterTemperature::Zero) => 999.8,
        (WaterType::Fresh, WaterTemperature::Ten) => 999.6,
        (WaterType::Fresh, WaterTemperature::Twenty) => 998.1,
        (WaterType::Fresh, WaterTemperature::Thirty) => 995.7,
        (WaterType::Salt, WaterTemperature::Zero) => 1028.0,
        (WaterType::Salt, WaterTemperature::Ten) => 1027.0,
        (WaterType::Salt, WaterTemperature::Twenty) => 1025.0,
        (WaterType::Salt, WaterTemperature::Thirty) => 1022.0,
    }
}
