use eframe::Frame;
use egui::Ui;

#[derive(Default)]
pub(crate) struct Hydrostatics {
    stl_file: String,
    hull: Hull,
}

#[derive(Default)]
struct Hull {
    draft: f64, // Draft at the reference point in meters
    trim: f64, // Trim angle in degrees (positive = bow down)
    heel: f64, // Heel angle in degrees (positive = starboard down)
    vcg: Option<f64>, // Optional vertical center of gravity for GM calculation
    num_stations: Option<usize>, // Number of stations for sectional area curve (default: 21)
}

impl Hydrostatics {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            stl_file: "open_a_file".to_string(),
            hull: Hull::default(),
        }
    }
}

impl eframe::App for Hydrostatics {
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        egui::CentralPanel::default_margins().show_inside(ui, |i| {

        });
    }
}