use crate::midi::consts::{BULK_DUMP_REPLY, EOX, MIDI_TUNING, SYSEX, UNIVERSAL_NON_REAL_TIME};
use crate::midi::midi_frequency::MidiFrequency;
use anyhow::{bail, Result};
use std::io::{Bytes, Read};

macro_rules! read {
    ($iter: expr) => {
        $iter
            .next()
            .ok_or_else(|| ::anyhow::anyhow!("Failed to read byte"))?
    };
    ($iter: expr, $count: expr) => {{
        let mut result = Vec::with_capacity($count);
        for _ in 0..$count {
            result.push(
                $iter
                    .next()
                    .ok_or_else(|| ::anyhow::anyhow!("Failed to read byte"))?,
            );
        }
        result
    }};
}

struct ChecksumCalculator {
    count: i32,
    value: u8,
}

impl ChecksumCalculator {
    fn new(value: u8) -> Self {
        Self { count: 0, value }
    }

    #[must_use]
    fn update(&mut self, value: u8) -> u8 {
        self.count += 1;
        self.value ^= value;
        value
    }

    fn finalize(self, expected_count: Option<i32>) -> Result<u8> {
        if let Some(expected_count) = expected_count {
            if expected_count != self.count {
                bail!("Checksum item count was incorrect")
            }
        }
        Ok(self.value & 0x7f)
    }
}

#[derive(Debug)]
pub(crate) struct BulkTuningDumpReply {
    #[allow(unused)]
    device_id: u8,

    #[allow(unused)]
    program_number: u8,

    #[allow(unused)]
    name: String,

    #[allow(unused)]
    frequencies: Vec<MidiFrequency>,

    #[allow(unused)]
    checksum: u8,
}

impl BulkTuningDumpReply {
    #[allow(unused)]
    #[must_use]
    pub(crate) const fn device_id(&self) -> u8 {
        self.device_id
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) const fn program_number(&self) -> u8 {
        self.program_number
    }
}

impl BulkTuningDumpReply {
    // https://midi.org/midi-tuning-updated-specification
    pub(crate) fn from_bytes<R: Read>(bytes: Bytes<R>) -> Result<Self> {
        let mut calc = ChecksumCalculator::new(0xff);

        let mut iter = bytes.filter_map(Result::<_, _>::ok).peekable();

        if read!(iter) != SYSEX {
            bail!("Unsupported header");
        }

        if calc.update(read!(iter)) != UNIVERSAL_NON_REAL_TIME {
            bail!("Unsupported header");
        }

        let device_id = calc.update(read!(iter));

        if calc.update(read!(iter)) != MIDI_TUNING {
            bail!("Expected MIDI Tuning")
        }

        if calc.update(read!(iter)) != BULK_DUMP_REPLY {
            bail!("Expected Bulk Dump reply")
        }

        let program_number = calc.update(read!(iter));

        let name_bytes = read!(iter, 16);

        for b in &name_bytes {
            _ = calc.update(*b)
        }

        let name = String::from(String::from_utf8(name_bytes)?.trim());

        let frequencies = (0..128)
            .map(|_| {
                let xx = read!(iter);
                let yy = read!(iter);
                let zz = read!(iter);
                Ok(MidiFrequency::new(xx, yy, zz)?)
            })
            .collect::<Result<Vec<_>>>()?;

        for f in &frequencies {
            _ = calc.update(f.xx());
            _ = calc.update(f.yy());
            _ = calc.update(f.zz());
        }

        let checksum = read!(iter);

        if read!(iter) != EOX {
            bail!("EOX not found");
        }

        let actual_checksum = calc.finalize(Some(405))?;
        if actual_checksum != checksum {
            bail!("Checksum validation failed")
        }

        Ok(Self {
            device_id,
            program_number,
            name,
            frequencies,
            checksum,
        })
    }

    #[allow(unused)]
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.push(SYSEX);
        bytes.push(UNIVERSAL_NON_REAL_TIME);
        bytes.push(self.device_id);
        bytes.push(MIDI_TUNING);
        bytes.push(BULK_DUMP_REPLY);
        bytes.push(self.program_number);

        let name_bytes = self.name.as_bytes();
        let name_bytes_len = name_bytes.len();
        if name_bytes_len > 16 {
            bail!("Invalid name");
        }

        bytes.extend(name_bytes);
        for _ in 0..(16 - name_bytes_len) {
            bytes.push(0x00);
        }

        if self.frequencies.len() != 128 {
            bail!("Invalid number of records");
        }

        for record in &self.frequencies {
            bytes.push(record.xx());
            bytes.push(record.yy());
            bytes.push(record.zz());
        }

        bytes.push(self.checksum);
        bytes.push(EOX);

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::midi::bulk_tuning_dump_reply::BulkTuningDumpReply;
    use crate::resources::RESOURCE_DIR;
    use anyhow::{anyhow, Result};
    use std::io::Read;

    #[test]
    fn basics() -> Result<()> {
        let bytes = RESOURCE_DIR
            .get_file("syx/carlos_super.syx")
            .ok_or_else(|| anyhow!("Could not load tuning dump"))?
            .contents();
        let reply = BulkTuningDumpReply::from_bytes(bytes.bytes())?;
        let output = reply.to_bytes()?;
        assert_eq!(bytes, output);
        Ok(())
    }
}
