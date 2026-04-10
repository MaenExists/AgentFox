use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use glib::MainLoop;
use agentfox_protocol::SemanticNode;
use javascriptcore::ValueExt;
use serde::Deserialize;
use serde_json::Value;
use gtk::prelude::*;
use webkit2gtk::{LoadEvent, TLSErrorsPolicy, WebContext, WebContextExt, WebView, WebViewExt};

pub struct Browser {
    webview: WebView,
    selectors: RefCell<HashMap<String, String>>,
}

pub struct PageInfo {
    pub url: String,
    pub title: String,
}

pub struct Snapshot {
    pub url: String,
    pub title: String,
    pub elements: Vec<SemanticNode>,
}

#[derive(Debug, Deserialize)]
struct SnapshotPayload {
    url: String,
    title: String,
    elements: Vec<SnapshotElement>,
}

#[derive(Debug, Deserialize)]
struct SnapshotElement {
    id: String,
    role: String,
    text: Option<String>,
    href: Option<String>,
    value: Option<String>,
    selector: String,
}

impl Browser {
    pub fn new() -> Result<Self, String> {
        gtk::init().map_err(|err| format!("failed to initialize gtk: {err}"))?;
        let context = WebContext::default().ok_or_else(|| "failed to create web context".to_string())?;
        context.set_tls_errors_policy(TLSErrorsPolicy::Ignore);
        let webview = WebView::builder().web_context(&context).build();
        Ok(Self {
            webview,
            selectors: RefCell::new(HashMap::new()),
        })
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

    pub fn snap(&self) -> Result<Snapshot, String> {
        let value = self.eval(
            r#"
            (() => {
              if (!window.__afoxNextId) {
                window.__afoxNextId = 1;
              }

              const inferRole = (el) => {
                const tag = el.tagName.toLowerCase();
                if (tag === "a") return "link";
                if (tag === "button") return "button";
                if (tag === "input") {
                  const type = (el.getAttribute("type") || "text").toLowerCase();
                  return type === "submit" || type === "button" ? "button" : "input";
                }
                if (tag === "textarea") return "textbox";
                if (tag === "select") return "select";
                if (tag === "label") return "label";
                if (/^h[1-6]$/.test(tag)) return "heading";
                if (tag === "p") return "paragraph";
                return el.getAttribute("role") || tag;
              };

              const textFor = (el) => {
                const direct = (el.innerText || el.textContent || "").replace(/\s+/g, " ").trim();
                if (direct) return direct;
                if ("value" in el && typeof el.value === "string") {
                  return el.value.trim();
                }
                return "";
              };

              const cssPath = (el) => {
                const segments = [];
                let current = el;
                while (current && current.nodeType === Node.ELEMENT_NODE && current !== document.body) {
                  const tag = current.tagName.toLowerCase();
                  let index = 1;
                  let sibling = current.previousElementSibling;
                  while (sibling) {
                    if (sibling.tagName === current.tagName) index += 1;
                    sibling = sibling.previousElementSibling;
                  }
                  segments.unshift(`${tag}:nth-of-type(${index})`);
                  current = current.parentElement;
                }
                segments.unshift("body");
                return segments.join(" > ");
              };

              const candidates = Array.from(
                document.querySelectorAll("a, button, input, textarea, select, label, h1, h2, h3, h4, h5, h6, p, [role]")
              );

              const elements = [];
              for (const el of candidates) {
                if (!(el instanceof HTMLElement)) continue;
                const style = window.getComputedStyle(el);
                const rect = el.getBoundingClientRect();
                if (style.display === "none" || style.visibility === "hidden") continue;
                if (rect.width === 0 && rect.height === 0) continue;

                if (!el.dataset.afoxId) {
                  el.dataset.afoxId = `e${window.__afoxNextId++}`;
                }

                const text = textFor(el);
                const href = el instanceof HTMLAnchorElement ? el.href : null;
                const value = "value" in el && typeof el.value === "string" ? el.value : null;
                elements.push({
                  id: el.dataset.afoxId,
                  role: inferRole(el),
                  text: text || null,
                  href: href || null,
                  value: value || null,
                  selector: cssPath(el)
                });
              }

              return {
                url: window.location.href,
                title: document.title,
                elements
              };
            })()
            "#,
        )?;

        let payload: SnapshotPayload =
            serde_json::from_value(value).map_err(|error| format!("failed to decode snapshot: {error}"))?;
        let mut selectors = self.selectors.borrow_mut();
        selectors.clear();
        let elements = payload
            .elements
            .into_iter()
            .map(|element| {
                selectors.insert(element.id.clone(), element.selector);
                SemanticNode {
                    id: element.id,
                    role: element.role,
                    text: element.text,
                    href: element.href,
                    value: element.value,
                }
            })
            .collect();
        Ok(Snapshot {
            url: payload.url,
            title: payload.title,
            elements,
        })
    }

    pub fn text(&self, element_id: &str) -> Result<String, String> {
        let selector = self
            .selectors
            .borrow()
            .get(element_id)
            .cloned()
            .ok_or_else(|| format!("Element '{element_id}' not found"))?;
        let selector = serde_json::to_string(&selector)
            .map_err(|error| format!("failed to encode selector: {error}"))?;
        self.evaluate_string(&format!(
            r#"
            (() => {{
              const selector = {selector};
              const el = document.querySelector(selector);
              if (!el) {{
                throw new Error(`Element for selector '${{selector}}' not found`);
              }}
              if ("value" in el && typeof el.value === "string" && el.value.trim()) {{
                return el.value;
              }}
              return (el.innerText || el.textContent || "").trim();
            }})()
            "#
        ))
    }

    fn evaluate_string(&self, script: &str) -> Result<String, String> {
        match self.eval(script)? {
            Value::String(value) => Ok(value),
            other => Ok(other.to_string()),
        }
    }
}
