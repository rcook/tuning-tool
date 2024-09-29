use crate::consts::{BULK_DUMP_REPLY, EOX, MIDI_TUNING, UNIVERSAL_NON_REAL_TIME};
use crate::hex_dump::{hex_dump, HexDumpOptionsBuilder};
use crate::string_extras::StringExtras;
use anyhow::Result;
use core::str;
use midly::num::{u28, u7};
use midly::{Smf, TrackEventKind};
use std::slice::Iter;

macro_rules! pull_u7 {
    ($iter: expr) => {
        std::convert::TryInto::<midly::num::u7>::try_into(pull_u8!($iter)).expect("TBD")
    };
}

macro_rules! pull_u8 {
    ($iter: expr) => {{
        let value: u8 = $iter.next().cloned().expect("TBD");
        value
    }};
}

#[derive(Debug)]
pub(crate) struct SysExEvent<'a> {
    delta: u28,
    data: &'a [u8],
}

impl<'a> SysExEvent<'a> {
    pub(crate) fn find_all<'b>(smf: &'b Smf) -> Vec<SysExEvent<'b>> {
        let mut events = Vec::new();
        for track in &smf.tracks {
            for event in track {
                if let TrackEventKind::SysEx(data) = &event.kind {
                    events.push(SysExEvent {
                        delta: event.delta,
                        data,
                    })
                }
            }
        }

        events
    }

    pub(crate) fn dump(&self) {
        println!("SysEx: delta {delta:08X}", delta = u32::from(self.delta));
        hex_dump(self.data);
    }

    pub(crate) fn decode(&self) {
        let mut iter = self.data.iter();

        if pull_u7!(iter) == UNIVERSAL_NON_REAL_TIME {
            Self::decode_non_real_time_sysex(iter)
        } else {
            todo!()
        }
    }

    fn decode_non_real_time_sysex(mut iter: Iter<'_, u8>) {
        let device_id = pull_u7!(iter);
        println!("Device ID: {device_id}");

        if pull_u7!(iter) == MIDI_TUNING {
            Self::decode_midi_tuning(iter);
        } else {
            todo!()
        }
    }

    fn decode_midi_tuning(mut iter: Iter<'_, u8>) {
        if pull_u7!(iter) == BULK_DUMP_REPLY {
            Self::decode_bulk_dump_reply(iter)
        }
    }

    fn decode_bulk_dump_reply(mut iter: Iter<'_, u8>) {
        let program_number = pull_u7!(iter);
        println!("Program number: {program_number}");

        let name = Self::read_str(&mut iter, 16);
        println!("Name: {name}");

        for i in 0..128 {
            let xx = pull_u7!(iter);
            let yy = pull_u7!(iter);
            let zz = pull_u7!(iter);
            println!("Note {i}: {xx} {yy} {zz}")
        }

        let checksum = pull_u7!(iter);
        println!("Checksum: {checksum}");

        assert_eq!(EOX, pull_u8!(iter));
        assert!(iter.next().is_none());
    }

    fn read_str(iter: &mut Iter<'_, u8>, len: usize) -> String {
        let mut values = Vec::with_capacity(len);
        for _ in 0..len {
            values.push(pull_u7!(iter));
        }
        String::from_u7_slice(&values)
    }
}
