use std::net::IpAddr;

/// Application.
#[derive(Debug, Default)]
pub struct App {
    // should the application exit?
    pub should_quit: bool,
    // selected network in Activation menu
    pub selected_network: usize,
    // what in focus statusbar or what
    pub focus: Focus,
    // screens
    pub screen: Screen,
    // overlays
    pub overlay: Overlay,
    // Connection state
    pub connection: ConnectionState,
    // Vector of errors
    pub notifications: Vec<crate::event::Event>,
    // vector of wifi networks
    pub networks: Vec<Network>,
}

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Activation,
    Menu,
    KnownNetworks,
    NetworkDetails,
    Interfaces,
}

#[derive(Debug, Default)]
pub enum Overlay {
    #[default]
    None,
    PasswordInput,
    ConfirmDisconnect,
}

#[derive(Debug, Default)]
pub enum ConnectionState {
    #[default]
    Disconnected,
    Connecting,
    Connected(ConnectionInfo),
}
#[derive(Debug, Default)]
pub enum Focus {
    #[default]
    NetworkList,
    StatusBar,
}
#[derive(Debug, Default)]
pub struct ConnectionInfo {
    pub interface: String,
    pub network: String,
    pub ip: Option<IpAddr>,
}
#[derive(Debug, Default)]
pub struct Network {
    pub ssid: String,
    pub signal_dbm: i32,
    pub known: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
