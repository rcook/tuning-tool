// Copyright (c) 2024 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

use crate::types::MidiValue;

pub(crate) const SYSEX: u8 = 0xf0;
pub(crate) const UNIVERSAL_REAL_TIME: MidiValue = MidiValue::constant::<0x7f>();
pub(crate) const UNIVERSAL_NON_REAL_TIME: MidiValue = MidiValue::constant::<0x7e>();
pub(crate) const MIDI_TUNING: MidiValue = MidiValue::constant::<8>();
pub(crate) const NOTE_CHANGE: MidiValue = MidiValue::constant::<2>();
pub(crate) const BULK_DUMP_REPLY: MidiValue = MidiValue::constant::<1>();
pub(crate) const BULK_DUMP_REPLY_CHECKSUM_COUNT: usize = 405;

#[cfg(test)]
pub(crate) const BULK_DUMP_REPLY_MESSAGE_SIZE: usize = BULK_DUMP_REPLY_CHECKSUM_COUNT + 1;

pub(crate) const EOX: u8 = 0xf7;
