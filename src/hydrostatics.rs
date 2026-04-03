use std::path::PathBuf;
use eframe::Frame;
use egui::Ui;
use egui_notify::{Toasts};
use navaltoolbox::{Hull as NavalHull, HydrostaticsCalculator, Vessel};
use crate::density_of_water::{density, WaterTemperature, WaterType};

#[derive(Default)]
pub(crate) struct Hydrostatics {
    stl_file: PathBuf,
    hull: Hull,
    water_type: WaterType,
    water_temp: WaterTemperature,
    
    toasts: Toasts
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

    fn run_calculations(&mut self, ctx: &egui::Context) {
        let Some(path_str) = self.stl_file.to_str() else {
            self.toasts.info("You must load an .stl file first.");
            ctx.request_repaint();
            return;
        };

        match NavalHull::from_stl(path_str) {
            Ok(hull) => {
                let vessel = Vessel::new(hull);
                let rho = density(&mut self.water_type, &mut self.water_temp);
                let _calc = HydrostaticsCalculator::new(&vessel, rho);

                self.toasts.success("Hydrostatics Calculated Successfully!");
                ctx.request_repaint();
            }
            Err(e) => {
                self.toasts.error(format!("STL Error: {}", e));
                ctx.request_repaint();
            }
        }
    }

    fn display_combo_box(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {ui.label("Water")});
        ui.horizontal(|ui|{
            egui::ComboBox::from_label("Type")
                .selected_text(self.water_type.label())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.water_type, WaterType::Salt, "Salt");
                    ui.selectable_value(&mut self.water_type, WaterType::Fresh, "Fresh");
                });

            egui::ComboBox::from_label("Temperature")
                .selected_text(self.water_temp.label())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.water_temp, WaterTemperature::Zero, "0°C");
                    ui.selectable_value(&mut self.water_temp, WaterTemperature::Ten, "10°C");
                    ui.selectable_value(&mut self.water_temp, WaterTemperature::Twenty, "20°C");
                    ui.selectable_value(&mut self.water_temp, WaterTemperature::Thirty, "30°C");
                });
        });
        ui.separator();
    }

    fn display_footer(&mut self, ui: &mut Ui) {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if ui.button("📂").clicked() {
                self.open_file()
            }
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
            .show_inside(ui, |ui| {
                self.display_footer(ui)
            });

        egui::CentralPanel::default_margins().show_inside(ui, |ui| {
            self.display_combo_box(ui);
            if ui.button("Load Hydrostatics").clicked() {
                self.run_calculations(ui.ctx());
            }
        });

        self.toasts.show(ui.ctx());
    }
}

