pub(crate) enum Mode {
    Edit,
    Normal,
    Command,
}

impl Mode {
    pub(crate) fn stringify(&self) -> String {
        match self {
            Mode::Edit => "Edit".to_string(),
            Mode::Normal => "Normal".to_string(),
            Mode::Command => "Command".to_string(),
        }
    }
}
