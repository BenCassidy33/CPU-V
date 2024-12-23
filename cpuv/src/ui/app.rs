use egui_code_editor::{ColorTheme, Syntax};
use egui_file_dialog::FileDialog;
use std::{default, sync::mpsc, time::Duration};

use egui::Ui;

use super::{center_pannel, sidebar, text_editor};
use crate::{
    core::engine::{ClientCommands, EngineData, StdLogMessage},
    FPS,
};

#[derive(PartialEq, Debug, Default)]
pub enum ParsingResultViewOptions {
    Raw,
    #[default]
    Tabled,
    None,
}

#[derive(Default)]
pub struct UiOptions {
    pub parsing_results: ParsingResultViewOptions,
}

pub struct UiApp {
    pub data_recv: mpsc::Receiver<EngineData>,
    pub command_sender: mpsc::Sender<ClientCommands>,
    pub stdlog_reciever: mpsc::Receiver<StdLogMessage>,

    pub previous_data: EngineData,
    pub sidebar_shown: bool,

    file_dialog: FileDialog,
    file_path: Option<String>,

    pub code: String,

    pub system_logs: Vec<String>,
    pub stdout: Vec<String>,

    pub ui_opts: UiOptions,
}

impl UiApp {
    pub fn new(
        command_sender: mpsc::Sender<ClientCommands>,
        data_recv: mpsc::Receiver<EngineData>,
        stdlog_reciever: mpsc::Receiver<StdLogMessage>,
    ) -> Self {
        return Self {
            data_recv,
            command_sender,
            stdlog_reciever,
            sidebar_shown: true,
            previous_data: EngineData::default(),
            code: "".to_string(),
            file_dialog: FileDialog::new(),
            file_path: None,
            system_logs: Vec::from(vec!["".to_string()]),
            stdout: Vec::from(vec!["".to_string()]),
            ui_opts: Default::default(),
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
                self.code = file;
            }
        }
    }
}

impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(1000 / FPS));
        if let Ok(data) = self.data_recv.try_recv() {
            self.previous_data = data;
        }

        sidebar::render(self, ctx);
        text_editor::render(self, ctx);
        center_pannel::render(self, ctx);
    }
}
