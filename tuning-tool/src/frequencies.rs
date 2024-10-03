use crate::frequency::Frequency;
use crate::interval::Interval;
use crate::keyboard_mapping::KeyboardMapping;
use crate::scale::Scale;
use std::iter::once;
use std::iter::zip;
use std::ops::Rem;

pub(crate) fn calculate_frequencies(
    scale: &Scale,
    keyboard_mapping: &KeyboardMapping,
) -> Vec<Frequency> {
    let start = keyboard_mapping.start_note_number().to_u8() as i32;
    let end = keyboard_mapping.end_note_number().to_u8() as i32;
    let base = keyboard_mapping.base_note_number().to_u8() as i32;

    let note_count = (end - start + 1) as usize;
    let scale_size = scale.intervals().len();
    let equave_ratio = scale.equave_ratio().0;
    let low = start - base;
    let equave_count = (low as f64 / scale_size as f64).floor() as i32;
    let offset = (low - equave_count * scale_size as i32) as usize;
    let unison = Interval::unison();
    let intervals = scale
        .intervals()
        .iter()
        .take(scale_size - 1)
        .chain(once(&unison))
        .cycle()
        .skip((offset + scale_size - 1).rem(scale_size));

    let mut frequencies = Vec::with_capacity(note_count);
    let mut f = keyboard_mapping.base_frequency().0 * equave_ratio.powi(equave_count);
    let mut degree = offset;
    for (_, interval) in zip(start..=end, intervals) {
        frequencies.push(Frequency(f * interval.as_ratio().0));
        degree += 1;
        if degree >= scale_size {
            degree -= scale_size;
            f *= equave_ratio;
        }
    }

    frequencies
}

#[cfg(test)]
mod tests {
    use crate::frequencies::calculate_frequencies;
    use crate::frequency::Frequency;
    use crate::keyboard_mapping::KeyboardMapping;
    use crate::midi_note::MidiNote;
    use crate::note_number::NoteNumber;
    use crate::scale::Scale;
    use anyhow::Result;
    use std::iter::zip;
    use std::sync::LazyLock;
    use tuning_tool_macros::scale;

    static BOHLEN_P: LazyLock<Scale> = LazyLock::new(|| {
        scale![
            27/25
            25/21
            9/7
            7/5
            75/49
            5/3
            9/5
            49/25
            15/7
            7/3
            63/25
            25/9
            3/1
        ]
    });

    static SCALE_24EDO2: LazyLock<Scale> = LazyLock::new(|| {
        scale![
            50.0 100.0 150.0 200.0 250.0 300.0 350.0 400.0 450.0
            500.0 550.0 600.0 650.0 700.0 750.0 800.0 850.0 900.0
            950.0 1000.0 1050.0 1100.0 1150.0 2/1
        ]
    });

    static SCALE_12EDO2: LazyLock<Scale> = LazyLock::new(|| {
        scale![
            100.0 200.0 300.0 400.0 500.0 600.0 700.0 800.0 900.0
            1000.0 1100.0 2/1
        ]
    });

    static CARLOS_SUPER: LazyLock<Scale> = LazyLock::new(|| {
        scale![
            17/16 9/8 6/5 5/4 4/3 11/8 3/2 13/8 5/3 7/4 15/8
            2/1
        ]
    });

