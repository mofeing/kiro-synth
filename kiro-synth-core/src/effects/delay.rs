use crate::float::Float;
use std::collections::VecDeque;

struct DelayLine<F: Float> {
  sample_rate: F,
  buffer: VecDeque<F>,
}

impl<F: Float> DelayLine<F> {
  pub fn new(sample_rate: F) -> Self {
    let delay_max = F::val(2.0);
    let len = (sample_rate * delay_max).ceil().to_usize().unwrap();
    Self {
      sample_rate,
      buffer: VecDeque::<F>::with_capacity(len),
    }
  }

  pub fn update(&mut self, input: F) {
    self.buffer.pop_front();
    self.buffer.push_back(input);
  }

  pub fn get(&self, index: F) -> F {
    let index = (index * self.sample_rate).to_usize().unwrap();
    *self.buffer.iter().nth_back(index).unwrap_or(&F::zero())
  }
}

pub struct Delay<F: Float> {
  delay: F,
  feedback: F,
  mix: F,
  delayline: DelayLine<F>,
}

impl<F: Float> Delay<F> {
  pub fn new(sample_rate: F) -> Self {
    Self {
      delay: F::one(),     // delay in seconds NOTE it must never be 0
      feedback: F::zero(), // between 0 and 1
      mix: F::zero(),      // between 0 and 1
      delayline: DelayLine::<F>::new(sample_rate),
    }
  }

  pub fn set_delay(&mut self, delay: F) {
    self.delay = delay
  }

  pub fn get_delay(&self) -> F {
    self.delay
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
    let sample = self.delayline.get(self.delay);
    self.delayline.update(input + sample * self.feedback);

    sample * self.mix + input * (F::val(1.0) - self.mix)
  }
}
