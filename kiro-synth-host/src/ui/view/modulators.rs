use std::sync::{Arc, Mutex};

use druid::{Widget, Env};
use druid::widget::{Flex, WidgetExt};

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;
use crate::ui::model::{SynthModel, EnvGen, EgFromSynth, Lfo, LfoFromSynth};
use crate::ui::view::{build_tabs, build_switcher, build_knob_value, build_knob_enum};


pub struct ModulatorsView;

impl ModulatorsView {
  pub fn new<F: Float + 'static>(synth_model: &SynthModel,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

    let eg_len = synth_model.eg.lock().unwrap().len();
    let tabs_len = eg_len + synth_model.lfo.lock().unwrap().len();
    let tab_title = move |index| {
      if index < eg_len {
        format!("EG{}", index + 1)
      }
      else {
        format!("LFO{}", index - eg_len + 1)
      }
    };

    let tabs = build_tabs(tabs_len, tab_title)
        .lens(SynthModel::mod_index);

    build_switcher(tabs,
                   |data: &SynthModel, _env: &Env| data.mod_index,
                   move |index: &usize, _data: &SynthModel, _env: &Env| {
                     if *index < eg_len {
                       Box::new(build_eg_view().lens(EgFromSynth))
                     }
                     else {
                       Box::new(build_lfo_view(synth_client.clone()).lens(LfoFromSynth))
                     }
                   })
  }
}

fn build_eg_view() -> impl Widget<EnvGen> {

  let row1 = Flex::row()
      .with_child(
        build_knob_value("Attack", " s").lens(EnvGen::attack)
      )
      .with_child(
        build_knob_value("Decay", " s").lens(EnvGen::decay)
      )
      .with_child(
        build_knob_value("Sustain", "").lens(EnvGen::sustain)
      )
      .with_child(
        build_knob_value("Release", " s").lens(EnvGen::release)
      )
      .with_flex_spacer(1.0);

  let row2 = Flex::row()
      .with_child(
        build_knob_value("Mode", "").lens(EnvGen::mode)
      )
      .with_child(
        build_knob_value("Intensity", "").lens(EnvGen::dca_intensity)
      )
      .with_flex_spacer(1.0);

  Flex::column()
      .with_child(row1)
      .with_spacer(10.0)
      .with_child(row2)
}

fn build_lfo_view<F: Float + 'static>(synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Lfo> {

  let shape_client = synth_client.clone();
  let shape_fn = move |index: usize| shape_client.lock().unwrap().lfo_waveforms().name(index).to_string();

  let row1 = Flex::row()
      .with_child(
        build_knob_enum("Shape", shape_fn).lens(Lfo::shape)
      )
      .with_child(
        build_knob_value("Rate", " Hz").lens(Lfo::rate)
      )
      .with_child(
        build_knob_value("Phase", "").lens(Lfo::phase)
      )
      .with_child(
        build_knob_value("Depth", "").lens(Lfo::depth)
      )
      .with_flex_spacer(1.0);

  let row2 = Flex::row()
      .with_child(
        build_knob_value("Osc Pitch", "").lens(Lfo::osc_pitch_mod)
      )
      .with_child(
        build_knob_value("F. Cutoff", "").lens(Lfo::filter_cutoff_mod)
      )
      .with_child(
        build_knob_value("DCA Amp", "").lens(Lfo::dca_amp_mod)
      )
      .with_child(
        build_knob_value("DCA Pan", "").lens(Lfo::dca_pan_mod)
      )
      .with_flex_spacer(1.0);

  Flex::column()
      .with_child(row1)
      .with_spacer(10.0)
      .with_child(row2)
}