    #[test]
    fn bohlen_p() -> Result<()> {
        const EXPECTED_FREQUENCIES: [f64; 128] = [
            1.2933568489124043f64,
            1.4083219021490627f64,
            1.5209876543209875f64,
            1.6765736930345982f64,
            1.810699588477366f64,
            1.9555555555555555f64,
            2.1555947481873403f64,
            2.328042328042328f64,
            2.5349794238683123f64,
            2.771478961955152f64,
            3.017832647462277f64,
            3.259259259259259f64,
            3.548971193415637f64,
            3.8800705467372127f64,
            4.224965706447188f64,
            4.562962962962962f64,
            5.029721079103794f64,
            5.432098765432098f64,
            5.866666666666666f64,
            6.466784244562022f64,
            6.984126984126983f64,
            7.6049382716049365f64,
            8.314436885865456f64,
            9.05349794238683f64,
            9.777777777777777f64,
            10.646913580246911f64,
            11.64021164021164f64,
            12.674897119341562f64,
            13.688888888888886f64,
            15.089163237311382f64,
            16.296296296296294f64,
            17.599999999999998f64,
            19.400352733686063f64,
            20.952380952380953f64,
            22.81481481481481f64,
            24.943310657596367f64,
            27.16049382716049f64,
            29.33333333333333f64,
            31.940740740740736f64,
            34.92063492063492f64,
            38.02469135802469f64,
            41.06666666666666f64,
            45.26748971193415f64,
            48.888888888888886f64,
            52.8f64,
            58.201058201058196f64,
            62.85714285714286f64,
            68.44444444444443f64,
            74.82993197278911f64,
            81.48148148148148f64,
            88f64,
            95.82222222222221f64,
            104.76190476190474f64,
            114.07407407407408f64,
            123.19999999999999f64,
            135.80246913580245f64,
            146.66666666666666f64,
            158.4f64,
            174.60317460317458f64,
            188.57142857142858f64,
            205.33333333333331f64,
            224.48979591836732f64,
            244.44444444444443f64,
            264f64,
            287.46666666666664f64,
            314.2857142857143f64,
            342.22222222222223f64,
            369.59999999999997f64,
            407.4074074074074f64,
            440f64,
            475.20000000000005f64,
            523.8095238095239f64,
            565.7142857142858f64,
            616f64,
            673.469387755102f64,
            733.3333333333334f64,
            792f64,
            862.4f64,
            942.8571428571428f64,
            1026.6666666666667f64,
            1108.8f64,
            1222.2222222222222f64,
            1320f64,
            1425.6000000000001f64,
            1571.4285714285713f64,
            1697.1428571428573f64,
            1847.9999999999998f64,
            2020.408163265306f64,
            2200f64,
            2376f64,
            2587.2f64,
            2828.5714285714284f64,
            3080f64,
            3326.4f64,
            3666.6666666666665f64,
            3960f64,
            4276.8f64,
            4714.285714285715f64,
            5091.428571428572f64,
            5544f64,
            6061.224489795918f64,
            6600f64,
            7128f64,
            7761.599999999999f64,
            8485.714285714286f64,
            9240f64,
            9979.2f64,
            11000f64,
            11880f64,
            12830.400000000001f64,
            14142.857142857143f64,
            15274.285714285716f64,
            16632f64,
            18183.673469387755f64,
            19800f64,
            21384f64,
            23284.8f64,
            25457.142857142855f64,
            27720f64,
            29937.6f64,
            33000f64,
            35640f64,
            38491.200000000004f64,
            42428.57142857143f64,
            45822.857142857145f64,
            49896f64,
            54551.02040816326f64,
            59400f64,
        ];

        check_frequencies(
            &EXPECTED_FREQUENCIES,
            &BOHLEN_P,
            NoteNumber::A4,
            Frequency::A4,
        )?;
        Ok(())
    }

