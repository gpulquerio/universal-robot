/// Robot status mode
#[derive(Debug, PartialEq)]
pub enum RobotMode {
    NoController,
    Disconnected,
    ConfirmSafety,
    Booting,
    PowerOff,
    PowerOn,
    Idle,
    Backdrive,
    Running,
}

/// Robot State slowly changing metadata
#[derive(Debug)]
pub struct RobotState {
    pub program: Option<String>,
    pub is_saved: bool,
    pub version: String,
    pub mode: RobotMode,
    pub operational_mode: Option<OpMode>,
    pub safety_state: SafetyStatus,
    pub is_remote: bool,
    pub serial: String,
    pub model: String,
}

/// Program operational state data
#[derive(Debug)]
pub struct OperationalState {
    pub mode: RobotMode,
    pub state: ProgramState,
}

/// State of the active program and path to loaded program file, or STOPPED if no program is loaded
#[derive(Debug)]
pub enum ProgramState {
    Stopped(Option<String>),
    Playing(String),
    Paused(String),
}

/// Robot Operational Mode
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OpMode {
    Manual,
    Automatic,
}

/// Robot safety status
#[derive(Debug)]
pub enum SafetyStatus {
    Normal,
    Reduced,
    ProtectiveStop,
    Recovery,
    SafeguardStop,
    SystemEmergencyStop,
    RobotEmergencyStop,
    Violation,
    Fault,
    AutomaticModeSafeguardStop,
    SystemThreePositionEnablingStop,
}
