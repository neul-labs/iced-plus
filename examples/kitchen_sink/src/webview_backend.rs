//! WebView backend - opens URLs in system browser or embedded webview.
//!
//! Uses a `wry` window when the `webview` feature is enabled and falls back to
//! launching the system browser otherwise.

#[cfg(feature = "webview")]
pub mod backend {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    };
    use std::thread::{self, JoinHandle};

    use tao::dpi::LogicalSize;
    use tao::event::{Event, WindowEvent};
    use tao::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy};
    use tao::window::WindowBuilder;
    use wry::WebViewBuilder;

    enum WebViewEvent {
        Navigate(String),
        Close,
    }

    /// WebView window state
    pub struct WebViewWindow {
        url: String,
        is_open: Arc<AtomicBool>,
        proxy: Option<EventLoopProxy<WebViewEvent>>,
        thread_handle: Option<JoinHandle<()>>,
    }

    impl WebViewWindow {
        /// Create a new webview window controller.
        pub fn new() -> Self {
            Self {
                url: "https://example.com".to_string(),
                is_open: Arc::new(AtomicBool::new(false)),
                proxy: None,
                thread_handle: None,
            }
        }

        fn cleanup_closed_window(&mut self) {
            if let Some(handle) = self.thread_handle.as_ref() {
                if handle.is_finished() || !self.is_open() {
                    if let Some(handle) = self.thread_handle.take() {
                        let _ = handle.join();
                    }
                    self.proxy = None;
                    self.is_open.store(false, Ordering::SeqCst);
                }
            }
        }

        fn spawn_thread(&mut self, url: String) -> Result<(), String> {
            let is_open = Arc::clone(&self.is_open);
            let (proxy_tx, proxy_rx) = mpsc::channel();

            let handle = thread::Builder::new()
                .name("iced-plus-webview".to_string())
                .spawn(move || {
                    let event_loop = EventLoopBuilder::<WebViewEvent>::with_user_event().build();

                    let window = match WindowBuilder::new()
                        .with_title("iced-plus WebView")
                        .with_inner_size(LogicalSize::new(1024.0, 768.0))
                        .build(&event_loop)
                    {
                        Ok(window) => window,
                        Err(err) => {
                            let _ = proxy_tx
                                .send(Err(format!("Failed to create webview window: {err}")));
                            return;
                        }
                    };

                    let builder = WebViewBuilder::new(&window).with_url(&url);

                    let webview = match builder.build() {
                        Ok(webview) => webview,
                        Err(err) => {
                            let _ = proxy_tx.send(Err(format!("Failed to build webview: {err}")));
                            return;
                        }
                    };

                    let proxy = event_loop.create_proxy();
                    if proxy_tx.send(Ok(proxy.clone())).is_err() {
                        return;
                    }

                    is_open.store(true, Ordering::SeqCst);

                    event_loop.run(move |event, _, control_flow| {
                        let _ = &window;
                        *control_flow = ControlFlow::Wait;
                        match event {
                            Event::WindowEvent {
                                event: WindowEvent::CloseRequested,
                                ..
                            } => {
                                *control_flow = ControlFlow::Exit;
                            }
                            Event::UserEvent(WebViewEvent::Navigate(url)) => {
                                let _ = webview.load_url(&url);
                            }
                            Event::UserEvent(WebViewEvent::Close) => {
                                *control_flow = ControlFlow::Exit;
                            }
                            Event::LoopDestroyed => {
                                is_open.store(false, Ordering::SeqCst);
                            }
                            _ => {}
                        }
                    });
                })
                .map_err(|e| format!("Failed to spawn webview thread: {e}"))?;

            match proxy_rx.recv() {
                Ok(Ok(proxy)) => {
                    self.proxy = Some(proxy);
                    self.thread_handle = Some(handle);
                    Ok(())
                }
                Ok(Err(err)) => {
                    let _ = handle.join();
                    Err(err)
                }
                Err(_) => Err("WebView thread closed unexpectedly".to_string()),
            }
        }

        /// Open the URL in a wry-backed window (spawns if needed).
        pub fn open(&mut self, url: &str) -> Result<(), String> {
            self.cleanup_closed_window();
            self.url = url.to_string();

            if let Some(proxy) = &self.proxy {
                proxy
                    .send_event(WebViewEvent::Navigate(self.url.clone()))
                    .map_err(|_| "WebView window is not available".to_string())?;
                return Ok(());
            }

            self.spawn_thread(self.url.clone())
        }

        /// Explicitly close the webview window if running.
        pub fn close(&mut self) {
            if let Some(proxy) = &self.proxy {
                let _ = proxy.send_event(WebViewEvent::Close);
            }
            if let Some(handle) = self.thread_handle.take() {
                let _ = handle.join();
            }
            self.proxy = None;
            self.is_open.store(false, Ordering::SeqCst);
        }

        /// Get current URL.
        pub fn url(&self) -> &str {
            &self.url
        }

        /// Set URL.
        pub fn set_url(&mut self, url: &str) {
            self.url = url.to_string();
        }

        /// Check if open.
        pub fn is_open(&self) -> bool {
            self.is_open.load(Ordering::SeqCst)
        }
    }

    impl Default for WebViewWindow {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Drop for WebViewWindow {
        fn drop(&mut self) {
            self.close();
        }
    }
}

#[cfg(not(feature = "webview"))]
pub mod backend {
    use std::process::Command;

    /// Stub webview when feature is disabled
    pub struct WebViewWindow {
        url: String,
        is_open: bool,
    }

    impl WebViewWindow {
        pub fn new() -> Self {
            Self {
                url: "https://example.com".to_string(),
                is_open: false,
            }
        }

        pub fn open(&mut self, url: &str) -> Result<(), String> {
            self.url = url.to_string();

            #[cfg(target_os = "linux")]
            {
                Command::new("xdg-open")
                    .arg(url)
                    .spawn()
                    .map_err(|e| format!("Failed to open browser: {}", e))?;
            }

            #[cfg(target_os = "macos")]
            {
                Command::new("open")
                    .arg(url)
                    .spawn()
                    .map_err(|e| format!("Failed to open browser: {}", e))?;
            }

            #[cfg(target_os = "windows")]
            {
                Command::new("cmd")
                    .args(["/C", "start", "", url])
                    .spawn()
                    .map_err(|e| format!("Failed to open browser: {}", e))?;
            }

            self.is_open = true;
            Ok(())
        }

        pub fn url(&self) -> &str {
            &self.url
        }

        pub fn set_url(&mut self, url: &str) {
            self.url = url.to_string();
        }

        pub fn is_open(&self) -> bool {
            self.is_open
        }
    }

    impl Default for WebViewWindow {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub use backend::WebViewWindow;
