use std::path::PathBuf;
use eframe::Frame;
use egui::Ui;
use navaltoolbox::{Hull as NavalHull, HydrostaticsCalculator, Vessel};
use crate::density_of_water::{density,Temp, WaterType};

#[derive(Default)]
pub(crate) struct Hydrostatics {
    stl_file: PathBuf,
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
    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().
            add_filter("stl", &["stl"]).
            pick_file()
        {
            self.stl_file = path
        }
    }

    fn run_calculations(&mut self) {
        let Some(path_str) = self.stl_file.to_str() else { return; };

        if let Ok(hull) = NavalHull::from_stl(path_str) {
            let vessel = Vessel::new(hull);
            let rho = density(WaterType::Salt, Temp::Twenty);
            let _calc = HydrostaticsCalculator::new(&vessel, rho);

            println!("Calculated vessel displacement/hydrostatics");
        }
    }

    fn display_footer(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("File loaded: ");
            ui.label(self.stl_file.to_str().unwrap());
        });
    }
}

impl eframe::App for Hydrostatics {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        egui::Panel::bottom("footer_panel")
            .resizable(false)
            .min_size(30.0)
            .show_inside(ui, |ui_content| {
                self.display_footer(ui_content)
            });

        egui::CentralPanel::default_margins().show_inside(ui, |ui_content| {
            if ui_content.button("Load Hydrostatics").clicked() {
                self.open_file();
                self.run_calculations();
            }
        });
    }
}

