use std::io::Read;

use comicinfo::ComicInfo;
use eframe::egui;
use egui_file_dialog::FileDialog;

struct App {
    file_dialog: FileDialog,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                egui::global_theme_preference_switch(ui);

                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.file_dialog.pick_multiple();
                    }

                    if ui.button("Close").clicked() {
                        println!("Close");
                    }
                });
            });
        });

        self.file_dialog.update(ctx);
        if let Some(files) = self.file_dialog.take_picked_multiple() {
            println!("Selected files: {:?}", files);

            let file = files.get(0).unwrap();

            match std::fs::File::open(&file) {
                Ok(reader) => {
                    match zip::ZipArchive::new(reader) {
                        Ok(mut archive) => {
                            if let Ok(mut comic_info_file) = archive.by_name("ComicInfo.xml") {
                                let mut xml_content = String::new();
                                if comic_info_file.read_to_string(&mut xml_content).is_ok() {
                                    match quick_xml::de::from_str::<ComicInfo>(&xml_content) {
                                        Ok(comic_info) => {
                                            println!("Loaded ComicInfo: {:?}", comic_info);
                                            // Store comic_info for later modification and update
                                        }
                                        Err(e) => eprintln!("Failed to deserialize ComicInfo: {}", e),
                                    }
                                }
                            }
                        }
                        Err(e) => eprintln!("Failed to open CBZ file: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to read file: {}", e),
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, World!");
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // flush files
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "cbz.Edit",
        eframe::NativeOptions {
            vsync: true,
            ..Default::default()
        },
        Box::new(|_| {
            Ok(Box::<App>::default())
        }),
    )
}
