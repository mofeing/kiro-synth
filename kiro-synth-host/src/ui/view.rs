use std::sync::{Arc, Mutex};

use druid::{Widget, Color, Data, Env};
use druid::widget::{Flex, WidgetExt, Label, Container};

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;
use crate::ui::model::{SynthModel, Osc, EnvGen, ParamToKnobData, Param, Filter, Dca};
use crate::ui::widgets::knob::{KnobData, Knob};


pub fn build<F: Float + 'static>(synth_model: &SynthModel,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

  Flex::column()
    .with_child(
      build_osc("OSC1", &synth_model.osc1, synth_client.clone())
              .lens(SynthModel::osc1)
              .padding(6.0),
      1.0
    )
    .with_child(
      build_osc("OSC2", &synth_model.osc2, synth_client.clone())
              .lens(SynthModel::osc2)
              .padding(6.0),
      1.0
    )
    .with_child(
      build_eg("EG1", &synth_model.eg1, synth_client.clone())
              .lens(SynthModel::eg1)
              .padding(6.0),
      1.0
    )
    .with_child(
      Flex::row()
          .with_child(
            build_filt("FILT1", &synth_model.filt1, synth_client.clone())
                .lens(SynthModel::filt1)
                .padding(6.0),
            1.0
          )
          .with_child(
            build_dca("DCA", &synth_model.dca, synth_client.clone())
                .lens(SynthModel::dca)
                .padding(6.0),
            1.0
          ),
      1.0
    )
}

fn build_osc<F: Float + 'static>(title: &str,
                                 osc_model: &Osc,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Osc> {

  let shape_client = synth_client.clone();
  let shape_fn = move |index: usize| shape_client.lock().unwrap().waveforms().name(index).to_string();

  build_panel(title, Flex::row()
    .with_child(
      build_knob_enum("Shape", shape_fn, &osc_model.shape, synth_client.clone())
            .lens(Osc::shape),
      1.0
    )
    .with_child(
      build_knob_value("Octaves", "", &osc_model.octaves, synth_client.clone())
            .lens(Osc::octaves),
      1.0
    )
    .with_child(
      build_knob_value("Semitones", "", &osc_model.semitones, synth_client.clone())
            .lens(Osc::semitones),
      1.0
    )
    .with_child(
      build_knob_value("Cents", "", &osc_model.cents, synth_client.clone())
            .lens(Osc::cents),
      1.0
    )
    .with_child(
      build_knob_value("Amplitude", "", &osc_model.amplitude, synth_client.clone())
          .lens(Osc::amplitude),
      1.0
    )
  )
}

fn build_eg<F: Float + 'static>(title: &str,
                                eg_model: &EnvGen,
                                synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<EnvGen> {

  build_panel(title, Flex::row()
      .with_child(
        build_knob_value("Attack", " s", &eg_model.attack, synth_client.clone())
              .lens(EnvGen::attack),
        1.0
      )
      .with_child(
        build_knob_value("Decay", " s", &eg_model.decay, synth_client.clone())
              .lens(EnvGen::decay),
        1.0
      )
      .with_child(
        build_knob_value("Sustain", "", &eg_model.sustain, synth_client.clone())
              .lens(EnvGen::sustain),
        1.0
      )
      .with_child(
        build_knob_value("Release", " s", &eg_model.release, synth_client.clone())
              .lens(EnvGen::release),
        1.0
      )
      .with_child(
        build_knob_value("Mode", "", &eg_model.mode, synth_client.clone())
              .lens(EnvGen::mode),
        1.0
      )
      .with_child(
        build_knob_value("Intensity", "", &eg_model.dca_intensity, synth_client.clone())
              .lens(EnvGen::dca_intensity),
        1.0
      )
  )
}

fn build_filt<F: Float + 'static>(title: &str,
                                  filt_model: &Filter,
                                  synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Filter> {

  build_panel(title, Flex::row()
    .with_child(
      build_knob_value("Mode", "", &filt_model.mode, synth_client.clone())
            .lens(Filter::mode),
      1.0
    )
    .with_child(
      build_knob_value("Cutoff", " Hz", &filt_model.freq, synth_client.clone())
            .lens(Filter::freq),
      1.0
    )
    .with_child(
      build_knob_value("Res", "", &filt_model.q, synth_client.clone())
            .lens(Filter::q),
      1.0
    )
  )
}

fn build_dca<F: Float + 'static>(title: &str,
                                 dca_model: &Dca,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Dca> {

  build_panel(title, Flex::row()
      .with_child(
        build_knob_value("Amplitude", " dB", &dca_model.amplitude, synth_client.clone())
              .lens(Dca::amplitude),
        1.0
      )
      .with_child(
        build_knob_value("Pan", "", &dca_model.pan, synth_client.clone())
              .lens(Dca::pan),
        1.0
      )
  )
}

fn build_panel<T: Data>(title: &str, widget: impl Widget<T> + 'static) -> impl Widget<T> {
  let header = Container::new(Label::new(title).padding((8.0, 4.0, 0.0, 2.0)))
      .rounded(4.0)
      // .background(Color::WHITE)
      .border(Color::WHITE, 1.0);

  let body = Container::new(widget.padding(6.0))
      .rounded(4.0)
      .border(Color::WHITE, 1.0);

  Flex::column()
      .with_child(header, 0.0)
      .with_child(body, 1.0)
}

fn build_knob_value<F: Float + 'static>(title: &'static str,
                                        unit: &'static str,
                                        param: &Param,
                                        synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Param> {

  let step = param.step.max(0.001);
  let precision = (-step.log10().floor()).max(0.0).min(3.0) as usize;
  let value_fn = move |data: &KnobData| {
    let value = (data.value / step).round() * step;
    format!("{:.*}{}", precision, value, unit)
  };

  build_knob(title, value_fn, param, synth_client)
}

fn build_knob_enum<F: Float + 'static>(title: &'static str,
                                       value_fn: impl Fn(usize) -> String + 'static,
                                       param: &Param,
                                       synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Param> {

  build_knob(title, move |data: &KnobData| value_fn(data.value as usize), param, synth_client)
}

fn build_knob<F: Float + 'static>(title: &'static str,
                                  value_fn: impl Fn(&KnobData) -> String + 'static,
                                  param: &Param,
                                  synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Param> {

  let param_ref = param.param_ref;
  let callback = move |data: &KnobData| {
    synth_client.lock().unwrap().send_param_value(param_ref, F::val(data.value));
  };

  Flex::column()
    .with_child(Label::new(title).center(), 0.0)
    .with_child(
      Knob::new(param.origin, param.min, param.max, param.step, callback)
              .fix_size(48.0, 48.0),
      0.0
    )
    .with_child(Label::new(move |data: &KnobData, _env: &Env| value_fn(data)).center(), 0.0)
    .lens(ParamToKnobData)
}