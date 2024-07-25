#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

use core::f32::consts::PI;

use micromath::F32Ext;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Filter types.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FilterType {
    /// Bypass.
    #[default]
    Bypass,

    /// Low-pass.
    LowPass {
        /// Cutoff frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,
    },

    /// High-pass.
    HighPass {
        /// Cutoff frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,
    },

    /// Band-pass.
    BandPass {
        /// Center frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,
    },

    /// Notch.
    Notch {
        /// Center frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,
    },

    /// Peaking EQ.
    PeakingEq {
        /// Center frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,

        /// Gain in dB.
        gain: f32,
    },

    /// Low-shelf.
    LowShelf {
        /// Corner frequency in Hz.
        freq: f32,

        /// Gain in dB.
        gain: f32,
    },

    /// High-shelf.
    HighShelf {
        /// Corner frequency in Hz.
        freq: f32,

        /// Gain in dB.
        gain: f32,
    },

    /// All-pass.
    AllPass {
        /// Center frequency in Hz.
        freq: f32,

        /// Q value.
        q: f32,
    },

    /// 1st order low-pass.
    FirstOrderLowPass {
        /// Cutoff frequency in Hz.
        freq: f32,
    },

    /// 1st order high-pass.
    FirstOrderHighPass {
        /// Cutoff frequency in Hz.
        freq: f32,
    },

    /// 1st order low-shelf.
    FirstOrderLowShelf {
        /// Corner frequency in Hz.
        freq: f32,

        /// Gain in dB.
        gain: f32,
    },

    /// 1st order high-shelf.
    FirstOrderHighShelf {
        /// Corner frequency in Hz.
        freq: f32,

        /// Gain in dB.
        gain: f32,
    },

    /// 1st order all-pass.
    FirstOrderAllPass {
        /// Center frequency in Hz.
        freq: f32,
    },

    /// One-pole low-pass.
    OnePoleLowPass {
        /// Cutoff frequency in Hz.
        freq: f32,
    },
}

/// Normalized filter coefficients.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FilterCoefficients {
    /// Coefficient a0 / b0.
    a0: f32,

    /// Coefficient a1 / b0.
    a1: f32,

    /// Coefficient a2 / b0.
    a2: f32,

    /// Coefficient b1 / b0.
    b1: f32,

    /// Coefficient b2 / b0.
    b2: f32,
}

impl Default for FilterCoefficients {
    fn default() -> Self {
        Self {
            a0: 1.0,
            a1: 0.0,
            a2: 0.0,
            b1: 0.0,
            b2: 0.0,
        }
    }
}

