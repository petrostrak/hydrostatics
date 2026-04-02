use crate::hydrostatics::Hydrostatics;

mod hydrostatics;

fn main() -> eframe::Result<()>{
    eframe::run_native(
        "Hydrostatics",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Hydrostatics::default())))
    )
}


