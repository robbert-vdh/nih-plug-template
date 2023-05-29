use std::sync::Arc;
{# unfortunately we need to explicitly check == "True" because Jinja is stringly typed #}
{% if cookiecutter.__has_editor == "True" %}
use atomic_float::AtomicF32;
{% endif %}
use nih_plug::prelude::*;
{% if cookiecutter.editor_gui_framework == "iced" %}
use nih_plug_iced::IcedState;
{% elif cookiecutter.editor_gui_framework == "vizia" %}
use nih_plug_vizia::ViziaState;
{% endif %}

{% if cookiecutter.__has_editor == "True" %}
mod editor;

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
const PEAK_METER_DECAY_MS: f64 = 150.0;
{% endif %}

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

struct {{ cookiecutter.struct_name }} {
    params: Arc<{{ cookiecutter.struct_name }}Params>,
    {% if cookiecutter.__has_editor == "True" %}
    /// Needed to normalize the peak meter's response based on the sample rate.
    peak_meter_decay_weight: f32,
    /// The current data for the peak meter. This is stored as an [`Arc`] so we can share it between
    /// the GUI and the audio processing parts. If you have more state to share, then it's a good
    /// idea to put all of that in a struct behind a single `Arc`.
    ///
    /// This is stored as voltage gain.
    peak_meter: Arc<AtomicF32>,
    {% endif %}
}

#[derive(Params)]
struct {{ cookiecutter.struct_name }}Params {
    {% if cookiecutter.__has_editor == "True" %}
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    {% if cookiecutter.editor_gui_framework == "iced" %}
    editor_state: Arc<IcedState>,
    {% elif cookiecutter.editor_gui_framework == "vizia" %}
    editor_state: Arc<ViziaState>,
    {% endif %}
    {% endif %}

    /// The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
    /// these IDs remain constant, you can rename and reorder these fields as you wish. The
    /// parameters are exposed to the host in the same order they were defined. In this case, this
    /// gain parameter is stored as linear gain while the values are displayed in decibels.
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for {{ cookiecutter.struct_name }} {
    fn default() -> Self {
        Self {
            params: Arc::new({{ cookiecutter.struct_name }}Params::default()),
            {% if cookiecutter.__has_editor == "True" %}
            peak_meter_decay_weight: 1.0,
            peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            {% endif %}
        }
    }
}

impl Default for {{ cookiecutter.struct_name }}Params {
    fn default() -> Self {
        Self {
            {% if cookiecutter.__has_editor == "True" %}
            editor_state: editor::default_state(),
            {% endif %}

            // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
            // to treat these kinds of parameters as if we were dealing with decibels. Storing this
            // as decibels is easier to work with, but requires a conversion for every sample.
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            // Because the gain parameter is stored as linear gain instead of storing the value as
            // decibels, we need logarithmic smoothing
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            // There are many predefined formatters we can use here. If the gain was stored as
            // decibels instead of as a linear gain value, we could have also used the
            // `.with_step_size(0.1)` function to get internal rounding.
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for {{ cookiecutter.struct_name }} {
    const NAME: &'static str = "{{ cookiecutter.plugin_name }}";
    const VENDOR: &'static str = "{{ cookiecutter.author }}";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "{{ cookiecutter.email_address }}";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[],
            aux_output_ports: &[],

            // Individual ports and the layout as a whole can be named here. By default these names
            // are generated as needed. This layout will be called 'Stereo', while a layout with
            // only one input and output channel would be called 'Mono'.
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];


    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    {% if cookiecutter.__has_editor == "True" %}
    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.peak_meter.clone(),
            self.params.editor_state.clone(),
        )
    }
    {% endif %}

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        {# deal with the leading underscore depending on whether the variable is unused #}
        {% if cookiecutter.__has_editor == "True" %}
        buffer_config: &BufferConfig,
        {% else %}
        _buffer_config: &BufferConfig,
        {% endif %}
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.

        {% if cookiecutter.__has_editor == "True" %}
        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB
        self.peak_meter_decay_weight = 0.25f64
            .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32;
        {% endif %}

        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            {% if cookiecutter.__has_editor == "True" %}
            let mut amplitude = 0.0;
            let num_samples = channel_samples.len();
            {% endif %}

            // Smoothing is optionally built into the parameters themselves
            let gain = self.params.gain.smoothed.next();
            for sample in channel_samples {
                *sample *= gain;
                {% if cookiecutter.__has_editor == "True" %}
                amplitude += *sample;
                {% endif %}
            }
            {% if cookiecutter.__has_editor == "True" %}
            // To save resources, a plugin can (and probably should!) only perform expensive
            // calculations that are only displayed on the GUI while the GUI is open
            if self.params.editor_state.is_open() {
                amplitude = (amplitude / num_samples as f32).abs();
                let current_peak_meter = self.peak_meter.load(std::sync::atomic::Ordering::Relaxed);
                let new_peak_meter = if amplitude > current_peak_meter {
                    amplitude
                } else {
                    current_peak_meter * self.peak_meter_decay_weight
                        + amplitude * (1.0 - self.peak_meter_decay_weight)
                };

                self.peak_meter
                    .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed)
            }
            {% endif %}
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for {{ cookiecutter.struct_name }} {
    const CLAP_ID: &'static str = "{{ cookiecutter.clap_id }}";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("{{ cookiecutter.description }}");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // TODO: Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility
    ];
}

impl Vst3Plugin for {{ cookiecutter.struct_name }} {
    const VST3_CLASS_ID: [u8; 16] = *b"{{ cookiecutter.vst3_id }}";

    // TODO: And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!({{ cookiecutter.struct_name }});
nih_export_vst3!({{ cookiecutter.struct_name }});
