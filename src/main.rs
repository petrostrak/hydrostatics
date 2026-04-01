#[derive(Default)]
struct Hydrostatics;

impl Hydrostatics {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for Hydrostatics {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("Hydrostatics");
        });
    }
}

fn main() -> eframe::Result<()>{
    eframe::run_native(
        "Hydrostatics",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Hydrostatics::default())))
    )
}


