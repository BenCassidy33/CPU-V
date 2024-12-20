use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use egui_file_dialog::FileDialog;
use std::{
    default,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use egui::{CentralPanel, Ui};

use super::{sidebar, text_editor};
use crate::{
    core::engine::{ClientCommandType, ClientCommands, EngineData},
    FPS,
};

pub struct UiApp {
    pub data_recv: mpsc::Receiver<EngineData>,
    pub command_sender: mpsc::Sender<ClientCommands>,
    pub previous_data: EngineData,
    pub sidebar_shown: bool,

    file_dialog: FileDialog,
    file_path: Option<String>,

    pub code: String,

    pub parser_result: String,
}

impl UiApp {
    pub fn new(
        data_recv: mpsc::Receiver<EngineData>,
        command_sender: mpsc::Sender<ClientCommands>,
    ) -> Self {
        return Self {
            data_recv,
            command_sender,
            sidebar_shown: true,
            previous_data: EngineData::default(),
            code: "".to_string(),
            file_dialog: FileDialog::new(),
            file_path: None,
            parser_result: "".to_string(),
        };
    }

    pub fn show_code_editor(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let aval_height = ctx.available_rect().max.y;

        egui_code_editor::CodeEditor::default()
            .id_source("Code Editor")
            .with_rows(20)
            .with_fontsize(14.0)
            .with_theme(ColorTheme::GRUVBOX_DARK)
            .with_syntax(Syntax::asm())
            .with_numlines(true)
            .show(ui, &mut self.code);
    }

    pub fn show_file_picker(&mut self, ctx: &eframe::egui::Context, ui: &mut egui::Ui) {
        if (ui.button("Open File")).clicked() {
            self.file_dialog.select_file();
        }

        if let Some(path) = self.file_dialog.update(ctx).selected() {
            if self.file_path.is_some()
                && (self.file_path.clone().unwrap() == path.to_str().unwrap())
            {
                return;
            }

            self.file_path = Some(path.to_string_lossy().to_string());

            if let Ok(file) = std::fs::read_to_string(path) {
                println!("reading file");
                self.code = file;
            }
        }
    }
}

impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(1000 / FPS));

        sidebar::render(self, ctx);
        text_editor::render(self, ctx);
    }
}
