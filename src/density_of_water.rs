pub enum WaterType { Fresh, Salt }
pub enum Temp { Zero, Ten, Twenty, Thirty }

pub fn density(water: WaterType, temp: Temp) -> f64 {
    match (water, temp) {
        (WaterType::Fresh, Temp::Zero) => 999.8,
        (WaterType::Fresh, Temp::Ten) => 999.6,
        (WaterType::Fresh, Temp::Twenty) => 998.1,
        (WaterType::Fresh, Temp::Thirty) => 995.7,
        (WaterType::Salt, Temp::Zero) => 1028.0,
        (WaterType::Salt, Temp::Ten) => 1027.0,
        (WaterType::Salt, Temp::Twenty) => 1025.0,
        (WaterType::Salt, Temp::Thirty) => 1022.0
    }
}