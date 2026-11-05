//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use std::fmt;

/// Implemented according to IEEE 1278.1-2012 §6.2.49
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct JammingTechnique {
    pub kind: u8,
    pub category: u8,
    pub subcategory: u8,
    pub specific: u8,
}

impl JammingTechnique {
    #[must_use]
    pub const fn new(kind: u8, category: u8, subcategory: u8, specific: u8) -> Self {
        Self {
            kind,
            category,
            subcategory,
            specific,
        }
    }

    /// The technique's name as defined in SISO-REF-010, or `None` if the code is not
    /// in the table.
    #[must_use]
    pub fn name(self) -> Option<&'static str> {
        NAMES
            .iter()
            .find(|(technique, _)| *technique == self)
            .map(|&(_, name)| name)
    }

    /// Whether this technique is `parent` or falls under it in the hierarchy.
    ///
    /// Every level that `parent` specifies must match; the levels it leaves as 0
    /// match anything. So every noise technique is within [`NOISE`](Self::NOISE)
    /// and [`CLICK`](Self::CLICK) is within [`BARRAGE_NOISE`](Self::BARRAGE_NOISE).
    #[must_use]
    pub const fn is_within(self, parent: Self) -> bool {
        const fn level_matches(level: u8, parent: u8) -> bool {
            parent == 0 || level == parent
        }

        self.kind == parent.kind
            && level_matches(self.category, parent.category)
            && level_matches(self.subcategory, parent.subcategory)
            && level_matches(self.specific, parent.specific)
    }
}

