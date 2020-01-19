use crate::float::Float;

/// Calculates the pitch shift multiplier from the following parameters:
/// - A number of octaves
/// - A number of semitones
/// - A number of cents of semitone
/// - The pitch bend
/// - The frequency modulation
///
#[derive(Debug)]
pub struct OscPitchShift<F: Float> {
  /// shift in semitones for the number of octaves
  octaves_shift: F,
  /// shift in semitones
  semitones_shift: F,
  /// shift in semitones for the cents
  cents_shift: F,
  /// pitch bend
  pitch_bend: F,
  /// frequency modulation (exponential)
  modulation: F,
}

impl<F> Default for OscPitchShift<F>
where
  F: Float,
{
  fn default() -> Self {
    OscPitchShift {
      octaves_shift: F::zero(),
      semitones_shift: F::zero(),
      cents_shift: F::zero(),
      pitch_bend: F::zero(),
      modulation: F::zero(),
    }
  }
}

impl<F> OscPitchShift<F>
where
  F: Float,
{
  /// Set the shift for the octaves
  pub fn set_octaves(&mut self, octaves: F) {
    self.octaves_shift = octaves * F::from(12.0).unwrap();
  }

  /// Set the semitones shift
  pub fn set_semitones(&mut self, semitones: F) {
    self.semitones_shift = semitones;
  }

  /// Set the shift for the cents
  pub fn set_cents(&mut self, cents: F) {
    self.cents_shift = cents * F::from(0.01).unwrap();
  }

  /// Set the pitch bend
  pub fn set_pitch_bend(&mut self, pitch_bend: F) {
    self.pitch_bend = pitch_bend;
  }

  /// Set the frequency modulation
  pub fn set_modulation(&mut self, modulation: F) {
    self.modulation = modulation;
  }

  /// The multiplier for the configured pitch shift
  pub fn multiplier(&self) -> F {
    let total_semitones_shift = self.octaves_shift
      + self.semitones_shift
      + self.cents_shift
      + self.pitch_bend
      + self.modulation;

    F::from(2.0)
      .unwrap()
      .powf(total_semitones_shift / F::from(12.0).unwrap())
  }
}
