use crate::frequency::Frequency;
use crate::interval::Interval;
use crate::note_number::NoteNumber;
use crate::scale::Scale;
use num::Zero;
use std::iter::once;
use std::iter::zip;
use std::ops::Rem;

pub(crate) type Frequencies = [Frequency; 128];

pub(crate) fn calculate_frequencies(
    scale: &Scale,
    _base_note_number: NoteNumber,
    base_frequency: Frequency,
) -> Frequencies {
    let equave_ratio = scale.equave_ratio();
    assert_eq!(2f64, equave_ratio.0); // TBD: Haven't tested with anything other than octave-repeating scales!
    let scale_size = scale.intervals().len();
    let unison = Interval::unison();
    let intervals = once(&unison).chain(scale.intervals().iter().take(scale_size - 1));

    let mut reference_frequency = base_frequency;
    let mut frequencies = [Frequency(0f64); 128];
    for (i, interval) in zip(0..=127, intervals.cycle()) {
        if i > 0 && i.rem(scale_size).is_zero() {
            reference_frequency = Frequency(reference_frequency.0 * equave_ratio.0);
        }
        frequencies[i] = Frequency(reference_frequency.0 * interval.to_f64());
    }
    frequencies
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use crate::bulk_dump_reply::BulkDumpReply;
    use crate::consts::U7_ZERO;
    use crate::frequencies::calculate_frequencies;
    use crate::frequency::Frequency;
    use crate::note_number::NoteNumber;
    use crate::resources::RESOURCE_DIR;
    use crate::scala_file::ScalaFile;
    use anyhow::{anyhow, Result};
    use midly::num::u7;

    #[test]
    fn basics() -> Result<()> {
        let (ref_bytes, scala_file) = read_test_data()?;
        let scale = scala_file.scale();

        let entries = calculate_frequencies(scale, NoteNumber::ZERO, Frequency::MIDI_MIN)
            .map(|f| f.to_mts_entry());

        let reply = BulkDumpReply::new(
            U7_ZERO,
            u7::from_int_lossy(8),
            "carlos_super.mid".parse()?,
            entries,
        )?;

        let bytes = reply.to_bytes_with_start_and_end()?;
        assert_eq!(ref_bytes, bytes);
        Ok(())
    }

    #[test]
    fn base_note_number_0() -> Result<()> {
        const EXPECTED_FREQUENCIES: [f64; 128] = [
            8.175798915643707f64,
            8.686786347871438f64,
            9.19777378009917f64,
            9.810958698772447f64,
            10.219748644554635f64,
            10.901065220858275f64,
            11.241723509010097f64,
            12.26369837346556f64,
            13.285673237921024f64,
            13.626331526072846f64,
            14.307648102376486f64,
            15.32962296683195f64,
            16.351597831287414f64,
            17.373572695742876f64,
            18.39554756019834f64,
            19.621917397544895f64,
            20.43949728910927f64,
            21.80213044171655f64,
            22.483447018020193f64,
            24.52739674693112f64,
            26.57134647584205f64,
            27.252663052145692f64,
            28.615296204752973f64,
            30.6592459336639f64,
            32.70319566257483f64,
            34.74714539148575f64,
            36.79109512039668f64,
            39.24383479508979f64,
            40.87899457821854f64,
            43.6042608834331f64,
            44.96689403604039f64,
            49.05479349386224f64,
            53.1426929516841f64,
            54.505326104291385f64,
            57.230592409505945f64,
            61.3184918673278f64,
            65.40639132514966f64,
            69.4942907829715f64,
            73.58219024079337f64,
            78.48766959017958f64,
            81.75798915643708f64,
            87.2085217668662f64,
            89.93378807208077f64,
            98.10958698772448f64,
            106.2853859033682f64,
            109.01065220858277f64,
            114.46118481901189f64,
            122.6369837346556f64,
            130.8127826502993f64,
            138.988581565943f64,
            147.16438048158673f64,
            156.97533918035916f64,
            163.51597831287415f64,
            174.4170435337324f64,
            179.86757614416155f64,
            196.21917397544897f64,
            212.5707718067364f64,
            218.02130441716554f64,
            228.92236963802378f64,
            245.2739674693112f64,
            261.6255653005986f64,
            277.977163131886f64,
            294.32876096317347f64,
            313.9506783607183f64,
            327.0319566257483f64,
            348.8340870674648f64,
            359.7351522883231f64,
            392.43834795089793f64,
            425.1415436134728f64,
            436.0426088343311f64,
            457.84473927604756f64,
            490.5479349386224f64,
            523.2511306011972f64,
            555.954326263772f64,
            588.6575219263469f64,
            627.9013567214366f64,
            654.0639132514966f64,
            697.6681741349296f64,
            719.4703045766462f64,
            784.8766959017959f64,
            850.2830872269456f64,
            872.0852176686622f64,
            915.6894785520951f64,
            981.0958698772448f64,
            1046.5022612023945f64,
            1111.908652527544f64,
            1177.3150438526939f64,
            1255.8027134428733f64,
            1308.1278265029932f64,
            1395.3363482698592f64,
            1438.9406091532924f64,
            1569.7533918035917f64,
            1700.566174453891f64,
            1744.1704353373243f64,
            1831.3789571041902f64,
            1962.1917397544896f64,
            2093.004522404789f64,
            2223.817305055088f64,
            2354.6300877053877f64,
            2511.6054268857465f64,
            2616.2556530059865f64,
            2790.6726965397183f64,
            2877.8812183065847f64,
            3139.5067836071835f64,
            3401.132348907782f64,
            3488.3408706746486f64,
            3662.7579142083805f64,
            3924.3834795089792f64,
            4186.009044809578f64,
            4447.634610110176f64,
            4709.260175410775f64,
            5023.210853771493f64,
            5232.511306011973f64,
            5581.345393079437f64,
            5755.7624366131695f64,
            6279.013567214367f64,
            6802.264697815564f64,
            6976.681741349297f64,
            7325.515828416761f64,
            7848.7669590179585f64,
            8372.018089619156f64,
            8895.269220220353f64,
            9418.52035082155f64,
            10046.421707542986f64,
            10465.022612023946f64,
            11162.690786158873f64,
            11511.524873226339f64,
            12558.027134428734f64,
        ];

        let (_, scala_file) = read_test_data()?;
        let scale = scala_file.scale();
        let frequencies = calculate_frequencies(scale, NoteNumber::ZERO, Frequency::MIDI_MIN);

        assert_eq!(EXPECTED_FREQUENCIES.len(), frequencies.len());
        for (expected, actual) in zip(EXPECTED_FREQUENCIES, frequencies) {
            assert_eq!(expected, actual.0)
        }

        Ok(())
    }

    fn read_test_data() -> Result<(Vec<u8>, ScalaFile)> {
        let ref_bytes = RESOURCE_DIR
            .get_file("syx/carlos_super.syx")
            .ok_or_else(|| anyhow!("Could not load tuning dump"))?
            .contents()
            .to_vec();
        let scl_file = RESOURCE_DIR
            .get_file("scl/carlos_super.scl")
            .ok_or_else(|| anyhow!("Could not get scl file"))?;
        let s = scl_file
            .contents_utf8()
            .ok_or_else(|| anyhow!("Could not convert to string"))?;
        let scala_file = s.parse::<ScalaFile>()?;
        Ok((ref_bytes, scala_file))
    }
}
