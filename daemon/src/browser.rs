use gtk::prelude::*;
use webkit2gtk::WebView;

pub struct Browser {
    webview: WebView,
}

impl Browser {
    pub fn new() -> Result<Self, String> {
        gtk::init().map_err(|err| format!("failed to initialize gtk: {err}"))?;
        let webview = WebView::new();
        Ok(Self { webview })
    }

    pub fn webview(&self) -> &WebView {
        &self.webview
    }
}
