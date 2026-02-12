//! Webcam capture backend using nokhwa.

#[cfg(feature = "webcam")]
pub mod backend {
    use nokhwa::pixel_format::RgbFormat;
    use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
    use nokhwa::Camera;
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    };
    use std::thread::{self, JoinHandle};
    use std::time::Duration;

    struct FrameData {
        data: Vec<u8>,
        width: u32,
        height: u32,
    }

    /// Webcam capture state
    pub struct WebcamCapture {
        running: Arc<AtomicBool>,
        stop_flag: Arc<AtomicBool>,
        frame: Arc<Mutex<Option<FrameData>>>,
        resolution: Arc<Mutex<(u32, u32)>>,
        worker: Option<JoinHandle<()>>,
    }

    impl WebcamCapture {
        /// Create a new webcam capture
        pub fn new() -> Self {
            Self {
                running: Arc::new(AtomicBool::new(false)),
                stop_flag: Arc::new(AtomicBool::new(false)),
                frame: Arc::new(Mutex::new(None)),
                resolution: Arc::new(Mutex::new((640, 480))),
                worker: None,
            }
        }

        /// Start the webcam
        pub fn start(&mut self) -> Result<(), String> {
            if self.is_running() {
                return Ok(());
            }

            let stop_flag = Arc::clone(&self.stop_flag);
            stop_flag.store(false, Ordering::SeqCst);
            let running = Arc::clone(&self.running);
            let frame_store = Arc::clone(&self.frame);
            let resolution = Arc::clone(&self.resolution);
            let (ready_tx, ready_rx) = mpsc::channel();

            let handle = thread::Builder::new()
                .name("webcam-capture".to_string())
                .spawn(move || {
                    let index = CameraIndex::Index(0);
                    let requested = RequestedFormat::new::<RgbFormat>(
                        RequestedFormatType::AbsoluteHighestFrameRate,
                    );

                    let mut camera = match Camera::new(index, requested) {
                        Ok(camera) => camera,
                        Err(err) => {
                            let _ = ready_tx.send(Err(format!("Failed to open camera: {err}")));
                            return;
                        }
                    };

                    if let Err(err) = camera.open_stream() {
                        let _ = ready_tx.send(Err(format!("Failed to start camera stream: {err}")));
                        return;
                    }

                    let res = camera.resolution();
                    *resolution.lock().unwrap() = (res.width(), res.height());
                    running.store(true, Ordering::SeqCst);
                    let _ = ready_tx.send(Ok(()));

                    while !stop_flag.load(Ordering::SeqCst) {
                        match camera.frame() {
                            Ok(frame) => {
                                if let Ok(decoded) = frame.decode_image::<RgbFormat>() {
                                    let (width, height) = (decoded.width(), decoded.height());
                                    let rgb_bytes = decoded.into_raw();
                                    let mut rgba_bytes =
                                        Vec::with_capacity((width * height * 4) as usize);
                                    for chunk in rgb_bytes.chunks(3) {
                                        rgba_bytes.extend_from_slice(&[
                                            chunk[0], chunk[1], chunk[2], 255,
                                        ]);
                                    }
                                    *frame_store.lock().unwrap() = Some(FrameData {
                                        data: rgba_bytes,
                                        width,
                                        height,
                                    });
                                }
                            }
                            Err(_) => thread::sleep(Duration::from_millis(10)),
                        }
                    }

                    let _ = camera.stop_stream();
                    running.store(false, Ordering::SeqCst);
                })
                .map_err(|e| format!("Failed to spawn webcam thread: {e}"))?;

            match ready_rx.recv() {
                Ok(Ok(())) => {
                    self.worker = Some(handle);
                    Ok(())
                }
                Ok(Err(err)) => {
                    let _ = handle.join();
                    Err(err)
                }
                Err(_) => {
                    let _ = handle.join();
                    Err("Webcam thread closed unexpectedly".to_string())
                }
            }
        }

        /// Stop the webcam
        pub fn stop(&mut self) {
            if !self.is_running() {
                return;
            }

            self.stop_flag.store(true, Ordering::SeqCst);
            if let Some(handle) = self.worker.take() {
                let _ = handle.join();
            }
            self.stop_flag.store(false, Ordering::SeqCst);
            *self.frame.lock().unwrap() = None;
        }

        /// Capture a frame (returns RGBA bytes)
        pub fn capture_frame(&self) -> Option<(Vec<u8>, u32, u32)> {
            self.frame
                .lock()
                .unwrap()
                .as_ref()
                .map(|frame| (frame.data.clone(), frame.width, frame.height))
        }

        /// Check if running
        pub fn is_running(&self) -> bool {
            self.running.load(Ordering::SeqCst)
        }

        /// Get resolution
        pub fn resolution(&self) -> (u32, u32) {
            *self.resolution.lock().unwrap()
        }

        /// List available cameras
        pub fn list_cameras() -> Vec<String> {
            match nokhwa::query(nokhwa::utils::ApiBackend::Auto) {
                Ok(cameras) => cameras.iter().map(|c| c.human_name().to_string()).collect(),
                Err(_) => Vec::new(),
            }
        }
    }

    impl Default for WebcamCapture {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Drop for WebcamCapture {
        fn drop(&mut self) {
            self.stop();
        }
    }
}

#[cfg(not(feature = "webcam"))]
pub mod backend {
    /// Stub webcam capture when feature is disabled
    pub struct WebcamCapture {
        is_running: bool,
    }

    impl WebcamCapture {
        pub fn new() -> Self {
            Self { is_running: false }
        }

        pub fn start(&mut self) -> Result<(), String> {
            Err("Webcam feature not enabled. Build with --features webcam".to_string())
        }

        pub fn stop(&mut self) {
            self.is_running = false;
        }

        pub fn capture_frame(&self) -> Option<(Vec<u8>, u32, u32)> {
            None
        }

        pub fn is_running(&self) -> bool {
            self.is_running
        }

        pub fn resolution(&self) -> (u32, u32) {
            (640, 480)
        }

        pub fn list_cameras() -> Vec<String> {
            Vec::new()
        }
    }

    impl Default for WebcamCapture {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub use backend::WebcamCapture;
