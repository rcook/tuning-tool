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
    base_note_number: NoteNumber,
    base_frequency: Frequency,
) -> Frequencies {
    let start = 0f64;
    let end = 128f64;
    let scale_size = scale.intervals().len() as f64;
    let equave_ratio = scale.equave_ratio().0;

    // Deal with implicit 1/1 and centre on base note number
    let base = base_note_number.0.as_int() as f64;
    let low = start - 1f64 - base + 1f64;
    let high = end - 1f64 - base + 1f64;
    let numEquaves = (low / scale_size).floor();
    let mut referenceFrequency = base_frequency.0 * equave_ratio.powf(numEquaves);
    let mut result = Vec::new();

    let unison = Interval::unison();
    let intervals = once(&unison)
        .chain(scale.intervals().iter().take(scale.intervals().len() - 1))
        .collect::<Vec<_>>();

    let index = (low as i32 - numEquaves as i32 * scale_size as i32);
    assert!(index >= 0);
    let mut index = index as usize;
    for _ in start as usize..end as usize {
        result.push(Frequency(referenceFrequency * intervals[index].to_f64()));
        index += 1;
        if index >= scale.interval_count() {
            index -= scale.interval_count();
            referenceFrequency *= equave_ratio;
        }
    }
    result.try_into().expect("TBD")
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

        check_frequencies(&EXPECTED_FREQUENCIES, NoteNumber::ZERO, Frequency::MIDI_MIN)?;
        Ok(())
    }

    #[test]
    fn base_note_number_69() -> Result<()> {
        const EXPECTED_FREQUENCIES: [f64; 128] = [
            8.25f64,
            8.59375f64,
            9.166666666666666f64,
            9.453125f64,
            10.3125f64,
            11.171875f64,
            11.458333333333334f64,
            12.03125f64,
            12.890625f64,
            13.75f64,
            14.609375f64,
            15.46875f64,
            16.5f64,
            17.1875f64,
            18.333333333333332f64,
            18.90625f64,
            20.625f64,
            22.34375f64,
            22.916666666666668f64,
            24.0625f64,
            25.78125f64,
            27.5f64,
            29.21875f64,
            30.9375f64,
            33f64,
            34.375f64,
            36.666666666666664f64,
            37.8125f64,
            41.25f64,
            44.6875f64,
            45.833333333333336f64,
            48.125f64,
            51.5625f64,
            55f64,
            58.4375f64,
            61.875f64,
            66f64,
            68.75f64,
            73.33333333333333f64,
            75.625f64,
            82.5f64,
            89.375f64,
            91.66666666666667f64,
            96.25f64,
            103.125f64,
            110f64,
            116.875f64,
            123.75f64,
            132f64,
            137.5f64,
            146.66666666666666f64,
            151.25f64,
            165f64,
            178.75f64,
            183.33333333333334f64,
            192.5f64,
            206.25f64,
            220f64,
            233.75f64,
            247.5f64,
            264f64,
            275f64,
            293.3333333333333f64,
            302.5f64,
            330f64,
            357.5f64,
            366.6666666666667f64,
            385f64,
            412.5f64,
            440f64,
            467.5f64,
            495f64,
            528f64,
            550f64,
            586.6666666666666f64,
            605f64,
            660f64,
            715f64,
            733.3333333333334f64,
            770f64,
            825f64,
            880f64,
            935f64,
            990f64,
            1056f64,
            1100f64,
            1173.3333333333333f64,
            1210f64,
            1320f64,
            1430f64,
            1466.6666666666667f64,
            1540f64,
            1650f64,
            1760f64,
            1870f64,
            1980f64,
            2112f64,
            2200f64,
            2346.6666666666665f64,
            2420f64,
            2640f64,
            2860f64,
            2933.3333333333335f64,
            3080f64,
            3300f64,
            3520f64,
            3740f64,
            3960f64,
            4224f64,
            4400f64,
            4693.333333333333f64,
            4840f64,
            5280f64,
            5720f64,
            5866.666666666667f64,
            6160f64,
            6600f64,
            7040f64,
            7480f64,
            7920f64,
            8448f64,
            8800f64,
            9386.666666666666f64,
            9680f64,
            10560f64,
            11440f64,
            11733.333333333334f64,
            12320f64,
        ];

        check_frequencies(
            &EXPECTED_FREQUENCIES,
            NoteNumber::A4,
            NoteNumber::A4.to_frequency(),
        )?;
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

    fn check_frequencies(
        expected_frequencies: &[f64],
        base_note_number: NoteNumber,
        base_frequency: Frequency,
    ) -> Result<()> {
        let (_, scala_file) = read_test_data()?;
        let scale = scala_file.scale();
        let frequencies = calculate_frequencies(scale, base_note_number, base_frequency);

        assert_eq!(expected_frequencies.len(), frequencies.len());
        for (expected, actual) in zip(expected_frequencies, frequencies) {
            assert_eq!(*expected, actual.0)
        }

        Ok(())
    }
}
