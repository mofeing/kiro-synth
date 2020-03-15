use std::sync::{Arc, Mutex};

use druid::{Widget, Data, Env, EventCtx, Event, Color, Key, PaintCtx, Rect, PaintBrush, KeyOrValue, Value};
use druid::widget::{Flex, WidgetExt, Label, Button, Controller};

use kiro_synth_core::float::Float;

use crate::synth::SynthClient;
use crate::ui::model::{SynthModel, Osc, EnvGen, KnobDataFromParam, Param, Filter, Dca, OscFromSynth};
use crate::ui::widgets::knob::{KnobData, Knob};
use crate::ui::widgets::container::Container;
use crate::ui::widgets::view_switcher::ViewSwitcher;
use crate::ui::{GREY_83, GREY_46};

struct TabController<T> {
  action: Box<dyn Fn(&mut T)>
}

impl<T: Data> TabController<T> {
  pub fn new(action: impl Fn(&mut T) + 'static) -> Self {
    TabController {
      action: Box::new(action)
    }
  }
}

impl<T, W: Widget<T>> Controller<T, W> for TabController<T> {
  fn event(
    &mut self,
    child: &mut W,
    ctx: &mut EventCtx,
    event: &Event,
    data: &mut T,
    env: &Env,
  ) {
    if let Event::MouseDown(_) = event {
      (self.action)(data);
      ctx.request_paint();
    }
    else {
      child.event(ctx, event, data, env);
    }
  }
}

const TAB_BACKGROUND: Key<Color> = Key::new("tab.background-color");

pub fn build<F: Float + 'static>(synth_model: &SynthModel,
                                 synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

  Flex::column()
    .with_child(
      build_osc_tabs(synth_model, synth_client.clone())
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

fn build_osc_tabs<F: Float + 'static>(synth_model: &SynthModel,
                                      synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<SynthModel> {

  let mut tabs = Flex::row();
  for (i, osc) in synth_model.osc.iter().enumerate() {
    let callback = move |data: &mut usize| *data = i;
    let label = Container::new(Label::new(format!("OSC{}", i + 1)).padding((6.0, 2.0, 2.0, 0.0)))
        .border(GREY_83, 2.0)
        .background_color(TAB_BACKGROUND)
        .env_scope(move |env, data| {
          if *data == i {
            env.set(TAB_BACKGROUND, GREY_83)
          } else {
            env.set(TAB_BACKGROUND, GREY_46)
          }
        })
        .controller(TabController::new(callback))
        .lens(SynthModel::osc_index);

    tabs.add_child(label, 0.0);
  }

  let osc_model: Vec<Osc> = synth_model.osc.clone();
  let switcher = ViewSwitcher::new(
    |data: &SynthModel, _env| data.osc_index,
    move |index: &usize, _env| {
      Box::new(
        build_osc_view(&osc_model[*index], synth_client.clone())
            .lens(OscFromSynth)
      )
    },
  );
  let body = Container::new(switcher.padding(6.0))
      .border(GREY_83, 2.0);

  Flex::column()
      .with_child(tabs, 0.0)
      .with_child(body, 1.0)
}

fn build_osc_view<F: Float + 'static>(osc_model: &Osc,
                                      synth_client: Arc<Mutex<SynthClient<F>>>) -> impl Widget<Osc> {

  let shape_client = synth_client.clone();
  let shape_fn = move |index: usize| shape_client.lock().unwrap().waveforms().name(index).to_string();

  Flex::row()
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
  let header = Container::new(
    Label::new(title)
        .padding((6.0, 2.0, 2.0, 0.0))
        .background(GREY_83)
        .border(GREY_83, 2.0)
  );

  let body = Container::new(widget.padding(6.0))
      // .rounded(4.0)
      .border(GREY_83, 2.0);

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
    println!("value changed {}", data.value);
    synth_client.lock().unwrap().send_param_value(param_ref, F::val(data.value));
  };

  Flex::column()
    .with_child(Label::new(title).center(), 0.0)
    .with_child(Knob::new(param.origin, param.min, param.max, param.step, callback).fix_size(48.0, 48.0).center(),0.0)
    .with_child(Label::new(move |data: &KnobData, _env: &Env| value_fn(data)).center(), 0.0)
    .lens(KnobDataFromParam)
}
