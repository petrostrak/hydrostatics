use egui::CursorIcon::Default;
use crate::hydrostatics::Hydrostatics;

mod hydrostatics;
pub mod density_of_water;

fn main() -> eframe::Result<()>{
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_min_inner_size([400.0, 600.0])
            .with_max_inner_size([400.0, 600.0]),
        ..core::default::Default::default()
    };

    eframe::run_native(
        "Hydrostatics",
        options,
        Box::new(|_cc| Ok(Box::new(Hydrostatics::default())))
    )
}


