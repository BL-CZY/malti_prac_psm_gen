use eframe::egui;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Multi-Screen Text Editor with MP3 File Selector",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Debug, Clone, PartialEq)]
enum AppScreen {
    Main,
    Settings,
    FileManager,
    TextAnalyzer,
}

#[derive(Default)]
struct MyApp {
    current_screen: AppScreen,

    // Main screen data
    text_area_1: String,
    text_area_2: String,
    mp3_file_1: Option<PathBuf>,
    mp3_file_2: Option<PathBuf>,

    // Settings screen data
    window_title: String,
    auto_save: bool,
    theme_dark: bool,

    // File manager screen data
    file_history: Vec<PathBuf>,
}

impl Default for AppScreen {
    fn default() -> Self {
        AppScreen::Main
    }
}

impl MyApp {
    fn render_text_analyzer_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("Text Analyzer");
        ui.separator();

        // Split the content horizontally using columns
        ui.columns(2, |columns| {
            // Left column - Text Area 1 analysis
            columns[0].group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Text Area 1 - Line by Line:");
                    ui.add_space(5.0);

                    let lines_1: Vec<&str> = self.text_area_1.split('\n').collect();

                    egui::ScrollArea::vertical()
                        .max_height(350.0)
                        .show(ui, |ui| {
                            if lines_1.is_empty() || (lines_1.len() == 1 && lines_1[0].is_empty()) {
                                ui.label("No text content to analyze");
                            } else {
                                for (i, line) in lines_1.iter().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{}:", i + 1));
                                        if line.is_empty() {
                                            ui.label("(empty line)");
                                        } else {
                                            ui.label(format!("\"{}\"", line));
                                        }
                                    });
                                }
                            }
                        });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.label(format!("Total lines: {}", lines_1.len()));
                    ui.label(format!(
                        "Non-empty lines: {}",
                        lines_1.iter().filter(|&line| !line.is_empty()).count()
                    ));
                    ui.label(format!(
                        "Empty lines: {}",
                        lines_1.iter().filter(|&line| line.is_empty()).count()
                    ));
                });
            });

            // Right column - Text Area 2 analysis
            columns[1].group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Text Area 2 - Line by Line:");
                    ui.add_space(5.0);

                    let lines_2: Vec<&str> = self.text_area_2.split('\n').collect();

                    egui::ScrollArea::vertical()
                        .max_height(350.0)
                        .show(ui, |ui| {
                            if lines_2.is_empty() || (lines_2.len() == 1 && lines_2[0].is_empty()) {
                                ui.label("No text content to analyze");
                            } else {
                                for (i, line) in lines_2.iter().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{}:", i + 1));
                                        if line.is_empty() {
                                            ui.label("(empty line)");
                                        } else {
                                            ui.label(format!("\"{}\"", line));
                                        }
                                    });
                                }
                            }
                        });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.label(format!("Total lines: {}", lines_2.len()));
                    ui.label(format!(
                        "Non-empty lines: {}",
                        lines_2.iter().filter(|&line| !line.is_empty()).count()
                    ));
                    ui.label(format!(
                        "Empty lines: {}",
                        lines_2.iter().filter(|&line| line.is_empty()).count()
                    ));
                });
            });
        });

        ui.add_space(20.0);

        // Overall statistics
        ui.separator();
        ui.group(|ui| {
            ui.label("Combined Statistics:");
            let total_lines_1 = self.text_area_1.split('\n').count();
            let total_lines_2 = self.text_area_2.split('\n').count();
            let total_chars_1 = self.text_area_1.len();
            let total_chars_2 = self.text_area_2.len();

            ui.label(format!(
                "Total lines across both areas: {}",
                total_lines_1 + total_lines_2
            ));
            ui.label(format!(
                "Total characters across both areas: {}",
                total_chars_1 + total_chars_2
            ));
            ui.label(format!(
                "Average line length: {:.1} characters",
                if total_lines_1 + total_lines_2 > 0 {
                    (total_chars_1 + total_chars_2) as f32 / (total_lines_1 + total_lines_2) as f32
                } else {
                    0.0
                }
            ));
        });
    }

    fn render_main_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("Text Editor with MP3 File Selector");
        ui.separator();

        // First section: Text Area 1 and MP3 File 1
        ui.group(|ui| {
            ui.label("Text Area 1:");
            egui::ScrollArea::vertical()
                .id_source("0")
                .max_height(150.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.text_area_1)
                            .desired_width(f32::INFINITY)
                            .desired_rows(8)
                            .hint_text("Enter your text here..."),
                    );
                });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("MP3 File 1:");
                if let Some(path) = &self.mp3_file_1 {
                    ui.label(format!("Selected: {}", path.display()));
                } else {
                    ui.label("No file selected");
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Browse MP3 File 1").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("MP3 Audio", &["mp3"])
                        .set_title("Select MP3 File 1")
                        .pick_file()
                    {
                        self.mp3_file_1 = Some(path.clone());
                        if !self.file_history.contains(&path) {
                            self.file_history.push(path);
                        }
                    }
                }

                if self.mp3_file_1.is_some() && ui.button("Clear MP3 File 1").clicked() {
                    self.mp3_file_1 = None;
                }
            });
        });

        ui.add_space(20.0);

        // Second section: Text Area 2 and MP3 File 2
        ui.group(|ui| {
            ui.label("Text Area 2:");
            egui::ScrollArea::vertical()
                .id_source("1")
                .max_height(150.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.text_area_2)
                            .desired_width(f32::INFINITY)
                            .desired_rows(8)
                            .hint_text("Enter your text here..."),
                    );
                });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("MP3 File 2:");
                if let Some(path) = &self.mp3_file_2 {
                    ui.label(format!("Selected: {}", path.display()));
                } else {
                    ui.label("No file selected");
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Browse MP3 File 2").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("MP3 Audio", &["mp3"])
                        .set_title("Select MP3 File 2")
                        .pick_file()
                    {
                        self.mp3_file_2 = Some(path.clone());
                        if !self.file_history.contains(&path) {
                            self.file_history.push(path);
                        }
                    }
                }

                if self.mp3_file_2.is_some() && ui.button("Clear MP3 File 2").clicked() {
                    self.mp3_file_2 = None;
                }
            });
        });

        ui.add_space(20.0);

        // Navigation to Text Analyzer
        ui.separator();
        if ui.button("🔍 Analyze Text Content").clicked() {
            self.current_screen = AppScreen::TextAnalyzer;
        }

        // Status section
        ui.separator();
        ui.label("Status:");
        ui.label(format!(
            "Text Area 1: {} characters",
            self.text_area_1.len()
        ));
        ui.label(format!(
            "Text Area 2: {} characters",
            self.text_area_2.len()
        ));

        if let Some(path1) = &self.mp3_file_1 {
            ui.label(format!(
                "MP3 File 1: {}",
                path1.file_name().unwrap_or_default().to_string_lossy()
            ));
        }

        if let Some(path2) = &self.mp3_file_2 {
            ui.label(format!(
                "MP3 File 2: {}",
                path2.file_name().unwrap_or_default().to_string_lossy()
            ));
        }
    }

    fn render_settings_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.separator();

        ui.group(|ui| {
            ui.label("Application Settings");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Window Title:");
                ui.text_edit_singleline(&mut self.window_title);
            });

            ui.add_space(5.0);
            ui.checkbox(&mut self.auto_save, "Auto-save text content");
            ui.checkbox(&mut self.theme_dark, "Dark theme (not implemented)");

            ui.add_space(10.0);

            if ui.button("Reset to Defaults").clicked() {
                self.window_title.clear();
                self.auto_save = false;
                self.theme_dark = false;
            }
        });

        ui.add_space(20.0);

        ui.group(|ui| {
            ui.label("Statistics");
            ui.add_space(5.0);
            ui.label(format!(
                "Total files in history: {}",
                self.file_history.len()
            ));
            ui.label(format!(
                "Current text length: {} + {} = {} characters",
                self.text_area_1.len(),
                self.text_area_2.len(),
                self.text_area_1.len() + self.text_area_2.len()
            ));

            if ui.button("Clear All Data").clicked() {
                self.text_area_1.clear();
                self.text_area_2.clear();
                self.mp3_file_1 = None;
                self.mp3_file_2 = None;
            }
        });
    }

    fn render_file_manager_screen(&mut self, ui: &mut egui::Ui) {
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

    fn render_navigation(&mut self, ui: &mut egui::Ui) {
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

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_navigation(ui);
            ui.add_space(10.0);

            match self.current_screen {
                AppScreen::Main => self.render_main_screen(ui),
                AppScreen::Settings => self.render_settings_screen(ui),
                AppScreen::FileManager => self.render_file_manager_screen(ui),
                AppScreen::TextAnalyzer => self.render_text_analyzer_screen(ui),
            }
        });
    }
}
