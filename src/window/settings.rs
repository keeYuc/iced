/// The window settings of an application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    /// The initial size of the window.
    pub size: (u32, u32),

    /// The minimum size of the window.
    pub min_size: Option<(u32, u32)>,

    /// The maximum size of the window.
    pub max_size: Option<(u32, u32)>,

    /// Whether the window should be resizable or not.
    pub resizable: bool,

    /// Whether the window should have a border, a title bar, etc. or not.
    pub decorations: bool,

    /// The window icon, which is also usually used in the taskbar
    pub icon: Option<iced_winit::settings::Icon>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            size: (1024, 768),
            min_size: None,
            max_size: None,
            resizable: true,
            decorations: true,
            icon: None,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Settings> for iced_winit::settings::Window {
    fn from(settings: Settings) -> Self {
        Self {
            size: settings.size,
            min_size: settings.min_size,
            max_size: settings.max_size,
            resizable: settings.resizable,
            decorations: settings.decorations,
            icon: settings.icon,
            platform_specific: Default::default(),
        }
    }
}