    #[test]
    fn scale_24edo2_a4_at_69_432_hz() -> Result<()> {
        const EXPECTED_FREQUENCIES: [f64; 128] = [
            58.88741756392392f64,
            60.61295060870614f64,
            62.38904563110274f64,
            64.21718421014694f64,
            66.09889133845138f64,
            68.03573669432315f64,
            70.02933595115452f64,
            72.08135212518185f64,
            74.19349696273682f64,
            76.36753236814714f64,
            78.60527187347812f64,
            80.9085821513408f64,
            83.2793845720288f64,
            85.71965680628277f64,
            88.2314344750194f64,
            90.81681284740117f64,
            93.47794858866344f64,
            96.21706155915665f64,
            99.03643666610449f64,
            101.9384257696229f64,
            104.92544964458943f64,
            108f64,
            111.16464155749715f64,
            114.42201419080389f64,
            117.77483512784784f64,
            121.22590121741229f64,
            124.77809126220548f64,
            128.43436842029388f64,
            132.19778267690276f64,
            136.0714733886463f64,
            140.05867190230904f64,
            144.1627042503637f64,
            148.38699392547363f64,
            152.73506473629428f64,
            157.21054374695623f64,
            161.8171643026816f64,
            166.5587691440576f64,
            171.43931361256554f64,
            176.4628689500388f64,
            181.63362569480233f64,
            186.95589717732688f64,
            192.4341231183133f64,
            198.07287333220899f64,
            203.8768515392458f64,
            209.85089928917887f64,
            216f64,
            222.3292831149943f64,
            228.84402838160779f64,
            235.54967025569567f64,
            242.45180243482457f64,
            249.55618252441096f64,
            256.86873684058776f64,
            264.3955653538055f64,
            272.1429467772926f64,
            280.1173438046181f64,
            288.3254085007274f64,
            296.77398785094726f64,
            305.47012947258855f64,
            314.42108749391247f64,
            323.6343286053632f64,
            333.1175382881152f64,
            342.8786272251311f64,
            352.9257379000776f64,
            363.26725138960467f64,
            373.91179435465375f64,
            384.8682462366266f64,
            396.14574666441797f64,
            407.7537030784916f64,
            419.70179857835774f64,
            432f64,
            444.6585662299886f64,
            457.68805676321557f64,
            471.09934051139135f64,
            484.90360486964914f64,
            499.1123650488219f64,
            513.7374736811755f64,
            528.791130707611f64,
            544.2858935545852f64,
            560.2346876092362f64,
            576.6508170014548f64,
            593.5479757018945f64,
            610.9402589451771f64,
            628.8421749878249f64,
            647.2686572107264f64,
            666.2350765762304f64,
            685.7572544502622f64,
            705.8514758001552f64,
            726.5345027792093f64,
            747.8235887093075f64,
            769.7364924732532f64,
            792.2914933288359f64,
            815.5074061569832f64,
            839.4035971567155f64,
            864f64,
            889.3171324599772f64,
            915.3761135264311f64,
            942.1986810227827f64,
            969.8072097392983f64,
            998.2247300976438f64,
            1027.474947362351f64,
            1057.582261415222f64,
            1088.5717871091704f64,
            1120.4693752184724f64,
            1153.3016340029096f64,
            1187.095951403789f64,
            1221.8805178903542f64,
            1257.6843499756499f64,
            1294.5373144214527f64,
            1332.4701531524609f64,
            1371.5145089005243f64,
            1411.7029516003104f64,
            1453.0690055584187f64,
            1495.647177418615f64,
            1539.4729849465064f64,
            1584.5829866576719f64,
            1631.0148123139663f64,
            1678.807194313431f64,
            1728f64,
            1778.6342649199544f64,
            1830.7522270528623f64,
            1884.3973620455654f64,
            1939.6144194785966f64,
            1996.4494601952877f64,
            2054.949894724702f64,
            2115.164522830444f64,
            2177.1435742183407f64,
            2240.9387504369447f64,
            2306.6032680058192f64,
        ];

        check_frequencies(
            &EXPECTED_FREQUENCIES,
            &SCALE_24EDO2,
            NoteNumber::A4,
            Frequency(432f64),
        )?;
        Ok(())
    }

