use crate::midi::consts::{BULK_DUMP_REPLY, EOX, MIDI_TUNING, UNIVERSAL_NON_REAL_TIME};
use crate::u7::u7;
use midly::num::u28;
use midly::{Smf, TrackEventKind};
use std::slice::Iter;

macro_rules! pull_u7 {
    ($iter: expr) => {
        std::convert::TryInto::<crate::u7::u7>::try_into(pull_u8!($iter)).expect("TBD")
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
        let mut i = 0;
        const COLUMNS: usize = 32;
        let n = self.data.len();
        while i < n {
            let bound = n.min(i + COLUMNS);
            for j in i..bound {
                print!(" {byte:02X}", byte = self.data[j])
            }
            println!();
            i += COLUMNS;
        }
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
        u7::to_utf8_lossy(&values)
    }

    /*
    7E 7F 08 01 00 53 43 41 4C 41 20 54 55 4E 49 4E 47 00 00 00 00 00 00 00 01 00 00 02 00 00 03 00
    00 04 00 00 05 00 00 06 00 00 07 00 00 08 00 00 09 00 00 0A 00 00 0B 00 00 0C 00 00 0D 00 00 0E
    00 00 0F 00 00 10 00 00 11 00 00 12 00 00 13 00 00 14 00 00 15 00 00 16 00 00 17 00 00 18 00 00
    19 00 00 1A 00 00 1B 00 00 1C 00 00 1D 00 00 1E 00 00 1F 00 00 20 00 00 21 00 00 22 00 00 23 00
    00 24 00 00 25 00 00 26 00 00 27 00 00 28 00 00 29 00 00 2A 00 00 2B 00 00 2C 00 00 2D 00 00 2E
    00 00 2F 00 00 30 00 00 31 00 00 32 00 00 33 00 00 34 00 00 35 00 00 36 00 00 37 00 00 38 00 00
    39 00 00 3A 00 00 3B 00 00 3C 00 00 3D 00 00 3E 00 00 3F 00 00 40 00 00 41 00 00 42 00 00 43 00
    00 44 00 00 45 00 00 46 00 00 47 00 00 48 00 00 49 00 00 4A 00 00 4B 00 00 4C 00 00 4D 00 00 4E
    00 00 4F 00 00 50 00 00 51 00 00 52 00 00 53 00 00 54 00 00 55 00 00 56 00 00 57 00 00 58 00 00
    59 00 00 5A 00 00 5B 00 00 5C 00 00 5D 00 00 5E 00 00 5F 00 00 60 00 00 61 00 00 62 00 00 63 00
    00 64 00 00 65 00 00 66 00 00 67 00 00 68 00 00 69 00 00 6A 00 00 6B 00 00 6C 00 00 6D 00 00 6E
    00 00 6F 00 00 70 00 00 71 00 00 72 00 00 73 00 00 74 00 00 75 00 00 76 00 00 77 00 00 78 00 00
    79 00 00 7A 00 00 7B 00 00 7C 00 00 7D 00 00 7E 00 00 7F 00 00 00 F7
        */
}
