use crate::hydrostatics::Hydrostatics;

mod hydrostatics;
pub mod density_of_water;

fn main() -> eframe::Result<()>{
    eframe::run_native(
        "Hydrostatics",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Hydrostatics::default())))
    )
}