impl FilterCoefficients {
    /// Calculates the coefficients from the filter type.
    ///
    /// `sample_time` is `1.0 / sample_rate`.
    pub fn from_type(filter_type: FilterType, sample_time: f32) -> FilterCoefficients {
        match filter_type {
            FilterType::Bypass => FilterCoefficients::default(),
            FilterType::LowPass { freq, q } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (1.0 + k / q + k * k);
                let a0 = k * k * norm;
                Self {
                    a0,
                    a1: 2.0 * a0,
                    a2: a0,
                    b1: 2.0 * (k * k - 1.0) * norm,
                    b2: (1.0 - k / q + k * k) * norm,
                }
            }
            FilterType::HighPass { freq, q } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (1.0 + k / q + k * k);
                let a0 = norm;
                Self {
                    a0,
                    a1: -2.0 * a0,
                    a2: a0,
                    b1: 2.0 * (k * k - 1.0) * norm,
                    b2: (1.0 - k / q + k * k) * norm,
                }
            }
            FilterType::BandPass { freq, q } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (1.0 + k / q + k * k);
                let a0 = k / q * norm;
                Self {
                    a0,
                    a1: 0.0,
                    a2: -a0,
                    b1: 2.0 * (k * k - 1.0) * norm,
                    b2: (1.0 - k / q + k * k) * norm,
                }
            }
            FilterType::Notch { freq, q } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (1.0 + k / q + k * k);
                let a0 = (1.0 + k * k) * norm;
                let a1 = 2.0 * (k * k - 1.0) * norm;
                Self {
                    a0,
                    a1,
                    a2: a0,
                    b1: a1,
                    b2: (1.0 - k / q + k * k) * norm,
                }
            }
            FilterType::PeakingEq { freq, q, gain } => {
                let k = (PI * freq * sample_time).tan();
                let v = 10.0.powf(gain.abs() / 20.0);
                if gain >= 0.0 {
                    let norm = 1.0 / (1.0 + 1.0 / q * k + k * k);
                    let a1 = 2.0 * (k * k - 1.0) * norm;
                    Self {
                        a0: (1.0 + v / q * k + k * k) * norm,
                        a1,
                        a2: (1.0 - v / q * k + k * k) * norm,
                        b1: a1,
                        b2: (1.0 - 1.0 / q * k + k * k) * norm,
                    }
                } else {
                    let norm = 1.0 / (1.0 + v / q * k + k * k);
                    let a1 = 2.0 * (k * k - 1.0) * norm;
                    Self {
                        a0: (1.0 + 1.0 / q * k + k * k) * norm,
                        a1,
                        a2: (1.0 - 1.0 / q * k + k * k) * norm,
                        b1: a1,
                        b2: (1.0 - v / q * k + k * k) * norm,
                    }
                }
            }
            FilterType::LowShelf { freq, gain } => {
                let k = (PI * freq * sample_time).tan();
                let v = 10.0.powf(gain.abs() / 20.0);
                if gain >= 0.0 {
                    let norm = 1.0 / (1.0 + 2.0.sqrt() * k + k * k);
                    Self {
                        a0: (1.0 + (2.0 * v).sqrt() * k + v * k * k) * norm,
                        a1: 2.0 * (v * k * k - 1.0) * norm,
                        a2: (1.0 - (2.0 * v).sqrt() * k + v * k * k) * norm,
                        b1: 2.0 * (k * k - 1.0) * norm,
                        b2: (1.0 - 2.0.sqrt() * k + k * k) * norm,
                    }
                } else {
                    let norm = 1.0 / (1.0 + (2.0 * v).sqrt() * k + v * k * k);
                    Self {
                        a0: (1.0 + 2.0.sqrt() * k + k * k) * norm,
                        a1: 2.0 * (k * k - 1.0) * norm,
                        a2: (1.0 - 2.0.sqrt() * k + k * k) * norm,
                        b1: 2.0 * (v * k * k - 1.0) * norm,
                        b2: (1.0 - (2.0 * v).sqrt() * k + v * k * k) * norm,
                    }
                }
            }
            FilterType::HighShelf { freq, gain } => {
                let k = (PI * freq * sample_time).tan();
                let v = 10.0.powf(gain.abs() / 20.0);
                if gain >= 0.0 {
                    let norm = 1.0 / (1.0 + 2.0.sqrt() * k + k * k);
                    Self {
                        a0: (v + (2.0 * v).sqrt() * k + k * k) * norm,
                        a1: 2.0 * (k * k - v) * norm,
                        a2: (v - (2.0 * v).sqrt() * k + k * k) * norm,
                        b1: 2.0 * (k * k - 1.0) * norm,
                        b2: (1.0 - 2.0.sqrt() * k + k * k) * norm,
                    }
                } else {
                    let norm = 1.0 / (v + (2.0 * v).sqrt() * k + k * k);
                    Self {
                        a0: (1.0 + 2.0.sqrt() * k + k * k) * norm,
                        a1: 2.0 * (k * k - 1.0) * norm,
                        a2: (1.0 - 2.0.sqrt() * k + k * k) * norm,
                        b1: 2.0 * (k * k - v) * norm,
                        b2: (v - (2.0 * v).sqrt() * k + k * k) * norm,
                    }
                }
            }
            FilterType::AllPass { freq, q } => {
                let k = (PI * freq * sample_time).tan();
                let div_q = 1.0 / q;
                let norm = 1.0 / (1.0 + k * div_q + k * k);
                let a0 = (1.0 - k * div_q + k * k) * norm;
                let a1 = 2.0 * (k * k - 1.0) * norm;
                Self {
                    a0,
                    a1,
                    a2: 1.0,
                    b1: a1,
                    b2: a0,
                }
            }
            FilterType::FirstOrderLowPass { freq } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (1.0 / k + 1.0);
                Self {
                    a0: norm,
                    a1: norm,
                    a2: 0.0,
                    b1: (1.0 - 1.0 / k) * norm,
                    b2: 0.0,
                }
            }
            FilterType::FirstOrderHighPass { freq } => {
                let k = (PI * freq * sample_time).tan();
                let norm = 1.0 / (k + 1.0);
                Self {
                    a0: norm,
                    a1: -norm,
                    a2: 0.0,
                    b1: (k - 1.0) * norm,
                    b2: 0.0,
                }
            }
            FilterType::FirstOrderLowShelf { freq, gain } => {
                let k = (PI * freq * sample_time).tan();
                let v = 10.0.powf(gain.abs() / 20.0);
                if gain >= 0.0 {
                    let norm = 1.0 / (k + 1.0);
                    Self {
                        a0: (k * v + 1.0) * norm,
                        a1: (k * v - 1.0) * norm,
                        a2: 0.0,
                        b1: (k - 1.0) * norm,
                        b2: 0.0,
                    }
                } else {
                    let norm = 1.0 / (k * v + 1.0);
                    Self {
                        a0: (k + 1.0) * norm,
                        a1: (k - 1.0) * norm,
                        a2: 0.0,
                        b1: (k * v - 1.0) * norm,
                        b2: 0.0,
                    }
                }
            }
            FilterType::FirstOrderHighShelf { freq, gain } => {
                let k = (PI * freq * sample_time).tan();
                let v = 10.0.powf(gain.abs() / 20.0);
                if gain >= 0.0 {
                    let norm = 1.0 / (k + 1.0);
                    Self {
                        a0: (k + v) * norm,
                        a1: (k - v) * norm,
                        a2: 0.0,
                        b1: (k - 1.0) * norm,
                        b2: 0.0,
                    }
                } else {
                    let norm = 1.0 / (k + v);
                    Self {
                        a0: (k + 1.0) * norm,
                        a1: (k - 1.0) * norm,
                        a2: 0.0,
                        b1: (k - v) * norm,
                        b2: 0.0,
                    }
                }
            }
            FilterType::FirstOrderAllPass { freq } => {
                let k = (PI * freq * sample_time).tan();
                let a0 = (1.0 - k) / (1.0 + k);
                Self {
                    a0,
                    a1: -1.0,
                    a2: 0.0,
                    b1: -a0,
                    b2: 0.0,
                }
            }
            FilterType::OnePoleLowPass { freq } => {
                let b1 = (-2.0 * PI * freq * sample_time).exp();
                Self {
                    a0: 1.0 - b1,
                    a1: 0.0,
                    a2: 0.0,
                    b1: -b1,
                    b2: 0.0,
                }
            }
        }
    }
}

