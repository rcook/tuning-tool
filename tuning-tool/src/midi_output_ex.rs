// Workaround for https://github.com/Boddlnagg/midir/issues/55

use midir::{ConnectError, MidiOutput, MidiOutputConnection, MidiOutputPort};
use std::result::Result as StdResult;

pub(crate) trait MidiOutputEx {
    fn connect_ex(
        self,
        port: &MidiOutputPort,
        port_name: &str,
    ) -> StdResult<MidiOutputConnection, ConnectError<()>>;
}

impl MidiOutputEx for MidiOutput {
    fn connect_ex(
        self,
        port: &MidiOutputPort,
        port_name: &str,
    ) -> StdResult<MidiOutputConnection, ConnectError<()>> {
        self.connect(port, port_name)
            .map_err(|e| ConnectError::new(e.kind(), ()))
    }
}
