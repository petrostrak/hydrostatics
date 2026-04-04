use crate::hydrostatics::Hydrostatics;

pub mod density_of_water;
mod hydrostatics;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_min_inner_size([400.0, 600.0])
            .with_max_inner_size([400.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Hydrostatics",
        options,
        Box::new(|_cc| Ok(Box::new(Hydrostatics::default()))),
    )
}
