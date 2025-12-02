//! Audio playback backend using rodio.

#[cfg(feature = "audio")]
pub mod backend {
    use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    /// Audio player state
    pub struct AudioPlayer {
        _stream: OutputStream,
        stream_handle: OutputStreamHandle,
        sink: Arc<Mutex<Option<Sink>>>,
        volume: f32,
    }

    impl AudioPlayer {
        /// Create a new audio player
        pub fn new() -> Result<Self, String> {
            let (stream, stream_handle) = OutputStream::try_default()
                .map_err(|e| format!("Failed to create audio output: {}", e))?;

            Ok(Self {
                _stream: stream,
                stream_handle,
                sink: Arc::new(Mutex::new(None)),
                volume: 1.0,
            })
        }

        /// Play a test tone (440Hz sine wave)
        pub fn play_test_tone(&self, duration_secs: u64) {
            let source = rodio::source::SineWave::new(440.0)
                .take_duration(Duration::from_secs(duration_secs))
                .amplify(0.3);

            if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                sink.set_volume(self.volume);
                sink.append(source);
                *self.sink.lock().unwrap() = Some(sink);
            }
        }

        /// Play audio from bytes
        pub fn play_bytes(&self, data: &[u8]) -> Result<(), String> {
            let cursor = std::io::Cursor::new(data.to_vec());
            let source = rodio::Decoder::new(cursor)
                .map_err(|e| format!("Failed to decode audio: {}", e))?;

            if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                sink.set_volume(self.volume);
                sink.append(source);
                *self.sink.lock().unwrap() = Some(sink);
                Ok(())
            } else {
                Err("Failed to create audio sink".to_string())
            }
        }

        /// Pause playback
        pub fn pause(&self) {
            if let Some(ref sink) = *self.sink.lock().unwrap() {
                sink.pause();
            }
        }

        /// Resume playback
        pub fn resume(&self) {
            if let Some(ref sink) = *self.sink.lock().unwrap() {
                sink.play();
            }
        }

        /// Stop playback
        pub fn stop(&self) {
            if let Some(ref sink) = *self.sink.lock().unwrap() {
                sink.stop();
            }
            *self.sink.lock().unwrap() = None;
        }

        /// Set volume (0.0 to 1.0)
        pub fn set_volume(&mut self, volume: f32) {
            self.volume = volume.clamp(0.0, 1.0);
            if let Some(ref sink) = *self.sink.lock().unwrap() {
                sink.set_volume(self.volume);
            }
        }

        /// Check if playing
        pub fn is_playing(&self) -> bool {
            if let Some(ref sink) = *self.sink.lock().unwrap() {
                !sink.is_paused() && !sink.empty()
            } else {
                false
            }
        }

        /// Check if paused
        pub fn is_paused(&self) -> bool {
            if let Some(ref sink) = *self.sink.lock().unwrap() {
                sink.is_paused()
            } else {
                false
            }
        }
    }

    impl Default for AudioPlayer {
        fn default() -> Self {
            Self::new().expect("Failed to create audio player")
        }
    }
}

#[cfg(not(feature = "audio"))]
pub mod backend {
    /// Stub audio player when audio feature is disabled
    pub struct AudioPlayer;

    impl AudioPlayer {
        pub fn new() -> Result<Self, String> {
            Ok(Self)
        }

        pub fn play_test_tone(&self, _duration_secs: u64) {}
        pub fn play_bytes(&self, _data: &[u8]) -> Result<(), String> {
            Err("Audio feature not enabled".to_string())
        }
        pub fn pause(&self) {}
        pub fn resume(&self) {}
        pub fn stop(&self) {}
        pub fn set_volume(&mut self, _volume: f32) {}
        pub fn is_playing(&self) -> bool { false }
        pub fn is_paused(&self) -> bool { false }
    }

    impl Default for AudioPlayer {
        fn default() -> Self {
            Self
        }
    }
}

pub use backend::AudioPlayer;
