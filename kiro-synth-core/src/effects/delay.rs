use crate::float::Float;

struct DelayLine<'a, F: Float> {
  head: usize,
  buffer: &'a mut [F],
}

impl<'a, F: Float> DelayLine<'a, F> {
  pub fn new(buffer: &'a mut [F]) -> Self {
    Self { head: 0, buffer }
  }

  pub fn update(&mut self, input: F) {
    self.buffer[self.head] = input;
    self.head = (self.head + 1) % self.buffer.len();
  }

  pub fn get(&self, delay_samples: usize) -> F {
    let offset = self.buffer.len().min(delay_samples);

    let index = if offset > self.head {
      self.buffer.len() - offset + self.head
    } else {
      self.head - offset
    };
    self.buffer[index]
  }
}

pub struct Delay<'a, F: Float> {
  delay_samples: usize,
  delay_seconds: F,
  feedback: F,
  mix: F,
  delayline: DelayLine<'a, F>,
  sample_rate: F,
}

impl<'a, F: Float> Delay<'a, F> {
  pub fn new(sample_rate: F, buffer: &'a mut [F]) -> Self {
    Self {
      delay_samples: 1, // delay in samples NOTE it must never be 0
      delay_seconds: F::val(1) / sample_rate,
      feedback: F::zero(), // between 0 and 1
      mix: F::zero(),      // between 0 and 1
      delayline: DelayLine::<F>::new(buffer),
      sample_rate,
    }
  }

  pub fn set_delay_seconds(&mut self, delay_seconds: F) {
    self.delay_seconds = delay_seconds;
    self.delay_samples = (delay_seconds * self.sample_rate).to_usize().unwrap()
  }

  pub fn get_delay_seconds(&self) -> F {
    self.delay_seconds
  }

  pub fn set_feedback(&mut self, feedback: F) {
    self.feedback = feedback;
  }

  pub fn get_feedback(&self) -> F {
    self.feedback
  }

  pub fn set_mix(&mut self, mix: F) {
    self.mix = mix;
  }

  pub fn get_mix(&self) -> F {
    self.mix
  }

  pub fn process(&mut self, input: F) -> F {
    let sample = self.delayline.get(self.delay_samples);
    self.delayline.update(input + sample * self.feedback);

    sample * self.mix + input * (F::val(1.0) - self.mix)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use assert_approx_eq::assert_approx_eq;

  #[test]
  fn delayline_get() {
    let mut buffer = [4., 3., 2., 1.];
    // let delayline = DelayLine::new(1.0, &mut buffer);
    let delayline = DelayLine {
      head: 1,
      buffer: &mut buffer,
    };

    assert_approx_eq!(delayline.get(1), 4.0f64);
    assert_approx_eq!(delayline.get(2), 1.0f64);
    assert_approx_eq!(delayline.get(3), 2.0f64);
    assert_approx_eq!(delayline.get(4), 3.0f64);
    assert_approx_eq!(delayline.get(5), 3.0f64);
    assert_approx_eq!(delayline.get(6), 3.0f64);
  }

  #[test]
  fn delayline_update() {
    let mut buffer = [0.; 4];
    let mut delayline = DelayLine {
      head: 1,
      buffer: &mut buffer,
    };

    delayline.update(1.0);
    delayline.update(2.0);
    delayline.update(3.0);
    delayline.update(4.0);

    let DelayLine { head: _, buffer } = delayline;
    buffer
      .iter()
      .zip([4.0f64, 1.0f64, 2.0f64, 3.0f64].iter())
      .for_each(|(a, b)| {
        assert_approx_eq!(a, b);
      });
  }
}
