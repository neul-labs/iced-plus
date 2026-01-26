# WebView

Embedded web browser integration for iced-plus applications.

!!! warning "Experimental Feature"
    WebView integration is experimental. System browser launching is stable, but embedded webview requires the `wry` feature and additional setup.

## System Browser

The simplest approach - open URLs in the user's default browser:

```rust
use iced_plus_platform::webview::open_url;

// Opens in default browser
open_url("https://example.com");
```

This works reliably on all platforms.

## Embedded WebView

For embedding a browser within your application:

```rust
use iced_plus_platform::webview::{WebView, WebViewConfig};

let config = WebViewConfig::new()
    .url("https://example.com")
    .devtools(cfg!(debug_assertions));

let webview = WebView::new(config);
```

### Requirements

Enable the `webview` feature:

```toml
[dependencies]
iced_plus_platform = { version = "0.1", features = ["webview"] }
```

This pulls in:
- `wry` - WebView library
- `tao` - Window management

### Configuration

```rust
WebViewConfig::new()
    .url("https://example.com")      // Initial URL
    .html("<h1>Hello</h1>")          // Or load HTML directly
    .devtools(true)                  // Enable dev tools
    .transparent(false)              // Transparent background
    .user_agent("Custom UA")         // Custom user agent
```

## WebViewState

State management for browser functionality:

```rust
use iced_plus_components::{WebViewState, WebViewCommand, BrowserBar};

struct App {
    webview: WebViewState,
}

impl App {
    fn new() -> Self {
        Self {
            webview: WebViewState::new("https://example.com"),
        }
    }
}
```

### Browser Bar

Navigation controls:

```rust
BrowserBar::new(&self.webview)
    .on_back(Message::WebBack)
    .on_forward(Message::WebForward)
    .on_refresh(Message::WebRefresh)
    .on_url_submit(Message::WebNavigate)
```

### Commands

```rust
match message {
    Message::WebBack => {
        return self.webview.command(WebViewCommand::Back);
    }
    Message::WebForward => {
        return self.webview.command(WebViewCommand::Forward);
    }
    Message::WebRefresh => {
        return self.webview.command(WebViewCommand::Refresh);
    }
    Message::WebNavigate(url) => {
        return self.webview.command(WebViewCommand::Navigate(url));
    }
}
```

## JavaScript Communication

Execute JavaScript in the webview:

```rust
// Execute script
webview.eval("document.body.style.background = 'red'");

// With callback
webview.eval_with_callback(
    "document.title",
    |result| Message::TitleReceived(result)
);
```

## Platform Notes

### macOS

- Uses WKWebView
- Full-featured, modern rendering

### Windows

- Uses WebView2 (Edge/Chromium)
- Requires WebView2 runtime (bundled with Windows 11, installable on Windows 10)

### Linux

- Uses WebKitGTK
- Requires `libwebkit2gtk-4.0-dev` package

## Example

See the kitchen sink example for a working implementation:

```bash
cargo run -p kitchen_sink --features webview
```
