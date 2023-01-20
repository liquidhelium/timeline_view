use timeline_view::*;

struct App {
    tracks: Vec<String>,
}
impl Default for App {
    fn default() -> Self {
        Self {
            tracks: vec![
                "helllllllllloooooooooooooooooooooooooooooooooooooooooooooooooooooooooo"
                .to_owned();
                40
            ],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            TrackView::new(&mut self.tracks)
                .header_width(60.0)
                .id_source("id_source")
                .style(Style::from_egui_style(ctx.style().as_ref()))
                .show(
                    ui,
                    |ui, data| {
                        ui.label(&*data);
                        ui.collapsing("heading", |ui| {
                            if ui.button("change").clicked() {
                                *data = "str".into();
                            }
                        });
                    },
                    |ui, data| {
                        ui.label(&*data);
                    },
                )
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("some", options, Box::new(|_cc| Box::new(App::default())));
}
