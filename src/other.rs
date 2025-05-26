use crate::{AppScreen, MyApp};

impl MyApp {
    pub fn render_file_manager_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("File Manager");
        ui.separator();

        ui.group(|ui| {
            ui.label("Currently Selected Files");
            ui.add_space(5.0);

            let mut clear_file_1 = false;
            let mut clear_file_2 = false;

            if let Some(path) = &self.mp3_file_1 {
                ui.horizontal(|ui| {
                    ui.label("MP3 File 1:");
                    ui.label(path.display().to_string());
                    if ui.button("Remove").clicked() {
                        clear_file_1 = true;
                    }
                });
            } else {
                ui.label("MP3 File 1: Not selected");
            }

            if let Some(path) = &self.mp3_file_2 {
                ui.horizontal(|ui| {
                    ui.label("MP3 File 2:");
                    ui.label(path.display().to_string());
                    if ui.button("Remove").clicked() {
                        clear_file_2 = true;
                    }
                });
            } else {
                ui.label("MP3 File 2: Not selected");
            }

            if clear_file_1 {
                self.mp3_file_1 = None;
            }
            if clear_file_2 {
                self.mp3_file_2 = None;
            }
        });

        ui.add_space(20.0);

        ui.group(|ui| {
            ui.label("File History");
            ui.add_space(5.0);

            if self.file_history.is_empty() {
                ui.label("No files in history");
            } else {
                let mut to_remove = None;
                let mut select_file_1 = None;
                let mut select_file_2 = None;

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (i, path) in self.file_history.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}. {}", i + 1, path.display()));

                                if ui.button("Select as File 1").clicked() {
                                    select_file_1 = Some(path.clone());
                                }

                                if ui.button("Select as File 2").clicked() {
                                    select_file_2 = Some(path.clone());
                                }

                                if ui.button("Remove").clicked() {
                                    to_remove = Some(i);
                                }
                            });
                        }
                    });

                if let Some(index) = to_remove {
                    self.file_history.remove(index);
                }

                if let Some(path) = select_file_1 {
                    self.mp3_file_1 = Some(path);
                }

                if let Some(path) = select_file_2 {
                    self.mp3_file_2 = Some(path);
                }

                ui.add_space(10.0);
                if ui.button("Clear History").clicked() {
                    self.file_history.clear();
                }
            }
        });
    }

    pub fn render_navigation(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.current_screen, AppScreen::Main, "📝 Main");
            ui.selectable_value(&mut self.current_screen, AppScreen::Settings, "⚙️ Settings");
            ui.selectable_value(
                &mut self.current_screen,
                AppScreen::FileManager,
                "📁 File Manager",
            );
            ui.selectable_value(
                &mut self.current_screen,
                AppScreen::TextAnalyzer,
                "🔍 Text Analyzer",
            );
        });
        ui.separator();
    }
}
