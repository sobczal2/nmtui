use std::fmt::Display;

use networkmanager::devices::Device;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Command,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Mode::Normal => "NORMAL",
            Mode::Command => "COMMAND",
        };
        write!(f, "{v}")
    }
}

pub struct State<'a> {
    pub mode: Mode,
    pub command_buffer: String,
    pub error_buffer: Option<String>,
    pub exit_requested: bool,
    pub page: Page,
    pub page_state: Option<PageState<'a>>,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        State {
            mode: Mode::Normal,
            command_buffer: String::new(),
            error_buffer: None,
            exit_requested: false,
            page: Page::Devices,
            page_state: None,
        }
    }
}

pub enum Page {
    Devices,
}

pub enum PageState<'a> {
    Devices(DevicesPageState<'a>),
}

pub struct DevicesPageState<'a> {
    devices: Vec<Device<'a>>,
    loading: bool,
}
