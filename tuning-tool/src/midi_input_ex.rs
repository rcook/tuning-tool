// Workaround for https://github.com/Boddlnagg/midir/issues/55

use midir::{ConnectError, MidiInput, MidiInputConnection, MidiInputPort};
use std::result::Result as StdResult;

pub(crate) trait MidiInputEx {
    fn connect_ex<F, T: Send>(
        self,
        port: &MidiInputPort,
        port_name: &str,
        callback: F,
        data: T,
    ) -> StdResult<MidiInputConnection<T>, ConnectError<()>>
    where
        F: FnMut(u64, &[u8], &mut T) + Send + 'static;
}

impl MidiInputEx for MidiInput {
    fn connect_ex<F, T: Send>(
        self,
        port: &MidiInputPort,
        port_name: &str,
        callback: F,
        data: T,
    ) -> StdResult<MidiInputConnection<T>, ConnectError<()>>
    where
        F: FnMut(u64, &[u8], &mut T) + Send + 'static,
    {
        self.connect(port, port_name, callback, data)
            .map_err(|e| ConnectError::new(e.kind(), ()))
    }
}
