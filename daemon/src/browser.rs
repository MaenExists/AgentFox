use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use glib::MainLoop;
use javascriptcore::ValueExt;
use serde_json::Value;
use gtk::prelude::*;
use webkit2gtk::{LoadEvent, TLSErrorsPolicy, WebContext, WebContextExt, WebView, WebViewExt};

pub struct Browser {
    webview: WebView,
}

pub struct PageInfo {
    pub url: String,
    pub title: String,
}

impl Browser {
    pub fn new() -> Result<Self, String> {
        gtk::init().map_err(|err| format!("failed to initialize gtk: {err}"))?;
        let context = WebContext::default().ok_or_else(|| "failed to create web context".to_string())?;
        context.set_tls_errors_policy(TLSErrorsPolicy::Ignore);
        let webview = WebView::builder().web_context(&context).build();
        Ok(Self { webview })
    }

    pub fn open(&self, url: &str) -> Result<PageInfo, String> {
        let loop_ = MainLoop::new(None, false);
        let result: Rc<RefCell<Option<Result<(), String>>>> = Rc::new(RefCell::new(None));

        let completed = result.clone();
        let completed_loop = loop_.clone();
        let load_changed_id = self.webview.connect_load_changed(move |_, event| {
            if event == LoadEvent::Finished {
                completed.borrow_mut().replace(Ok(()));
                completed_loop.quit();
            }
        });

        let failed = result.clone();
        let failed_loop = loop_.clone();
        let load_failed_id = self.webview.connect_load_failed(move |_, _, uri, error| {
            failed
                .borrow_mut()
                .replace(Err(format!("failed to load: {uri} - {error}")));
            failed_loop.quit();
            false
        });

        let timed_out = result.clone();
        let timeout_loop = loop_.clone();
        let timeout_id = glib::timeout_add_local_once(Duration::from_secs(30), move || {
            if timed_out.borrow().is_none() {
                timed_out
                    .borrow_mut()
                    .replace(Err("page load timed out after 30 seconds".to_string()));
            }
            timeout_loop.quit();
        });

        self.webview.load_uri(url);
        loop_.run();

        self.webview.disconnect(load_changed_id);
        self.webview.disconnect(load_failed_id);
        timeout_id.remove();

        match result.borrow_mut().take() {
            Some(Ok(())) => Ok(PageInfo {
                url: self.webview.uri().map(|value| value.to_string()).unwrap_or_else(|| url.to_string()),
                title: self.evaluate_string("document.title")?.trim_matches('"').to_string(),
            }),
            Some(Err(error)) => Err(error),
            None => Err("page load ended without a result".to_string()),
        }
    }

    pub fn eval(&self, script: &str) -> Result<Value, String> {
        let loop_ = MainLoop::new(None, false);
        let result: Rc<RefCell<Option<Result<Value, String>>>> = Rc::new(RefCell::new(None));

        let completed = result.clone();
        let completed_loop = loop_.clone();
        self.webview.run_javascript(script, None::<&gtk::gio::Cancellable>, move |value| {
            let resolved = value
                .map_err(|error| error.to_string())
                .and_then(|js| {
                    let js_value = js
                        .js_value()
                        .ok_or_else(|| "javascript returned no value".to_string())?;
                    let json = js_value
                        .to_json(0)
                        .map(|raw| raw.to_string())
                        .unwrap_or_else(|| format!("{:?}", js_value.to_str()));
                    serde_json::from_str(&json)
                        .map_err(|error| format!("failed to parse javascript result: {error}"))
                });
            completed.borrow_mut().replace(resolved);
            completed_loop.quit();
        });

        let timed_out = result.clone();
        let timeout_loop = loop_.clone();
        let timeout_id = glib::timeout_add_local_once(Duration::from_secs(5), move || {
            if timed_out.borrow().is_none() {
                timed_out
                    .borrow_mut()
                    .replace(Err("javascript evaluation timed out".to_string()));
            }
            timeout_loop.quit();
        });

        loop_.run();
        timeout_id.remove();

        result
            .borrow_mut()
            .take()
            .unwrap_or_else(|| Err("javascript evaluation ended without a result".to_string()))
    }

    fn evaluate_string(&self, script: &str) -> Result<String, String> {
        match self.eval(script)? {
            Value::String(value) => Ok(value),
            other => Ok(other.to_string()),
        }
    }
}