    #[test]
    fn scale_12edo2_a4_at_69() -> Result<()> {
        const EXPECTED_FREQUENCIES: [f64; 128] = [
            8.175798915643707f64,
            8.661957218027252f64,
            9.177023997418987f64,
            9.722718241315029f64,
            10.300861153527185f64,
            10.913382232281371f64,
            11.562325709738575f64,
            12.249857374429665f64,
            12.978271799373285f64,
            13.75f64,
            14.56761754744031f64,
            15.433853164253879f64,
            16.351597831287414f64,
            17.323914436054505f64,
            18.354047994837973f64,
            19.445436482630058f64,
            20.60172230705437f64,
            21.826764464562743f64,
            23.12465141947715f64,
            24.49971474885933f64,
            25.95654359874657f64,
            27.5f64,
            29.13523509488062f64,
            30.867706328507758f64,
            32.70319566257483f64,
            34.64782887210901f64,
            36.70809598967595f64,
            38.890872965260115f64,
            41.20344461410874f64,
            43.653528929125486f64,
            46.2493028389543f64,
            48.99942949771866f64,
            51.91308719749314f64,
            55f64,
            58.27047018976124f64,
            61.735412657015516f64,
            65.40639132514966f64,
            69.29565774421802f64,
            73.4161919793519f64,
            77.78174593052023f64,
            82.40688922821748f64,
            87.30705785825097f64,
            92.4986056779086f64,
            97.99885899543732f64,
            103.82617439498628f64,
            110f64,
            116.54094037952248f64,
            123.47082531403103f64,
            130.8127826502993f64,
            138.59131548843604f64,
            146.8323839587038f64,
            155.56349186104046f64,
            164.81377845643496f64,
            174.61411571650194f64,
            184.9972113558172f64,
            195.99771799087463f64,
            207.65234878997256f64,
            220f64,
            233.08188075904496f64,
            246.94165062806206f64,
            261.6255653005986f64,
            277.1826309768721f64,
            293.6647679174076f64,
            311.1269837220809f64,
            329.6275569128699f64,
            349.2282314330039f64,
            369.9944227116344f64,
            391.99543598174927f64,
            415.3046975799451f64,
            440f64,
            466.1637615180899f64,
            493.8833012561241f64,
            523.2511306011972f64,
            554.3652619537442f64,
            587.3295358348151f64,
            622.2539674441618f64,
            659.2551138257398f64,
            698.4564628660078f64,
            739.9888454232688f64,
            783.9908719634985f64,
            830.6093951598903f64,
            880f64,
            932.3275230361799f64,
            987.7666025122483f64,
            1046.5022612023945f64,
            1108.7305239074883f64,
            1174.6590716696303f64,
            1244.5079348883237f64,
            1318.5102276514797f64,
            1396.9129257320155f64,
            1479.9776908465376f64,
            1567.981743926997f64,
            1661.2187903197805f64,
            1760f64,
            1864.6550460723597f64,
            1975.5332050244965f64,
            2093.004522404789f64,
            2217.4610478149766f64,
            2349.3181433392606f64,
            2489.0158697766474f64,
            2637.0204553029594f64,
            2793.825851464031f64,
            2959.955381693075f64,
            3135.963487853994f64,
            3322.437580639561f64,
            3520f64,
            3729.3100921447194f64,
            3951.066410048993f64,
            4186.009044809578f64,
            4434.922095629953f64,
            4698.636286678521f64,
            4978.031739553295f64,
            5274.040910605919f64,
            5587.651702928062f64,
            5919.91076338615f64,
            6271.926975707988f64,
            6644.875161279122f64,
            7040f64,
            7458.620184289439f64,
            7902.132820097986f64,
            8372.018089619156f64,
            8869.844191259906f64,
            9397.272573357042f64,
            9956.06347910659f64,
            10548.081821211837f64,
            11175.303405856124f64,
            11839.8215267723f64,
            12543.853951415977f64,
        ];

        check_frequencies(
            &EXPECTED_FREQUENCIES,
            &SCALE_12EDO2,
            NoteNumber::A4,
            Frequency::A4,
        )?;
        Ok(())
    }

    #[test]
    fn carlos_super_at_0() -> Result<()> {
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

        check_frequencies(
            &EXPECTED_FREQUENCIES,
            &CARLOS_SUPER,
            NoteNumber::ZERO,
            Frequency::MIN,
        )?;
        Ok(())
    }

    #[test]
    fn carlos_super_at_69() -> Result<()> {
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
            &CARLOS_SUPER,
            NoteNumber::A4,
            MidiNote::ALL[NoteNumber::A4.to_u8() as usize].frequency(),
        )?;
        Ok(())
    }

    fn check_frequencies(
        expected_frequencies: &[f64],
        scale: &Scale,
        base_note_number: NoteNumber,
        base_frequency: Frequency,
    ) -> Result<()> {
        let keyboard_mapping = KeyboardMapping::new(
            NoteNumber::ZERO,
            NoteNumber::MAX,
            base_note_number,
            base_frequency,
        )?;

        let frequencies = calculate_frequencies(scale, &keyboard_mapping);

        assert_eq!(expected_frequencies.len(), frequencies.len());
        for (expected, actual) in zip(expected_frequencies, frequencies) {
            assert_eq!(*expected, actual.0)
        }

        Ok(())
    }

    /*
    pub(crate) fn show_scale_frequencies() -> anyhow::Result<()> {
        use crate::frequencies::calculate_frequencies;
        use crate::frequency::Frequency;
        use crate::keyboard_mapping::KeyboardMapping;
        use crate::note_number::NoteNumber;
        use tuning_tool_macros::scale;

        let bohlen_p = scale![
            27/25
            25/21
            9/7
            7/5
            75/49
            5/3
            9/5
            49/25
            15/7
            7/3
            63/25
            25/9
            3/1
        ];

        let keyboard_mapping = KeyboardMapping::new(
            NoteNumber::ZERO,
            NoteNumber::MAX,
            NoteNumber::A4,
            Frequency::A4,
        )?;

        let frequencies = calculate_frequencies(&bohlen_p, &keyboard_mapping);
        for (i, f) in frequencies.iter().enumerate() {
            println!("{i:>3}: {f} Hz,");
        }
        println!("const EXPECTED_FREQUENCIES: [f64; 128] = [");
        for f in frequencies {
            println!("  {f}f64,");
        }
        println!("];");

        Ok(())
    }
    */
}
