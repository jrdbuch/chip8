use sdl2::Sdl;
use sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};


pub struct SoundDriver {
    device: AudioDevice<SquareWave>,
    pub on: bool,
}


struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}


impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = match self.phase {
                0.0..=0.5 => self.volume,
                _ => -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}


impl SoundDriver {
    pub fn new(sdl: &Sdl) -> SoundDriver {
        let audio_subsystem = sdl.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        SoundDriver{device, on: false}
    }

    pub fn resume(&mut self) {
        self.device.resume();
        self.on = true;
    }

    pub fn pause(&mut self) {
        self.device.pause();
        self.on = false;
    }
}