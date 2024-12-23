use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
#[serde(default)]
pub struct RootConfig {
    pub program: ProgramConfig,
    pub engine: EngineConfig,
    pub ui: UIConfig,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ProgramConfig {
    pub program_path: String,
    pub output_parsing_results: ParsingResultsConfig,
    pub output_logs: LogsConfig,
    pub stdout: Vec<StdoutOption>,
}

impl Default for ProgramConfig {
    fn default() -> Self {
        Self {
            program_path: "./main.cpu".to_string(),
            output_parsing_results: Default::default(),
            output_logs: Default::default(),
            stdout: vec![StdoutOption::r#virtual],
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ParsingResultsConfig {
    pub should_write: bool,
    pub path: String,
}

impl Default for ParsingResultsConfig {
    fn default() -> Self {
        Self {
            should_write: false,
            path: "out/parsed.ptree".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct LogsConfig {
    pub should_write: bool,
    pub path: String,
}

impl Default for LogsConfig {
    fn default() -> Self {
        Self {
            should_write: false,
            path: "logs.txt".to_string(),
        }
    }
}

#[allow(warnings)]
#[derive(Default, Deserialize, Debug)]
pub enum StdoutOption {
    All,
    #[default]
    r#virtual,
    console,
    file,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct EngineConfig {
    pub tps: usize,
    pub ipt: usize,
    pub heap_memory_size: usize,
    pub heap_access_simulation: HeapAccessSimulationConfig,
    pub log_level: LogLevel,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            tps: 5,
            ipt: 1,
            heap_memory_size: 2048,
            heap_access_simulation: Default::default(),
            log_level: Default::default(),
        }
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct HeapAccessSimulationConfig {
    pub enabled: bool,
    pub minimum_delay: u32,
}

#[derive(Default, Deserialize, Debug)]
pub enum LogLevel {
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct UIConfig {
    pub theme: UITheme,
    pub font_color: String,
    pub font_size: u32,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            theme: UITheme::dark,
            font_color: "default".to_string(), //TODO: FIXME
            font_size: 16,
        }
    }
}

#[allow(warnings)]
#[derive(Default, Deserialize, Debug)]
pub enum UITheme {
    light,
    #[default]
    dark,
}
