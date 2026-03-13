use std::{fs::File, io::Read, path::PathBuf};

use comicinfo::ComicInfo;
use eframe::egui;
use egui_file_dialog::FileDialog;
use zip::{ZipArchive, result::ZipError};

struct App {
    file_dialog: FileDialog,
    opened_files: Vec<(PathBuf, ComicInfo)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new().add_file_filter_extensions("Comics", vec!["cbz"]).default_file_filter("Comics"),
            opened_files: Vec::new(),
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
        if let Some(paths) = self.file_dialog.take_picked_multiple() {
            // Is some cleanup of old files needed here?
            self.opened_files = open_cbzs(paths);
            println!("ComicInfos: {:?}", &self.opened_files);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, World!");
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // flush files
    }
}

fn open_cbzs(paths: Vec<PathBuf>) -> Vec<(PathBuf, ComicInfo)> {
    paths.into_iter()
        .filter_map(|path| {
            match open_cbz(&path) {
                Ok(content) => Some((path, content)),
                Err(e) => {
                    eprintln!("zip error: {}", e);
                    None
                },
            }
        })
        .filter_map(|(path, content)| {
            match quick_xml::de::from_str(&content) {
                Ok(comic_info) => Some((path, comic_info)),
                Err(e) => {
                    eprintln!("xml error: {}", e);
                    None
                },
            }
        })
        .collect()
}

fn open_cbz(path: &PathBuf) -> Result<String, ZipError> {
    let file = File::open(path).map_err(ZipError::Io)?;
    let mut archive = ZipArchive::new(file)?;
    let mut xml = archive.by_name("ComicInfo.xml")?;

    let mut buf = String::new();
    xml.read_to_string(&mut buf).map_err(ZipError::Io)?;
    Ok(buf)
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