/// Direct form 1.
#[derive(Debug, Default, Clone)]
pub struct DirectForm1 {
    /// Coefficients.
    coeffs: FilterCoefficients,

    /// Input sample memory.
    in_states: [f32; 2],

    /// Output sample memory.
    out_states: [f32; 2],
}

impl DirectForm1 {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the filter to bypass mode.
    pub fn reset(&mut self) {
        self.set_coefficients(FilterCoefficients::default());
    }

    /// Sets the coefficients.
    pub fn set_coefficients(&mut self, coeffs: FilterCoefficients) {
        self.coeffs = coeffs;
    }

    /// Processes a single sample.
    pub fn process_sample(&mut self, sample: f32) -> f32 {
        let out_sample = self.coeffs.a0 * sample
            + self.coeffs.a1 * self.in_states[0]
            + self.coeffs.a2 * self.in_states[1]
            - self.coeffs.b1 * self.out_states[0]
            - self.coeffs.b2 * self.out_states[1];

        self.in_states[1] = self.in_states[0];
        self.in_states[0] = sample;

        self.out_states[1] = self.out_states[0];
        self.out_states[0] = out_sample;

        out_sample
    }

    /// Processes a block of samples in-place.
    pub fn process_block(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }
}

/// Transposed direct form 2.
#[derive(Debug, Default, Clone)]
pub struct DirectForm2Transposed {
    /// Coefficients.
    coeffs: FilterCoefficients,

    /// Sample memory.
    states: [f32; 2],
}

impl DirectForm2Transposed {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the filter to bypass mode.
    pub fn reset(&mut self) {
        self.set_coefficients(FilterCoefficients::default());
    }

    /// Sets the coefficients.
    pub fn set_coefficients(&mut self, coeffs: FilterCoefficients) {
        self.coeffs = coeffs;
    }

    /// Processes a single sample.
    pub fn process_sample(&mut self, sample: f32) -> f32 {
        let out_sample = self.states[0] + self.coeffs.a0 * sample;

        self.states[0] = self.states[1] + self.coeffs.a1 * sample - self.coeffs.b1 * out_sample;
        self.states[1] = self.coeffs.a2 * sample - self.coeffs.b2 * out_sample;

        out_sample
    }

    /// Processes a block of samples in-place.
    pub fn process_block(&mut self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample = self.process_sample(*sample);
        }
    }
}