/// Every technique in SISO-REF-010-2023 Jamming Technique [UID 284], with its name.
const NAMES: [(JammingTechnique, &str); 134] = [
    (JammingTechnique::new(1, 0, 0, 0), "Noise"),
    (
        JammingTechnique::new(1, 5, 0, 0),
        "Amplitude Modulation (AM) Noise",
    ),
    (JammingTechnique::new(1, 10, 0, 0), "Barrage Noise"),
    (JammingTechnique::new(1, 10, 5, 0), "Click"),
    (JammingTechnique::new(1, 10, 10, 0), "Source Noise"),
    (JammingTechnique::new(1, 15, 0, 0), "Bistatic Clutter"),
    (JammingTechnique::new(1, 20, 0, 0), "Comb"),
    (
        JammingTechnique::new(1, 25, 0, 0),
        "Cooperative Blinked Noise (CBN)",
    ),
    (JammingTechnique::new(1, 30, 0, 0), "Doppler Noise"),
    (
        JammingTechnique::new(1, 35, 0, 0),
        "Frequency Modulation (FM) by Noise",
    ),
    (JammingTechnique::new(1, 40, 0, 0), "Impulse Noise"),
    (JammingTechnique::new(1, 45, 0, 0), "Partial Band"),
    (JammingTechnique::new(1, 50, 0, 0), "Pseudorandom AM"),
    (JammingTechnique::new(1, 55, 0, 0), "Pulse Noise"),
    (
        JammingTechnique::new(1, 60, 0, 0),
        "Quasi-Noise (aka Pseudorandom)",
    ),
    (
        JammingTechnique::new(1, 65, 0, 0),
        "Range Bin Masking (RBM) (aka Cover Pulse)",
    ),
    (
        JammingTechnique::new(1, 65, 5, 0),
        "Range Bin Masking with Velocity Bin Masking",
    ),
    (JammingTechnique::new(1, 70, 0, 0), "Repeater Noise"),
    (
        JammingTechnique::new(1, 70, 5, 0),
        "Narrowband Repeater Noise",
    ),
    (
        JammingTechnique::new(1, 70, 10, 0),
        "Wide Band Repeater Noise",
    ),
    (JammingTechnique::new(1, 75, 0, 0), "Spot Noise"),
    (
        JammingTechnique::new(1, 75, 5, 0),
        "Automatic Spot Noise (ASJ)",
    ),
    (JammingTechnique::new(1, 75, 10, 0), "Blinking Spot Noise"),
    (JammingTechnique::new(1, 75, 15, 0), "Burst Spot Noise"),
    (JammingTechnique::new(1, 75, 20, 0), "Doppler Spot Noise"),
    (JammingTechnique::new(1, 75, 25, 0), "Skirt Frequency"),
    (
        JammingTechnique::new(1, 80, 0, 0),
        "Swept Noise (aka Swept Spot Noise, Sweep)",
    ),
    (JammingTechnique::new(1, 80, 5, 0), "Frequency Swept"),
    (JammingTechnique::new(1, 80, 10, 0), "Swept AM"),
    (
        JammingTechnique::new(1, 85, 0, 0),
        "Velocity Bin Masking (VBM)",
    ),
    (JammingTechnique::new(2, 0, 0, 0), "Deception"),
    (JammingTechnique::new(2, 5, 0, 0), "Analyzer"),
    (JammingTechnique::new(2, 10, 0, 0), "Angle"),
    (JammingTechnique::new(2, 10, 5, 0), "Angle Gate Walk-Off"),
    (
        JammingTechnique::new(2, 10, 10, 0),
        "Cooperative Angle (CAJ)",
    ),
    (JammingTechnique::new(2, 10, 15, 0), "Cross-Eye"),
    (JammingTechnique::new(2, 10, 20, 0), "Cross-Polarization"),
    (JammingTechnique::new(2, 10, 25, 0), "Delta"),
    (
        JammingTechnique::new(2, 10, 30, 0),
        "Inverse Gain (aka Inverse Amplitude)",
    ),
    (JammingTechnique::new(2, 10, 35, 0), "Sea-Bounced"),
    (
        JammingTechnique::new(2, 10, 40, 0),
        "Swept Square Wave (SSW)",
    ),
    (JammingTechnique::new(2, 10, 45, 0), "Terrain Bounce"),
    (JammingTechnique::new(2, 15, 0, 0), "Angle and Gate Stealer"),
    (
        JammingTechnique::new(2, 15, 5, 0),
        "Cross-Polarization and Range Gate Pull-Off (RGPO)",
    ),
    (
        JammingTechnique::new(2, 15, 10, 0),
        "Cross-Polarization and Velocity Gate Pull-Off (VGPO)",
    ),
    (
        JammingTechnique::new(2, 15, 15, 0),
        "Cross-Polarization, RGPO and VGPO",
    ),
    (JammingTechnique::new(2, 15, 20, 0), "Inverse Gain and RGPO"),
    (
        JammingTechnique::new(2, 15, 25, 0),
        "Inverse Gain and RGPO and VGPO",
    ),
    (JammingTechnique::new(2, 15, 30, 0), "Inverse Gain and VGPO"),
    (JammingTechnique::new(2, 15, 35, 0), "RGPO and SSW"),
    (JammingTechnique::new(2, 15, 40, 0), "SSW and VGPO"),
    (JammingTechnique::new(2, 20, 0, 0), "Angle and False Target"),
    (
        JammingTechnique::new(2, 20, 5, 0),
        "Angle and Velocity False Targets (VFT)",
    ),
    (JammingTechnique::new(2, 20, 5, 5), "Inverse Gain and VFT"),
    (JammingTechnique::new(2, 20, 5, 10), "SSW and VFT"),
    (
        JammingTechnique::new(2, 20, 10, 0),
        "Range False Targets (RFT) and Inverse Gain",
    ),
    (JammingTechnique::new(2, 20, 15, 0), "RFT and SSW"),
    (
        JammingTechnique::new(2, 25, 0, 0),
        "Angle and Random Range Programs (RANRAP)",
    ),
    (JammingTechnique::new(2, 25, 5, 0), "RANRAP and SSW"),
    (JammingTechnique::new(2, 30, 0, 0), "Angle and Velocity"),
    (JammingTechnique::new(2, 30, 5, 0), "Inverse Gain and VBM"),
    (JammingTechnique::new(2, 30, 10, 0), "SSW and VBM"),
    (
        JammingTechnique::new(2, 35, 0, 0),
        "Automatic Gain Control (AGC)",
    ),
    (JammingTechnique::new(2, 40, 0, 0), "AGC and Gate Stealer"),
    (JammingTechnique::new(2, 40, 5, 0), "AGC and RGPO"),
    (JammingTechnique::new(2, 40, 10, 0), "AGC and VGPO"),
    (JammingTechnique::new(2, 40, 15, 0), "AGC and RGPO and VGPO"),
    (JammingTechnique::new(2, 45, 0, 0), "Colinear"),
    (
        JammingTechnique::new(2, 50, 0, 0),
        "Constant False Alarm Rate",
    ),
    (JammingTechnique::new(2, 55, 0, 0), "Double Cross"),
    (JammingTechnique::new(2, 60, 0, 0), "Down Link"),
    (JammingTechnique::new(2, 65, 0, 0), "False Target"),
    (JammingTechnique::new(2, 65, 5, 0), "Coherent False Targets"),
    (
        JammingTechnique::new(2, 65, 10, 0),
        "False Doppler Target (FDT)",
    ),
    (
        JammingTechnique::new(2, 65, 15, 0),
        "Multiple False Targets",
    ),
    (
        JammingTechnique::new(2, 65, 20, 0),
        "Range False Targets (RFT)",
    ),
    (JammingTechnique::new(2, 65, 25, 0), "Transponder"),
    (
        JammingTechnique::new(2, 65, 30, 0),
        "Velocity False Targets (VFT)",
    ),
    (JammingTechnique::new(2, 70, 0, 0), "Figure Eight"),
    (JammingTechnique::new(2, 75, 0, 0), "Gate Stealer"),
    (
        JammingTechnique::new(2, 75, 5, 0),
        "Chirp Gate Stealer (CGS)",
    ),
    (
        JammingTechnique::new(2, 75, 10, 0),
        "Range Gate Pull-Off (RGPO)",
    ),
    (JammingTechnique::new(2, 75, 15, 0), "RGPO and VGPO"),
    (
        JammingTechnique::new(2, 75, 20, 0),
        "VGPO (aka Velocity Gate Stealer, VGS)",
    ),
    (
        JammingTechnique::new(2, 80, 0, 0),
        "Gate Stealer and Repeater",
    ),
    (
        JammingTechnique::new(2, 80, 5, 0),
        "Repeater Swept Amplitude Modulation (RSAM) and VGPO",
    ),
    (JammingTechnique::new(2, 85, 0, 0), "Glint Enhance"),
    (JammingTechnique::new(2, 90, 0, 0), "Image Frequency"),
    (
        JammingTechnique::new(2, 95, 0, 0),
        "Jittered Pulse Repetition Frequency",
    ),
    (JammingTechnique::new(2, 100, 0, 0), "Jittered Pulse Width"),
    (
        JammingTechnique::new(2, 105, 0, 0),
        "Pseudorandom Noise (PRN)",
    ),
    (JammingTechnique::new(2, 110, 0, 0), "Pulse"),
    (
        JammingTechnique::new(2, 115, 0, 0),
        "Pulse Compression Deception",
    ),
    (
        JammingTechnique::new(2, 120, 0, 0),
        "Random Range Programs (RANRAP)",
    ),
    (JammingTechnique::new(2, 125, 0, 0), "Refraction"),
    (JammingTechnique::new(2, 130, 0, 0), "Repeater"),
    (
        JammingTechnique::new(2, 130, 5, 0),
        "Continuous Wave Repeater",
    ),
    (JammingTechnique::new(2, 130, 10, 0), "Repeater Noise"),
    (
        JammingTechnique::new(2, 130, 15, 0),
        "Multiple Frequency Repeater (MFR)",
    ),
    (
        JammingTechnique::new(2, 130, 20, 0),
        "Narrow Band Repeater Noise (NBRN)",
    ),
    (JammingTechnique::new(2, 130, 25, 0), "Random Doppler (RD)"),
    (
        JammingTechnique::new(2, 130, 30, 0),
        "Repeater Digital Radio Frequency Memory (DRFM)",
    ),
    (JammingTechnique::new(2, 130, 30, 1), "DRFM Level A"),
    (JammingTechnique::new(2, 130, 30, 2), "DRFM Level B"),
    (JammingTechnique::new(2, 130, 30, 3), "DRFM Level C"),
    (JammingTechnique::new(2, 130, 30, 4), "DRFM Level D"),
    (JammingTechnique::new(2, 130, 30, 5), "DRFM Level E"),
    (
        JammingTechnique::new(2, 130, 35, 0),
        "Repeater Swept Amplitude Modulation (RSAM)",
    ),
    (JammingTechnique::new(2, 135, 0, 0), "Scintillation"),
    (JammingTechnique::new(2, 140, 0, 0), "Serrodyne"),
    (JammingTechnique::new(2, 145, 0, 0), "Velocity"),
    (JammingTechnique::new(3, 0, 0, 0), "Deception and Noise"),
    (JammingTechnique::new(3, 5, 0, 0), "Angle and Noise"),
    (JammingTechnique::new(3, 5, 5, 0), "Angle and Barrage"),
    (
        JammingTechnique::new(3, 5, 5, 5),
        "Barrage and Inverse Gain",
    ),
    (JammingTechnique::new(3, 5, 5, 10), "Barrage and SSW"),
    (JammingTechnique::new(3, 10, 0, 0), "Angle and FM by Noise"),
    (
        JammingTechnique::new(3, 10, 5, 0),
        "FM by Noise and Inverse Gain",
    ),
    (JammingTechnique::new(3, 10, 10, 0), "FM by Noise and SSW"),
    (
        JammingTechnique::new(3, 15, 0, 0),
        "Angle and Pseudorandom AM",
    ),
    (
        JammingTechnique::new(3, 15, 5, 0),
        "Inverse Gain and Pseudorandom AM",
    ),
    (
        JammingTechnique::new(3, 15, 10, 0),
        "Pseudorandom AM and SSW",
    ),
    (JammingTechnique::new(3, 20, 0, 0), "Angle and Spot"),
    (
        JammingTechnique::new(3, 20, 5, 0),
        "Inverse Gain and Low Level Noise",
    ),
    (
        JammingTechnique::new(3, 20, 10, 0),
        "Inverse Gain and Spot Noise",
    ),
    (JammingTechnique::new(3, 20, 15, 0), "Spot and SSW"),
    (JammingTechnique::new(3, 25, 0, 0), "Gate Stealer and Noise"),
    (JammingTechnique::new(3, 25, 5, 0), "Noise and RGPO"),
    (
        JammingTechnique::new(3, 25, 5, 5),
        "Low Level Noise and RGPO",
    ),
    (JammingTechnique::new(3, 25, 10, 0), "Noise and VGPO"),
    (
        JammingTechnique::new(3, 25, 10, 5),
        "Low Level Noise and VGPO",
    ),
    (
        JammingTechnique::new(3, 30, 0, 0),
        "False Target Deception and Swept Noise",
    ),
    (JammingTechnique::new(4, 0, 0, 0), "Special"),
    (JammingTechnique::new(4, 1, 0, 0), "Super Jam"),
];

