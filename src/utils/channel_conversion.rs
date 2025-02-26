pub fn freq_to_channel(freq: u32) -> u16 {
    match freq {
        2412 => 1,
        2417 => 2,
        2422 => 3,
        2427 => 4,
        2432 => 5,
        2437 => 6,
        2442 => 7,
        2447 => 8,
        2452 => 9,
        2457 => 10,
        2462 => 11,
        2467 => 12,
        2472 => 13,
        2484 => 14,
        5160 => 32,
        5180 => 36,
        5200 => 40,
        5220 => 44,
        5240 => 48,
        5260 => 52,
        5280 => 56,
        5300 => 60,
        5320 => 64,
        5340 => 68,
        5360 => 72,
        5380 => 76,
        5400 => 80,
        5420 => 84,
        5440 => 88,
        5460 => 92,
        5480 => 96,
        5500 => 100,
        5520 => 104,
        5540 => 108,
        5560 => 112,
        5580 => 116,
        5600 => 120,
        5620 => 124,
        5640 => 128,
        5660 => 132,
        5680 => 136,
        5700 => 140,
        5720 => 144,
        5745 => 149,
        5765 => 153,
        5785 => 157,
        5805 => 161,
        5825 => 165,
        5845 => 169,
        5865 => 173,
        5885 => 177,
        _ => 0,
    }
}

pub fn channel_to_freq(channel: u16) -> u32 {
    match channel {
        1 => 2412,
        2 => 2417,
        3 => 2422,
        4 => 2427,
        5 => 2432,
        6 => 2437,
        7 => 2442,
        8 => 2447,
        9 => 2452,
        10 => 2457,
        11 => 2462,
        12 => 2467,
        13 => 2472,
        14 => 2484,
        32 => 5160,
        36 => 5180,
        40 => 5200,
        44 => 5220,
        48 => 5240,
        52 => 5260,
        56 => 5280,
        60 => 5300,
        64 => 5320,
        68 => 5340,
        72 => 5360,
        76 => 5380,
        80 => 5400,
        84 => 5420,
        88 => 5440,
        92 => 5460,
        96 => 5480,
        100 => 5500,
        104 => 5520,
        108 => 5540,
        112 => 5560,
        116 => 5580,
        120 => 5600,
        124 => 5620,
        128 => 5640,
        132 => 5660,
        136 => 5680,
        140 => 5700,
        144 => 5720,
        149 => 5745,
        153 => 5765,
        157 => 5785,
        161 => 5805,
        165 => 5825,
        169 => 5845,
        173 => 5865,
        177 => 5885,
        _ => 0,
    }
}

mod test {
    use super::{channel_to_freq, freq_to_channel};

    #[test]
    fn test_freq_to_channel_2ghz() {
        assert_eq!(freq_to_channel(2412), 1);
        assert_eq!(freq_to_channel(2437), 6);
        assert_eq!(freq_to_channel(2462), 11);
        assert_eq!(freq_to_channel(2484), 14);
    }

    #[test]
    fn test_freq_to_channel_5ghz() {
        assert_eq!(freq_to_channel(5180), 36);
        assert_eq!(freq_to_channel(5500), 100);
        assert_eq!(freq_to_channel(5805), 161);
    }

    #[test]
    fn test_freq_to_channel_invalid() {
        // Test an invalid frequency
        assert_eq!(freq_to_channel(1000), 0);
    }

    #[test]
    fn test_channel_to_freq_2ghz() {
        assert_eq!(channel_to_freq(1), 2412);
        assert_eq!(channel_to_freq(6), 2437);
        assert_eq!(channel_to_freq(11), 2462);
    }

    #[test]
    fn test_channel_to_freq_5ghz() {
        assert_eq!(channel_to_freq(36), 5180);
        assert_eq!(channel_to_freq(100), 5500);
        assert_eq!(channel_to_freq(161), 5805);
    }

    #[test]
    fn test_channel_to_freq_invalid() {
        // Test an invalid channel
        assert_eq!(channel_to_freq(1000), 0);
    }

    #[test]
    fn test_bidirectional_conversion() {
        // Test that converting from freq to channel and back gives the original freq
        let original_freqs = [2412, 2437, 5180, 5500, 5805];

        for freq in original_freqs {
            let channel = freq_to_channel(freq);
            let converted_back = channel_to_freq(channel);
            assert_eq!(converted_back, freq);
        }

        // Test that converting from channel to freq and back gives the original channel
        let original_channels = [1, 6, 11, 36, 100, 161];

        for channel in original_channels {
            let freq = channel_to_freq(channel);
            let converted_back = freq_to_channel(freq);
            assert_eq!(converted_back, channel);
        }
    }
}
