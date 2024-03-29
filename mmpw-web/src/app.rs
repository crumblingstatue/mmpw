use eframe::{
    egui::{self, Button, Color32, Label, ScrollArea, TextEdit, Visuals},
    epi,
};
use mmpw_gen::{WordError, WordPrepareError};
use mmpw_validate::binstring;
use std::fmt::Write;

#[derive(Default)]
pub struct App {
    passwords: String,
    words: String,
    name: String,
    err: Option<WordPrepareError>,
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Monster mind password tool"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        ctx.set_visuals(Visuals::dark());
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self {
            passwords,
            words,
            name,
            err,
        } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Monster Mind Password tool");
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(name);
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("6 letter words");
                if ui.button("Clear").clicked() {
                    words.clear();
                }
                if ui.button("Example words").clicked() {
                    *words = include_str!("words.txt").into();
                }
            });
            ScrollArea::from_max_height(240.)
                .id_source("input_area")
                .show(ui, |ui| {
                    let te = TextEdit::multiline(words)
                        .desired_width(720.)
                        .desired_rows(16)
                        .code_editor();
                    ui.add(te);
                });
            if ui
                .add(Button::new("Find passwords").enabled(!name.is_empty() && !words.is_empty()))
                .clicked()
            {
                match generate(name, words) {
                    Ok(pws) => {
                        *passwords = pws;
                        *err = None;
                    }
                    Err(e) => *err = Some(e),
                }
            }
            ui.separator();
            let n = passwords.lines().count();
            let mut buf;
            let text = match err {
                Some(WordPrepareError { error: err, word }) => {
                    buf = format!("Word '{}' is incorrect: ", word);
                    match err {
                        WordError::InvalidLength => buf.push_str("Invalid word length: Must be 6"),
                        WordError::InvalidChar(c) => {
                            let _ = write!(buf, "Invalid character: {}", *c as char);
                        }
                    }
                    &buf
                }
                None => {
                    if n == 0 {
                        "no password found"
                    } else {
                        buf = format!("{} passwords found for {}", n, name);
                        &buf
                    }
                }
            };
            let mut label = Label::new(text);
            if err.is_some() {
                label = label.text_color(Color32::RED);
            }
            ui.add(label);
            ScrollArea::from_max_height(240.).show(ui, |ui| {
                let te = TextEdit::multiline(passwords)
                    .id_source("output_area")
                    .desired_width(720.)
                    .desired_rows(16)
                    .code_editor();
                ui.add(te);
            });
        });
    }
}

pub fn generate(name: &str, words: &str) -> Result<String, WordPrepareError> {
    let prepared_words = mmpw_gen::prepare_words(words.split_whitespace())?;
    let mut buf = String::new();
    let key = binstring::hash_name(name.as_bytes());
    mmpw_gen::permutate(&key, &prepared_words, name, |pw, _name| {
        let s = std::str::from_utf8(pw).unwrap();
        buf += &s[0..6];
        buf += " ";
        buf += &s[6..12];
        buf += " ";
        buf += &s[12..18];
        buf += "\n";
    });
    Ok(buf)
}