/// Formats the code in SISO-REF-010's dotted form, without trailing zero levels,
/// followed by the name when it's known: for example `1.10.5 Click`.
impl fmt::Display for JammingTechnique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let levels = [self.kind, self.category, self.subcategory, self.specific];
        let shown = levels
            .iter()
            .rposition(|&level| level != 0)
            .map_or(1, |last| last.saturating_add(1));
        for (i, level) in levels.iter().take(shown).enumerate() {
            if i > 0 {
                f.write_str(".")?;
            }
            write!(f, "{level}")?;
        }
        if let Some(name) = self.name() {
            write!(f, " {name}")?;
        }
        Ok(())
    }
}

/// The techniques defined in SISO-REF-010-2023 Jamming Technique [UID 284].
impl JammingTechnique {
    /// `1` Noise.
    pub const NOISE: Self = Self::new(1, 0, 0, 0);
    /// `1.5` Amplitude Modulation (AM) Noise, under Noise.
    pub const AMPLITUDE_MODULATION_NOISE: Self = Self::new(1, 5, 0, 0);
    /// `1.10` Barrage Noise, under Noise.
    pub const BARRAGE_NOISE: Self = Self::new(1, 10, 0, 0);
    /// `1.10.5` Click, under Noise → Barrage Noise.
    pub const CLICK: Self = Self::new(1, 10, 5, 0);
    /// `1.10.10` Source Noise, under Noise → Barrage Noise.
    pub const SOURCE_NOISE: Self = Self::new(1, 10, 10, 0);
    /// `1.15` Bistatic Clutter, under Noise.
    pub const BISTATIC_CLUTTER: Self = Self::new(1, 15, 0, 0);
    /// `1.20` Comb, under Noise.
    pub const COMB: Self = Self::new(1, 20, 0, 0);
    /// `1.25` Cooperative Blinked Noise (CBN), under Noise.
    pub const COOPERATIVE_BLINKED_NOISE: Self = Self::new(1, 25, 0, 0);
    /// `1.30` Doppler Noise, under Noise.
    pub const DOPPLER_NOISE: Self = Self::new(1, 30, 0, 0);
    /// `1.35` Frequency Modulation (FM) by Noise, under Noise.
    pub const FREQUENCY_MODULATION_BY_NOISE: Self = Self::new(1, 35, 0, 0);
    /// `1.40` Impulse Noise, under Noise.
    pub const IMPULSE_NOISE: Self = Self::new(1, 40, 0, 0);
    /// `1.45` Partial Band, under Noise.
    pub const PARTIAL_BAND: Self = Self::new(1, 45, 0, 0);
    /// `1.50` Pseudorandom AM, under Noise.
    pub const PSEUDORANDOM_AM: Self = Self::new(1, 50, 0, 0);
    /// `1.55` Pulse Noise, under Noise.
    pub const PULSE_NOISE: Self = Self::new(1, 55, 0, 0);
    /// `1.60` Quasi-Noise (aka Pseudorandom), under Noise.
    pub const QUASI_NOISE: Self = Self::new(1, 60, 0, 0);
    /// `1.65` Range Bin Masking (RBM) (aka Cover Pulse), under Noise.
    pub const RANGE_BIN_MASKING: Self = Self::new(1, 65, 0, 0);
    /// `1.65.5` Range Bin Masking with Velocity Bin Masking, under Noise → Range Bin Masking (RBM) (aka Cover Pulse).
    pub const RANGE_BIN_MASKING_WITH_VELOCITY_BIN_MASKING: Self = Self::new(1, 65, 5, 0);
    /// `1.70` Repeater Noise, under Noise.
    pub const NOISE_REPEATER_NOISE: Self = Self::new(1, 70, 0, 0);
    /// `1.70.5` Narrowband Repeater Noise, under Noise → Repeater Noise.
    pub const NARROWBAND_REPEATER_NOISE: Self = Self::new(1, 70, 5, 0);
    /// `1.70.10` Wide Band Repeater Noise, under Noise → Repeater Noise.
    pub const WIDE_BAND_REPEATER_NOISE: Self = Self::new(1, 70, 10, 0);
    /// `1.75` Spot Noise, under Noise.
    pub const SPOT_NOISE: Self = Self::new(1, 75, 0, 0);
    /// `1.75.5` Automatic Spot Noise (ASJ), under Noise → Spot Noise.
    pub const AUTOMATIC_SPOT_NOISE: Self = Self::new(1, 75, 5, 0);
    /// `1.75.10` Blinking Spot Noise, under Noise → Spot Noise.
    pub const BLINKING_SPOT_NOISE: Self = Self::new(1, 75, 10, 0);
    /// `1.75.15` Burst Spot Noise, under Noise → Spot Noise.
    pub const BURST_SPOT_NOISE: Self = Self::new(1, 75, 15, 0);
    /// `1.75.20` Doppler Spot Noise, under Noise → Spot Noise.
    pub const DOPPLER_SPOT_NOISE: Self = Self::new(1, 75, 20, 0);
    /// `1.75.25` Skirt Frequency, under Noise → Spot Noise.
    pub const SKIRT_FREQUENCY: Self = Self::new(1, 75, 25, 0);
    /// `1.80` Swept Noise (aka Swept Spot Noise, Sweep), under Noise.
    pub const SWEPT_NOISE: Self = Self::new(1, 80, 0, 0);
    /// `1.80.5` Frequency Swept, under Noise → Swept Noise (aka Swept Spot Noise, Sweep).
    pub const FREQUENCY_SWEPT: Self = Self::new(1, 80, 5, 0);
    /// `1.80.10` Swept AM, under Noise → Swept Noise (aka Swept Spot Noise, Sweep).
    pub const SWEPT_AM: Self = Self::new(1, 80, 10, 0);
    /// `1.85` Velocity Bin Masking (VBM), under Noise.
    pub const VELOCITY_BIN_MASKING: Self = Self::new(1, 85, 0, 0);
    /// `2` Deception.
    pub const DECEPTION: Self = Self::new(2, 0, 0, 0);
    /// `2.5` Analyzer, under Deception.
    pub const ANALYZER: Self = Self::new(2, 5, 0, 0);
    /// `2.10` Angle, under Deception.
    pub const ANGLE: Self = Self::new(2, 10, 0, 0);
    /// `2.10.5` Angle Gate Walk-Off, under Deception → Angle.
    pub const ANGLE_GATE_WALK_OFF: Self = Self::new(2, 10, 5, 0);
    /// `2.10.10` Cooperative Angle (CAJ), under Deception → Angle.
    pub const COOPERATIVE_ANGLE: Self = Self::new(2, 10, 10, 0);
    /// `2.10.15` Cross-Eye, under Deception → Angle.
    pub const CROSS_EYE: Self = Self::new(2, 10, 15, 0);
    /// `2.10.20` Cross-Polarization, under Deception → Angle.
    pub const CROSS_POLARIZATION: Self = Self::new(2, 10, 20, 0);
    /// `2.10.25` Delta, under Deception → Angle.
    pub const DELTA: Self = Self::new(2, 10, 25, 0);
    /// `2.10.30` Inverse Gain (aka Inverse Amplitude), under Deception → Angle.
    pub const INVERSE_GAIN: Self = Self::new(2, 10, 30, 0);
    /// `2.10.35` Sea-Bounced, under Deception → Angle.
    pub const SEA_BOUNCED: Self = Self::new(2, 10, 35, 0);
    /// `2.10.40` Swept Square Wave (SSW), under Deception → Angle.
    pub const SWEPT_SQUARE_WAVE: Self = Self::new(2, 10, 40, 0);
    /// `2.10.45` Terrain Bounce, under Deception → Angle.
    pub const TERRAIN_BOUNCE: Self = Self::new(2, 10, 45, 0);
    /// `2.15` Angle and Gate Stealer, under Deception.
    pub const ANGLE_AND_GATE_STEALER: Self = Self::new(2, 15, 0, 0);
    /// `2.15.5` Cross-Polarization and Range Gate Pull-Off (RGPO), under Deception → Angle and Gate Stealer.
    pub const CROSS_POLARIZATION_AND_RANGE_GATE_PULL_OFF: Self = Self::new(2, 15, 5, 0);
    /// `2.15.10` Cross-Polarization and Velocity Gate Pull-Off (VGPO), under Deception → Angle and Gate Stealer.
    pub const CROSS_POLARIZATION_AND_VELOCITY_GATE_PULL_OFF: Self = Self::new(2, 15, 10, 0);
    /// `2.15.15` Cross-Polarization, RGPO and VGPO, under Deception → Angle and Gate Stealer.
    pub const CROSS_POLARIZATION_RGPO_AND_VGPO: Self = Self::new(2, 15, 15, 0);
    /// `2.15.20` Inverse Gain and RGPO, under Deception → Angle and Gate Stealer.
    pub const INVERSE_GAIN_AND_RGPO: Self = Self::new(2, 15, 20, 0);
    /// `2.15.25` Inverse Gain and RGPO and VGPO, under Deception → Angle and Gate Stealer.
    pub const INVERSE_GAIN_AND_RGPO_AND_VGPO: Self = Self::new(2, 15, 25, 0);
    /// `2.15.30` Inverse Gain and VGPO, under Deception → Angle and Gate Stealer.
    pub const INVERSE_GAIN_AND_VGPO: Self = Self::new(2, 15, 30, 0);
    /// `2.15.35` RGPO and SSW, under Deception → Angle and Gate Stealer.
    pub const RGPO_AND_SSW: Self = Self::new(2, 15, 35, 0);
    /// `2.15.40` SSW and VGPO, under Deception → Angle and Gate Stealer.
    pub const SSW_AND_VGPO: Self = Self::new(2, 15, 40, 0);
    /// `2.20` Angle and False Target, under Deception.
    pub const ANGLE_AND_FALSE_TARGET: Self = Self::new(2, 20, 0, 0);
    /// `2.20.5` Angle and Velocity False Targets (VFT), under Deception → Angle and False Target.
    pub const ANGLE_AND_VELOCITY_FALSE_TARGETS: Self = Self::new(2, 20, 5, 0);
    /// `2.20.5.5` Inverse Gain and VFT, under Deception → Angle and False Target → Angle and Velocity False Targets (VFT).
    pub const INVERSE_GAIN_AND_VFT: Self = Self::new(2, 20, 5, 5);
    /// `2.20.5.10` SSW and VFT, under Deception → Angle and False Target → Angle and Velocity False Targets (VFT).
    pub const SSW_AND_VFT: Self = Self::new(2, 20, 5, 10);
    /// `2.20.10` Range False Targets (RFT) and Inverse Gain, under Deception → Angle and False Target.
    pub const RANGE_FALSE_TARGETS_AND_INVERSE_GAIN: Self = Self::new(2, 20, 10, 0);
    /// `2.20.15` RFT and SSW, under Deception → Angle and False Target.
    pub const RFT_AND_SSW: Self = Self::new(2, 20, 15, 0);
    /// `2.25` Angle and Random Range Programs (RANRAP), under Deception.
    pub const ANGLE_AND_RANDOM_RANGE_PROGRAMS: Self = Self::new(2, 25, 0, 0);
    /// `2.25.5` RANRAP and SSW, under Deception → Angle and Random Range Programs (RANRAP).
    pub const RANRAP_AND_SSW: Self = Self::new(2, 25, 5, 0);
    /// `2.30` Angle and Velocity, under Deception.
    pub const ANGLE_AND_VELOCITY: Self = Self::new(2, 30, 0, 0);
    /// `2.30.5` Inverse Gain and VBM, under Deception → Angle and Velocity.
    pub const INVERSE_GAIN_AND_VBM: Self = Self::new(2, 30, 5, 0);
    /// `2.30.10` SSW and VBM, under Deception → Angle and Velocity.
    pub const SSW_AND_VBM: Self = Self::new(2, 30, 10, 0);
    /// `2.35` Automatic Gain Control (AGC), under Deception.
    pub const AUTOMATIC_GAIN_CONTROL: Self = Self::new(2, 35, 0, 0);
    /// `2.40` AGC and Gate Stealer, under Deception.
    pub const AGC_AND_GATE_STEALER: Self = Self::new(2, 40, 0, 0);
    /// `2.40.5` AGC and RGPO, under Deception → AGC and Gate Stealer.
    pub const AGC_AND_RGPO: Self = Self::new(2, 40, 5, 0);
    /// `2.40.10` AGC and VGPO, under Deception → AGC and Gate Stealer.
    pub const AGC_AND_VGPO: Self = Self::new(2, 40, 10, 0);
    /// `2.40.15` AGC and RGPO and VGPO, under Deception → AGC and Gate Stealer.
    pub const AGC_AND_RGPO_AND_VGPO: Self = Self::new(2, 40, 15, 0);
    /// `2.45` Colinear, under Deception.
    pub const COLINEAR: Self = Self::new(2, 45, 0, 0);
    /// `2.50` Constant False Alarm Rate, under Deception.
    pub const CONSTANT_FALSE_ALARM_RATE: Self = Self::new(2, 50, 0, 0);
    /// `2.55` Double Cross, under Deception.
    pub const DOUBLE_CROSS: Self = Self::new(2, 55, 0, 0);
    /// `2.60` Down Link, under Deception.
    pub const DOWN_LINK: Self = Self::new(2, 60, 0, 0);
    /// `2.65` False Target, under Deception.
    pub const FALSE_TARGET: Self = Self::new(2, 65, 0, 0);
    /// `2.65.5` Coherent False Targets, under Deception → False Target.
    pub const COHERENT_FALSE_TARGETS: Self = Self::new(2, 65, 5, 0);
    /// `2.65.10` False Doppler Target (FDT), under Deception → False Target.
    pub const FALSE_DOPPLER_TARGET: Self = Self::new(2, 65, 10, 0);
    /// `2.65.15` Multiple False Targets, under Deception → False Target.
    pub const MULTIPLE_FALSE_TARGETS: Self = Self::new(2, 65, 15, 0);
    /// `2.65.20` Range False Targets (RFT), under Deception → False Target.
    pub const RANGE_FALSE_TARGETS: Self = Self::new(2, 65, 20, 0);
    /// `2.65.25` Transponder, under Deception → False Target.
    pub const TRANSPONDER: Self = Self::new(2, 65, 25, 0);
    /// `2.65.30` Velocity False Targets (VFT), under Deception → False Target.
    pub const VELOCITY_FALSE_TARGETS: Self = Self::new(2, 65, 30, 0);
    /// `2.70` Figure Eight, under Deception.
    pub const FIGURE_EIGHT: Self = Self::new(2, 70, 0, 0);
    /// `2.75` Gate Stealer, under Deception.
    pub const GATE_STEALER: Self = Self::new(2, 75, 0, 0);
    /// `2.75.5` Chirp Gate Stealer (CGS), under Deception → Gate Stealer.
    pub const CHIRP_GATE_STEALER: Self = Self::new(2, 75, 5, 0);
    /// `2.75.10` Range Gate Pull-Off (RGPO), under Deception → Gate Stealer.
    pub const RANGE_GATE_PULL_OFF: Self = Self::new(2, 75, 10, 0);
    /// `2.75.15` RGPO and VGPO, under Deception → Gate Stealer.
    pub const RGPO_AND_VGPO: Self = Self::new(2, 75, 15, 0);
    /// `2.75.20` VGPO (aka Velocity Gate Stealer, VGS), under Deception → Gate Stealer.
    pub const VGPO: Self = Self::new(2, 75, 20, 0);
    /// `2.80` Gate Stealer and Repeater, under Deception.
    pub const GATE_STEALER_AND_REPEATER: Self = Self::new(2, 80, 0, 0);
    /// `2.80.5` Repeater Swept Amplitude Modulation (RSAM) and VGPO, under Deception → Gate Stealer and Repeater.
    pub const REPEATER_SWEPT_AMPLITUDE_MODULATION_AND_VGPO: Self = Self::new(2, 80, 5, 0);
    /// `2.85` Glint Enhance, under Deception.
    pub const GLINT_ENHANCE: Self = Self::new(2, 85, 0, 0);
    /// `2.90` Image Frequency, under Deception.
    pub const IMAGE_FREQUENCY: Self = Self::new(2, 90, 0, 0);
    /// `2.95` Jittered Pulse Repetition Frequency, under Deception.
    pub const JITTERED_PULSE_REPETITION_FREQUENCY: Self = Self::new(2, 95, 0, 0);
    /// `2.100` Jittered Pulse Width, under Deception.
    pub const JITTERED_PULSE_WIDTH: Self = Self::new(2, 100, 0, 0);
    /// `2.105` Pseudorandom Noise (PRN), under Deception.
    pub const PSEUDORANDOM_NOISE: Self = Self::new(2, 105, 0, 0);
    /// `2.110` Pulse, under Deception.
    pub const PULSE: Self = Self::new(2, 110, 0, 0);
    /// `2.115` Pulse Compression Deception, under Deception.
    pub const PULSE_COMPRESSION_DECEPTION: Self = Self::new(2, 115, 0, 0);
    /// `2.120` Random Range Programs (RANRAP), under Deception.
    pub const RANDOM_RANGE_PROGRAMS: Self = Self::new(2, 120, 0, 0);
    /// `2.125` Refraction, under Deception.
    pub const REFRACTION: Self = Self::new(2, 125, 0, 0);
    /// `2.130` Repeater, under Deception.
    pub const REPEATER: Self = Self::new(2, 130, 0, 0);
    /// `2.130.5` Continuous Wave Repeater, under Deception → Repeater.
    pub const CONTINUOUS_WAVE_REPEATER: Self = Self::new(2, 130, 5, 0);
    /// `2.130.10` Repeater Noise, under Deception → Repeater.
    pub const DECEPTION_REPEATER_NOISE: Self = Self::new(2, 130, 10, 0);
    /// `2.130.15` Multiple Frequency Repeater (MFR), under Deception → Repeater.
    pub const MULTIPLE_FREQUENCY_REPEATER: Self = Self::new(2, 130, 15, 0);
    /// `2.130.20` Narrow Band Repeater Noise (NBRN), under Deception → Repeater.
    pub const NARROW_BAND_REPEATER_NOISE: Self = Self::new(2, 130, 20, 0);
    /// `2.130.25` Random Doppler (RD), under Deception → Repeater.
    pub const RANDOM_DOPPLER: Self = Self::new(2, 130, 25, 0);
    /// `2.130.30` Repeater Digital Radio Frequency Memory (DRFM), under Deception → Repeater.
    pub const REPEATER_DIGITAL_RADIO_FREQUENCY_MEMORY: Self = Self::new(2, 130, 30, 0);
    /// `2.130.30.1` DRFM Level A, under Deception → Repeater → Repeater Digital Radio Frequency Memory (DRFM).
    pub const DRFM_LEVEL_A: Self = Self::new(2, 130, 30, 1);
    /// `2.130.30.2` DRFM Level B, under Deception → Repeater → Repeater Digital Radio Frequency Memory (DRFM).
    pub const DRFM_LEVEL_B: Self = Self::new(2, 130, 30, 2);
    /// `2.130.30.3` DRFM Level C, under Deception → Repeater → Repeater Digital Radio Frequency Memory (DRFM).
    pub const DRFM_LEVEL_C: Self = Self::new(2, 130, 30, 3);
    /// `2.130.30.4` DRFM Level D, under Deception → Repeater → Repeater Digital Radio Frequency Memory (DRFM).
    pub const DRFM_LEVEL_D: Self = Self::new(2, 130, 30, 4);
    /// `2.130.30.5` DRFM Level E, under Deception → Repeater → Repeater Digital Radio Frequency Memory (DRFM).
    pub const DRFM_LEVEL_E: Self = Self::new(2, 130, 30, 5);
    /// `2.130.35` Repeater Swept Amplitude Modulation (RSAM), under Deception → Repeater.
    pub const REPEATER_SWEPT_AMPLITUDE_MODULATION: Self = Self::new(2, 130, 35, 0);
    /// `2.135` Scintillation, under Deception.
    pub const SCINTILLATION: Self = Self::new(2, 135, 0, 0);
    /// `2.140` Serrodyne, under Deception.
    pub const SERRODYNE: Self = Self::new(2, 140, 0, 0);
    /// `2.145` Velocity, under Deception.
    pub const VELOCITY: Self = Self::new(2, 145, 0, 0);
    /// `3` Deception and Noise.
    pub const DECEPTION_AND_NOISE: Self = Self::new(3, 0, 0, 0);
    /// `3.5` Angle and Noise, under Deception and Noise.
    pub const ANGLE_AND_NOISE: Self = Self::new(3, 5, 0, 0);
    /// `3.5.5` Angle and Barrage, under Deception and Noise → Angle and Noise.
    pub const ANGLE_AND_BARRAGE: Self = Self::new(3, 5, 5, 0);
    /// `3.5.5.5` Barrage and Inverse Gain, under Deception and Noise → Angle and Noise → Angle and Barrage.
    pub const BARRAGE_AND_INVERSE_GAIN: Self = Self::new(3, 5, 5, 5);
    /// `3.5.5.10` Barrage and SSW, under Deception and Noise → Angle and Noise → Angle and Barrage.
    pub const BARRAGE_AND_SSW: Self = Self::new(3, 5, 5, 10);
    /// `3.10` Angle and FM by Noise, under Deception and Noise.
    pub const ANGLE_AND_FM_BY_NOISE: Self = Self::new(3, 10, 0, 0);
    /// `3.10.5` FM by Noise and Inverse Gain, under Deception and Noise → Angle and FM by Noise.
    pub const FM_BY_NOISE_AND_INVERSE_GAIN: Self = Self::new(3, 10, 5, 0);
    /// `3.10.10` FM by Noise and SSW, under Deception and Noise → Angle and FM by Noise.
    pub const FM_BY_NOISE_AND_SSW: Self = Self::new(3, 10, 10, 0);
    /// `3.15` Angle and Pseudorandom AM, under Deception and Noise.
    pub const ANGLE_AND_PSEUDORANDOM_AM: Self = Self::new(3, 15, 0, 0);
    /// `3.15.5` Inverse Gain and Pseudorandom AM, under Deception and Noise → Angle and Pseudorandom AM.
    pub const INVERSE_GAIN_AND_PSEUDORANDOM_AM: Self = Self::new(3, 15, 5, 0);
    /// `3.15.10` Pseudorandom AM and SSW, under Deception and Noise → Angle and Pseudorandom AM.
    pub const PSEUDORANDOM_AM_AND_SSW: Self = Self::new(3, 15, 10, 0);
    /// `3.20` Angle and Spot, under Deception and Noise.
    pub const ANGLE_AND_SPOT: Self = Self::new(3, 20, 0, 0);
    /// `3.20.5` Inverse Gain and Low Level Noise, under Deception and Noise → Angle and Spot.
    pub const INVERSE_GAIN_AND_LOW_LEVEL_NOISE: Self = Self::new(3, 20, 5, 0);
    /// `3.20.10` Inverse Gain and Spot Noise, under Deception and Noise → Angle and Spot.
    pub const INVERSE_GAIN_AND_SPOT_NOISE: Self = Self::new(3, 20, 10, 0);
    /// `3.20.15` Spot and SSW, under Deception and Noise → Angle and Spot.
    pub const SPOT_AND_SSW: Self = Self::new(3, 20, 15, 0);
    /// `3.25` Gate Stealer and Noise, under Deception and Noise.
    pub const GATE_STEALER_AND_NOISE: Self = Self::new(3, 25, 0, 0);
    /// `3.25.5` Noise and RGPO, under Deception and Noise → Gate Stealer and Noise.
    pub const NOISE_AND_RGPO: Self = Self::new(3, 25, 5, 0);
    /// `3.25.5.5` Low Level Noise and RGPO, under Deception and Noise → Gate Stealer and Noise → Noise and RGPO.
    pub const LOW_LEVEL_NOISE_AND_RGPO: Self = Self::new(3, 25, 5, 5);
    /// `3.25.10` Noise and VGPO, under Deception and Noise → Gate Stealer and Noise.
    pub const NOISE_AND_VGPO: Self = Self::new(3, 25, 10, 0);
    /// `3.25.10.5` Low Level Noise and VGPO, under Deception and Noise → Gate Stealer and Noise → Noise and VGPO.
    pub const LOW_LEVEL_NOISE_AND_VGPO: Self = Self::new(3, 25, 10, 5);
    /// `3.30` False Target Deception and Swept Noise, under Deception and Noise.
    pub const FALSE_TARGET_DECEPTION_AND_SWEPT_NOISE: Self = Self::new(3, 30, 0, 0);
    /// `4` Special.
    pub const SPECIAL: Self = Self::new(4, 0, 0, 0);
    /// `4.1` Super Jam, under Special.
    pub const SUPER_JAM: Self = Self::new(4, 1, 0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every constant, with the code SISO-REF-010 gives it.
    const TABLE: &[(JammingTechnique, [u8; 4])] = &[
        (JammingTechnique::NOISE, [1, 0, 0, 0]),
        (JammingTechnique::AMPLITUDE_MODULATION_NOISE, [1, 5, 0, 0]),
        (JammingTechnique::BARRAGE_NOISE, [1, 10, 0, 0]),
        (JammingTechnique::CLICK, [1, 10, 5, 0]),
        (JammingTechnique::SOURCE_NOISE, [1, 10, 10, 0]),
        (JammingTechnique::BISTATIC_CLUTTER, [1, 15, 0, 0]),
        (JammingTechnique::COMB, [1, 20, 0, 0]),
        (JammingTechnique::COOPERATIVE_BLINKED_NOISE, [1, 25, 0, 0]),
        (JammingTechnique::DOPPLER_NOISE, [1, 30, 0, 0]),
        (
            JammingTechnique::FREQUENCY_MODULATION_BY_NOISE,
            [1, 35, 0, 0],
        ),
        (JammingTechnique::IMPULSE_NOISE, [1, 40, 0, 0]),
        (JammingTechnique::PARTIAL_BAND, [1, 45, 0, 0]),
        (JammingTechnique::PSEUDORANDOM_AM, [1, 50, 0, 0]),
        (JammingTechnique::PULSE_NOISE, [1, 55, 0, 0]),
        (JammingTechnique::QUASI_NOISE, [1, 60, 0, 0]),
        (JammingTechnique::RANGE_BIN_MASKING, [1, 65, 0, 0]),
        (
            JammingTechnique::RANGE_BIN_MASKING_WITH_VELOCITY_BIN_MASKING,
            [1, 65, 5, 0],
        ),
        (JammingTechnique::NOISE_REPEATER_NOISE, [1, 70, 0, 0]),
        (JammingTechnique::NARROWBAND_REPEATER_NOISE, [1, 70, 5, 0]),
        (JammingTechnique::WIDE_BAND_REPEATER_NOISE, [1, 70, 10, 0]),
        (JammingTechnique::SPOT_NOISE, [1, 75, 0, 0]),
        (JammingTechnique::AUTOMATIC_SPOT_NOISE, [1, 75, 5, 0]),
        (JammingTechnique::BLINKING_SPOT_NOISE, [1, 75, 10, 0]),
        (JammingTechnique::BURST_SPOT_NOISE, [1, 75, 15, 0]),
        (JammingTechnique::DOPPLER_SPOT_NOISE, [1, 75, 20, 0]),
        (JammingTechnique::SKIRT_FREQUENCY, [1, 75, 25, 0]),
        (JammingTechnique::SWEPT_NOISE, [1, 80, 0, 0]),
        (JammingTechnique::FREQUENCY_SWEPT, [1, 80, 5, 0]),
        (JammingTechnique::SWEPT_AM, [1, 80, 10, 0]),
        (JammingTechnique::VELOCITY_BIN_MASKING, [1, 85, 0, 0]),
        (JammingTechnique::DECEPTION, [2, 0, 0, 0]),
        (JammingTechnique::ANALYZER, [2, 5, 0, 0]),
        (JammingTechnique::ANGLE, [2, 10, 0, 0]),
        (JammingTechnique::ANGLE_GATE_WALK_OFF, [2, 10, 5, 0]),
        (JammingTechnique::COOPERATIVE_ANGLE, [2, 10, 10, 0]),
        (JammingTechnique::CROSS_EYE, [2, 10, 15, 0]),
        (JammingTechnique::CROSS_POLARIZATION, [2, 10, 20, 0]),
        (JammingTechnique::DELTA, [2, 10, 25, 0]),
        (JammingTechnique::INVERSE_GAIN, [2, 10, 30, 0]),
        (JammingTechnique::SEA_BOUNCED, [2, 10, 35, 0]),
        (JammingTechnique::SWEPT_SQUARE_WAVE, [2, 10, 40, 0]),
        (JammingTechnique::TERRAIN_BOUNCE, [2, 10, 45, 0]),
        (JammingTechnique::ANGLE_AND_GATE_STEALER, [2, 15, 0, 0]),
        (
            JammingTechnique::CROSS_POLARIZATION_AND_RANGE_GATE_PULL_OFF,
            [2, 15, 5, 0],
        ),
        (
            JammingTechnique::CROSS_POLARIZATION_AND_VELOCITY_GATE_PULL_OFF,
            [2, 15, 10, 0],
        ),
        (
            JammingTechnique::CROSS_POLARIZATION_RGPO_AND_VGPO,
            [2, 15, 15, 0],
        ),
        (JammingTechnique::INVERSE_GAIN_AND_RGPO, [2, 15, 20, 0]),
        (
            JammingTechnique::INVERSE_GAIN_AND_RGPO_AND_VGPO,
            [2, 15, 25, 0],
        ),
        (JammingTechnique::INVERSE_GAIN_AND_VGPO, [2, 15, 30, 0]),
        (JammingTechnique::RGPO_AND_SSW, [2, 15, 35, 0]),
        (JammingTechnique::SSW_AND_VGPO, [2, 15, 40, 0]),
        (JammingTechnique::ANGLE_AND_FALSE_TARGET, [2, 20, 0, 0]),
        (
            JammingTechnique::ANGLE_AND_VELOCITY_FALSE_TARGETS,
            [2, 20, 5, 0],
        ),
        (JammingTechnique::INVERSE_GAIN_AND_VFT, [2, 20, 5, 5]),
        (JammingTechnique::SSW_AND_VFT, [2, 20, 5, 10]),
        (
            JammingTechnique::RANGE_FALSE_TARGETS_AND_INVERSE_GAIN,
            [2, 20, 10, 0],
        ),
        (JammingTechnique::RFT_AND_SSW, [2, 20, 15, 0]),
        (
            JammingTechnique::ANGLE_AND_RANDOM_RANGE_PROGRAMS,
            [2, 25, 0, 0],
        ),
        (JammingTechnique::RANRAP_AND_SSW, [2, 25, 5, 0]),
        (JammingTechnique::ANGLE_AND_VELOCITY, [2, 30, 0, 0]),
        (JammingTechnique::INVERSE_GAIN_AND_VBM, [2, 30, 5, 0]),
        (JammingTechnique::SSW_AND_VBM, [2, 30, 10, 0]),
        (JammingTechnique::AUTOMATIC_GAIN_CONTROL, [2, 35, 0, 0]),
        (JammingTechnique::AGC_AND_GATE_STEALER, [2, 40, 0, 0]),
        (JammingTechnique::AGC_AND_RGPO, [2, 40, 5, 0]),
        (JammingTechnique::AGC_AND_VGPO, [2, 40, 10, 0]),
        (JammingTechnique::AGC_AND_RGPO_AND_VGPO, [2, 40, 15, 0]),
        (JammingTechnique::COLINEAR, [2, 45, 0, 0]),
        (JammingTechnique::CONSTANT_FALSE_ALARM_RATE, [2, 50, 0, 0]),
        (JammingTechnique::DOUBLE_CROSS, [2, 55, 0, 0]),
        (JammingTechnique::DOWN_LINK, [2, 60, 0, 0]),
        (JammingTechnique::FALSE_TARGET, [2, 65, 0, 0]),
        (JammingTechnique::COHERENT_FALSE_TARGETS, [2, 65, 5, 0]),
        (JammingTechnique::FALSE_DOPPLER_TARGET, [2, 65, 10, 0]),
        (JammingTechnique::MULTIPLE_FALSE_TARGETS, [2, 65, 15, 0]),
        (JammingTechnique::RANGE_FALSE_TARGETS, [2, 65, 20, 0]),
        (JammingTechnique::TRANSPONDER, [2, 65, 25, 0]),
        (JammingTechnique::VELOCITY_FALSE_TARGETS, [2, 65, 30, 0]),
        (JammingTechnique::FIGURE_EIGHT, [2, 70, 0, 0]),
        (JammingTechnique::GATE_STEALER, [2, 75, 0, 0]),
        (JammingTechnique::CHIRP_GATE_STEALER, [2, 75, 5, 0]),
        (JammingTechnique::RANGE_GATE_PULL_OFF, [2, 75, 10, 0]),
        (JammingTechnique::RGPO_AND_VGPO, [2, 75, 15, 0]),
        (JammingTechnique::VGPO, [2, 75, 20, 0]),
        (JammingTechnique::GATE_STEALER_AND_REPEATER, [2, 80, 0, 0]),
        (
            JammingTechnique::REPEATER_SWEPT_AMPLITUDE_MODULATION_AND_VGPO,
            [2, 80, 5, 0],
        ),
        (JammingTechnique::GLINT_ENHANCE, [2, 85, 0, 0]),
        (JammingTechnique::IMAGE_FREQUENCY, [2, 90, 0, 0]),
        (
            JammingTechnique::JITTERED_PULSE_REPETITION_FREQUENCY,
            [2, 95, 0, 0],
        ),
        (JammingTechnique::JITTERED_PULSE_WIDTH, [2, 100, 0, 0]),
        (JammingTechnique::PSEUDORANDOM_NOISE, [2, 105, 0, 0]),
        (JammingTechnique::PULSE, [2, 110, 0, 0]),
        (
            JammingTechnique::PULSE_COMPRESSION_DECEPTION,
            [2, 115, 0, 0],
        ),
        (JammingTechnique::RANDOM_RANGE_PROGRAMS, [2, 120, 0, 0]),
        (JammingTechnique::REFRACTION, [2, 125, 0, 0]),
        (JammingTechnique::REPEATER, [2, 130, 0, 0]),
        (JammingTechnique::CONTINUOUS_WAVE_REPEATER, [2, 130, 5, 0]),
        (JammingTechnique::DECEPTION_REPEATER_NOISE, [2, 130, 10, 0]),
        (
            JammingTechnique::MULTIPLE_FREQUENCY_REPEATER,
            [2, 130, 15, 0],
        ),
        (
            JammingTechnique::NARROW_BAND_REPEATER_NOISE,
            [2, 130, 20, 0],
        ),
        (JammingTechnique::RANDOM_DOPPLER, [2, 130, 25, 0]),
        (
            JammingTechnique::REPEATER_DIGITAL_RADIO_FREQUENCY_MEMORY,
            [2, 130, 30, 0],
        ),
        (JammingTechnique::DRFM_LEVEL_A, [2, 130, 30, 1]),
        (JammingTechnique::DRFM_LEVEL_B, [2, 130, 30, 2]),
        (JammingTechnique::DRFM_LEVEL_C, [2, 130, 30, 3]),
        (JammingTechnique::DRFM_LEVEL_D, [2, 130, 30, 4]),
        (JammingTechnique::DRFM_LEVEL_E, [2, 130, 30, 5]),
        (
            JammingTechnique::REPEATER_SWEPT_AMPLITUDE_MODULATION,
            [2, 130, 35, 0],
        ),
        (JammingTechnique::SCINTILLATION, [2, 135, 0, 0]),
        (JammingTechnique::SERRODYNE, [2, 140, 0, 0]),
        (JammingTechnique::VELOCITY, [2, 145, 0, 0]),
        (JammingTechnique::DECEPTION_AND_NOISE, [3, 0, 0, 0]),
        (JammingTechnique::ANGLE_AND_NOISE, [3, 5, 0, 0]),
        (JammingTechnique::ANGLE_AND_BARRAGE, [3, 5, 5, 0]),
        (JammingTechnique::BARRAGE_AND_INVERSE_GAIN, [3, 5, 5, 5]),
        (JammingTechnique::BARRAGE_AND_SSW, [3, 5, 5, 10]),
        (JammingTechnique::ANGLE_AND_FM_BY_NOISE, [3, 10, 0, 0]),
        (
            JammingTechnique::FM_BY_NOISE_AND_INVERSE_GAIN,
            [3, 10, 5, 0],
        ),
        (JammingTechnique::FM_BY_NOISE_AND_SSW, [3, 10, 10, 0]),
        (JammingTechnique::ANGLE_AND_PSEUDORANDOM_AM, [3, 15, 0, 0]),
        (
            JammingTechnique::INVERSE_GAIN_AND_PSEUDORANDOM_AM,
            [3, 15, 5, 0],
        ),
        (JammingTechnique::PSEUDORANDOM_AM_AND_SSW, [3, 15, 10, 0]),
        (JammingTechnique::ANGLE_AND_SPOT, [3, 20, 0, 0]),
        (
            JammingTechnique::INVERSE_GAIN_AND_LOW_LEVEL_NOISE,
            [3, 20, 5, 0],
        ),
        (
            JammingTechnique::INVERSE_GAIN_AND_SPOT_NOISE,
            [3, 20, 10, 0],
        ),
        (JammingTechnique::SPOT_AND_SSW, [3, 20, 15, 0]),
        (JammingTechnique::GATE_STEALER_AND_NOISE, [3, 25, 0, 0]),
        (JammingTechnique::NOISE_AND_RGPO, [3, 25, 5, 0]),
        (JammingTechnique::LOW_LEVEL_NOISE_AND_RGPO, [3, 25, 5, 5]),
        (JammingTechnique::NOISE_AND_VGPO, [3, 25, 10, 0]),
        (JammingTechnique::LOW_LEVEL_NOISE_AND_VGPO, [3, 25, 10, 5]),
        (
            JammingTechnique::FALSE_TARGET_DECEPTION_AND_SWEPT_NOISE,
            [3, 30, 0, 0],
        ),
        (JammingTechnique::SPECIAL, [4, 0, 0, 0]),
        (JammingTechnique::SUPER_JAM, [4, 1, 0, 0]),
    ];

    #[test]
    fn every_constant_has_its_code_and_a_name() {
        assert_eq!(TABLE.len(), 134);
        for &(technique, [kind, category, subcategory, specific]) in TABLE {
            assert_eq!(
                technique,
                JammingTechnique::new(kind, category, subcategory, specific)
            );
            assert!(technique.name().is_some(), "{technique:?} has no name");
        }
    }

    #[test]
    fn unknown_codes_have_no_name_but_still_display() {
        let unknown = JammingTechnique::new(1, 200, 0, 0);
        assert_eq!(unknown.name(), None);
        assert_eq!(unknown.to_string(), "1.200");
        assert_eq!(JammingTechnique::default().to_string(), "0");
    }

    #[test]
    fn display_drops_trailing_zero_levels() {
        assert_eq!(JammingTechnique::NOISE.to_string(), "1 Noise");
        assert_eq!(
            JammingTechnique::BARRAGE_NOISE.to_string(),
            "1.10 Barrage Noise"
        );
        assert_eq!(
            JammingTechnique::DRFM_LEVEL_E.to_string(),
            "2.130.30.5 DRFM Level E"
        );
    }

    #[test]
    fn is_within_follows_the_hierarchy() {
        let drfm_e = JammingTechnique::DRFM_LEVEL_E;
        assert!(drfm_e.is_within(JammingTechnique::DECEPTION));
        assert!(drfm_e.is_within(JammingTechnique::REPEATER));
        assert!(drfm_e.is_within(JammingTechnique::REPEATER_DIGITAL_RADIO_FREQUENCY_MEMORY));
        assert!(drfm_e.is_within(drfm_e));
        assert!(!drfm_e.is_within(JammingTechnique::NOISE));
        assert!(!drfm_e.is_within(JammingTechnique::DRFM_LEVEL_A));
        assert!(!JammingTechnique::REPEATER.is_within(drfm_e));
        // The two techniques named "Repeater Noise" are in different kinds.
        assert!(JammingTechnique::NOISE_REPEATER_NOISE.is_within(JammingTechnique::NOISE));
        assert!(JammingTechnique::DECEPTION_REPEATER_NOISE.is_within(JammingTechnique::REPEATER));
    }

    #[test]
    fn encodes_as_four_octets_in_order() {
        let mut buf = Vec::new();
        crate::common::wire::write(&JammingTechnique::DRFM_LEVEL_E, &mut buf).unwrap();
        assert_eq!(buf, [2, 130, 30, 5]);
        let decoded: JammingTechnique = crate::common::wire::read(&mut buf.as_slice()).unwrap();
        assert_eq!(decoded, JammingTechnique::DRFM_LEVEL_E);
    }
}
