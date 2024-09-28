use crate::frequency::Frequency;
use crate::note_number::NoteNumber;
use crate::num::round_default_scale;
use crate::semitones::Semitones;

#[derive(Debug, PartialEq)]
pub(crate) struct MtsBytes {
    pub(crate) note_number: NoteNumber,
    pub(crate) yy: u8,
    pub(crate) zz: u8,
}

impl MtsBytes {
    // c.f. mtsBytesToMts
    pub(crate) fn to_semitones(&self) -> Semitones {
        let msb = if self.yy > 0x7f { 0x7f } else { self.yy };
        let mut lsb = self.zz;
        let note_number = if self.note_number.0 > 0x7f {
            0x7f
        } else {
            self.note_number.0 as u8
        };

        if note_number == 0x7f {
            if lsb >= 0x7f {
                lsb = 0x7e;
            }
        } else if lsb > 0x7f {
            lsb = 0x7f;
        }

        let fine = (((msb as u16) << 7) + lsb as u16) as f64 / 0x4000 as f64;
        Semitones(note_number as f64 + fine)
    }

    // c.f. mtsBytesToFrequency
    pub(crate) fn to_frequency(&self) -> Frequency {
        let mts = self.to_semitones();
        println!("mts={:?}", mts.0);
        let frequency = mts.to_frequency();
        Frequency(round_default_scale(frequency.0))
    }

    // c.f. mtsBytesToHex
    pub(crate) fn to_hex(&self) -> String {
        format!(
            "{xx:02x}{yy:02x}{zz:02x}",
            xx = self.note_number.0.clamp(0, 127),
            yy = self.yy.clamp(0, 127),
            zz = self.zz.clamp(0, 127)
        )
    }
}
