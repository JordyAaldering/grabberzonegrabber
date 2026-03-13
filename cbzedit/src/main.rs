use std::{fs::File, io::{Cursor, Read, Write}, path::PathBuf};

use comicinfo::ComicInfo;
use eframe::egui;
use egui_file_dialog::FileDialog;
use zip::{ZipArchive, ZipWriter, result::ZipError, write::SimpleFileOptions};

struct App {
    file_dialog: FileDialog,
    opened_files: Vec<(PathBuf, ComicInfo)>,
    shared_info: ComicInfo,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new().add_file_filter_extensions("Comics", vec!["cbz"]).default_file_filter("Comics"),
            opened_files: Vec::new(),
            shared_info: ComicInfo::default(),
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

                    if ui.button("Save").clicked() {
                        for (path, comic_info) in &self.opened_files {
                            if let Err(e) = write_cbz(path, &comic_info, &self.shared_info) {
                                eprintln!("Error writing CBZ: {}", e);
                            }
                        }
                    }
                });
            });
        });

        self.file_dialog.update(ctx);
        if let Some(paths) = self.file_dialog.take_picked_multiple() {
            // Is some cleanup of old files needed here?
            self.opened_files = open_cbzs(paths);
            println!("ComicInfos: {:?}", &self.opened_files);
            self.shared_info = created_shared_info(&self.opened_files);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Title");
            let mut title = self.shared_info.title.clone().unwrap_or_default();
            if ui.text_edit_singleline(&mut title).changed() {
                self.shared_info.title = Some(title);
            }

            ui.label("Series");
            let mut series = self.shared_info.series.clone().unwrap_or_default();
            if ui.text_edit_singleline(&mut series).changed() {
                self.shared_info.series = Some(series);
            }

            ui.label("LanguageISO");
            let mut language_iso = self.shared_info.language_iso.clone().unwrap_or_default();
            if ui.text_edit_singleline(&mut language_iso).changed() {
                self.shared_info.language_iso = Some(language_iso);
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // flush files
    }
}

fn created_shared_info(opened_files: &Vec<(PathBuf, ComicInfo)>) -> ComicInfo {
    let mut iter = opened_files.iter().map(|(_, info)| info);
    if let Some(comic_info) = iter.next() {
        let mut shared_info = comic_info.clone();

        for comic_info in iter {
            if shared_info.title != comic_info.title {
                shared_info.title = None;
            }
            if shared_info.series != comic_info.series {
                shared_info.series = None;
            }
            if shared_info.language_iso != comic_info.language_iso {
                shared_info.language_iso = None;
            }
        }

        shared_info
    } else {
        ComicInfo::default()
    }
}

/// TODO: currently, this assumes a ComicInfo.xml MUST exist
/// Ideally, we create one if it is not there yet.
/// I.e. -> Vec<(PathBuf, Option<ComicInfo>)>
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

fn write_cbz(path: &PathBuf, old_comic_info: &ComicInfo, new_comic_info: &ComicInfo) -> Result<(), ZipError> {
    println!("Updating ComicInfo.xml in {} with {:?}", path.display(), new_comic_info);
    let file = File::open(path).map_err(ZipError::Io)?;
    let mut archive = ZipArchive::new(file)?;

    let buf = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(buf);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name();

        if name == "ComicInfo.xml" {
            continue;
        }

        let options = SimpleFileOptions::default()
            .compression_method(file.compression());

        if file.is_dir() {
            zip.add_directory(name, options)?;
        } else {
            zip.start_file(name, options)?;
            std::io::copy(&mut file, &mut zip)?;
        }
    }

    // Add updated ComicInfo.xml
    zip.start_file("ComicInfo.xml", SimpleFileOptions::default())?;
    let comic_info = merge_comic_info(old_comic_info, new_comic_info);
    quick_xml::se::to_utf8_io_writer(&mut zip, &comic_info).unwrap();

    // Overwrite original file
    println!("Overwriting {}", path.display());
    let buf = zip.finish()?;
    let mut out = File::create(path)?;
    out.write_all(&buf.into_inner())?;

    Ok(())
}

fn merge_comic_info(old: &ComicInfo, new: &ComicInfo) -> ComicInfo {
    ComicInfo {
        title: new.title.clone().or_else(|| old.title.clone()),
        series: new.series.clone().or_else(|| old.series.clone()),
        language_iso: new.language_iso.clone().or_else(|| old.language_iso.clone()),
        ..old.clone()
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
