use super::types::{OperationalState, RobotMode, RobotState};
use crate::prelude::*;

impl UniversalRobot {
    /// Request several status metrics from the robot
    pub fn get_meta_data(&mut self) -> Result<RobotState> {
        let port = &mut self.dashboard;
        let (is_saved, program) = port.is_saved()?;
        Ok(RobotState {
            program,
            is_saved,
            version: port.get_version()?,
            mode: port.get_mode()?,
            is_remote: port.is_remote_mode()?,
            serial: port.get_version()?,
            model: port.get_model()?,
            operational_mode: port.get_op_mode()?,
            safety_state: port.safety_status()?,
        })
    }
    /// Request operational status from the Robot
    pub fn get_state(&mut self) -> Result<OperationalState> {
        let port = &mut self.dashboard;
        Ok(OperationalState {
            mode: port.get_mode()?,
            state: port.get_program_state()?,
        })
    }
    /// Wait on program to be loaded
    pub fn load(&mut self, program: &str, timeout: Duration) -> Result<()> {
        let port = &mut self.dashboard;
        let loaded = port.get_loaded_program()?;
        if loaded == program {
            return Ok(());
        }
        port.load_program(program)?;
        let now = Instant::now();
        while port.get_loaded_program()? != program {
            if now.elapsed() > timeout {
                return Err(Error::Timeout(program.to_owned(), now.elapsed().as_secs()));
            }
            sleep(Duration::from_millis(1));
        }
        Ok(())
    }
    /// Wait on power up or timeout
    pub fn power_on(&mut self, timeout: Duration) -> Result<()> {
        let port = &mut self.dashboard;
        let on_states = [RobotMode::PowerOn, RobotMode::Idle, RobotMode::Running];
        // check if we're already powered on
        if on_states.contains(&port.get_mode()?) {
            return Ok(());
        }
        port.power(true)?;
        let now = Instant::now();
        loop {
            let mode = port.get_mode()?;
            if on_states.contains(&port.get_mode()?) {
                break Ok(());
            } else if now.elapsed() > timeout {
                break Err(Error::Timeout(format!("{mode:?}"), now.elapsed().as_secs()));
            }
            sleep(Duration::from_millis(1));
        }
    }
}
