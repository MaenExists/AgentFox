use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use std::fs;

use glib::MainLoop;
use agentfox_protocol::{get_config_path, Config, SemanticNode};
use javascriptcore::ValueExt;
use serde::{Deserialize, Serialize};
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

pub struct Snapshot {
    pub url: String,
    pub title: String,
    pub elements: Vec<SemanticNode>,
}

impl Browser {
    pub fn new() -> Result<Self, String> {
        gtk::init().map_err(|err| format!("failed to initialize gtk: {err}"))?;
        let context = WebContext::default().ok_or_else(|| "failed to create web context".to_string())?;
        context.set_tls_errors_policy(TLSErrorsPolicy::Ignore);
        let webview = WebView::builder().web_context(&context).build();
        
        // Standard high-res viewport for agents to ensure elements are visible and have dimensions
        webview.set_size_request(1280, 1024);
        
        Ok(Self {
            webview,
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
        if let Some(source) = glib::MainContext::default().find_source_by_id(&timeout_id) {
            source.destroy();
        }

        match result.borrow_mut().take() {
            Some(Ok(())) => self.page_info(),
            Some(Err(error)) => Err(error),
            None => Err("page load ended without a result".to_string()),
        }
    }

    pub fn search(&self, query: &str) -> Result<PageInfo, String> {
        let target = resolve_search_target(query);
        self.open(&target)
    }

    pub fn click(&self, element_id: &str) -> Result<PageInfo, String> {
        let element_id = serde_json::to_string(element_id)
            .map_err(|error| format!("failed to encode element_id: {error}"))?;

        self.eval(&format!(
            r#"
            (() => {{
              const id = {element_id};
              const el = document.querySelector(`[data-afox-id="${{id}}"]`);
              if (!el) {{
                throw new Error(`Element with id '${{id}}' not found`);
              }}
              const eventInit = {{ bubbles: true, cancelable: true, composed: true, view: window }};
              const down = window.PointerEvent || window.MouseEvent;
              const up = window.PointerEvent || window.MouseEvent;
              el.dispatchEvent(new down("pointerdown", eventInit));
              el.dispatchEvent(new MouseEvent("mousedown", eventInit));
              if (typeof el.focus === "function") el.focus();
              el.dispatchEvent(new MouseEvent("mouseup", eventInit));
              el.dispatchEvent(new MouseEvent("click", eventInit));
              return true;
            }})()
            "#
        ))?;

        self.pump_for(Duration::from_millis(300));
        if self.webview.is_loading() {
            self.wait_for_finish(Duration::from_secs(10))?;
        }
        self.page_info()
    }

    pub fn fill(&self, element_id: &str, text: &str) -> Result<(), String> {
        let element_id = serde_json::to_string(element_id)
            .map_err(|error| format!("failed to encode element_id: {error}"))?;
        let text =
            serde_json::to_string(text).map_err(|error| format!("failed to encode fill text: {error}"))?;

        self.eval(&format!(
            r#"
            (() => {{
              const id = {element_id};
              const value = {text};
              const el = document.querySelector(`[data-afox-id="${{id}}"]`);
              if (!el) {{
                throw new Error(`Element with id '${{id}}' not found`);
              }}
              if (!("value" in el)) {{
                throw new Error("Target element is not fillable");
              }}
              if (typeof el.focus === "function") el.focus();
              el.value = value;
              el.dispatchEvent(new InputEvent("input", {{ bubbles: true, cancelable: true, data: value, inputType: "insertText" }}));
              el.dispatchEvent(new Event("change", {{ bubbles: true, cancelable: true }}));
              return true;
            }})()
            "#
        ))?;
        Ok(())
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
        if let Some(source) = glib::MainContext::default().find_source_by_id(&timeout_id) {
            source.destroy();
        }

        result
            .borrow_mut()
            .take()
            .unwrap_or_else(|| Err("javascript evaluation ended without a result".to_string()))
    }

    pub fn snap(&self) -> Result<Snapshot, String> {
        // Increase settling time for heavy dynamic sites like Google Search
        self.pump_for(Duration::from_millis(1500));
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
                const role = el.getAttribute("role");
                if (role) return role;
                if (el.hasAttribute("onclick") || window.getComputedStyle(el).cursor === "pointer") return "button";
                return tag;
              };

              const textFor = (el) => {
                if (el.tagName === "INPUT" || el.tagName === "TEXTAREA" || el.tagName === "SELECT") {
                   return el.value ? el.value.trim() : "";
                }
                return (el.innerText || el.textContent || "").replace(/\s+/g, " ").trim();
              };

              const elements = [];
              const processNode = (node) => {
                if (node.nodeType !== Node.ELEMENT_NODE) return;
                const el = node;
                const tag = el.tagName.toLowerCase();
                
                if (["script", "style", "noscript", "template", "meta", "link"].includes(tag)) return;

                const style = window.getComputedStyle(el);
                if (style.display === "none") return; // Keep visibility:hidden as it might be a container

                const text = textFor(el);
                const isInteractive = [
                    "a", "button", "input", "textarea", "select", "label"
                  ].includes(tag) || 
                  el.hasAttribute("role") || 
                  el.hasAttribute("onclick") ||
                  el.hasAttribute("jsaction") ||
                  style.cursor === "pointer";
                
                const isSemantic = [
                    "h1", "h2", "h3", "h4", "h5", "h6", "p", "article"
                  ].includes(tag);

                if (isInteractive || isSemantic || (text.length > 0 && !["div", "span"].includes(tag))) {
                  if (!el.dataset.afoxId) {
                    el.dataset.afoxId = `e${window.__afoxNextId++}`;
                  }
                  elements.push({
                    i: el.dataset.afoxId,
                    r: inferRole(el),
                    t: text || null,
                    h: el instanceof HTMLAnchorElement ? el.href : null,
                    v: "value" in el ? el.value : null
                  });
                }

                if (el.shadowRoot) {
                  Array.from(el.shadowRoot.childNodes).forEach(processNode);
                }
                Array.from(el.childNodes).forEach(processNode);
              };

              processNode(document.body);

              return {
                u: window.location.href,
                t: document.title,
                e: elements
              };
            })()
            "#,
        )?;

        #[derive(Debug, Deserialize)]
        struct CompactSnapshot {
            u: String,
            t: String,
            e: Vec<CompactElement>,
        }
        #[derive(Debug, Deserialize)]
        struct CompactElement {
            i: String,
            r: String,
            t: Option<String>,
            h: Option<String>,
            v: Option<String>,
        }

        let compact: CompactSnapshot =
            serde_json::from_value(value).map_err(|error| format!("failed to decode compact snapshot: {error}"))?;
        
        let elements = compact
            .e
            .into_iter()
            .map(|element| {
                SemanticNode {
                    id: element.i,
                    role: element.r,
                    text: element.t,
                    href: element.h,
                    value: element.v,
                }
            })
            .collect();

        Ok(Snapshot {
            url: compact.u,
            title: compact.t,
            elements,
        })
    }

    pub fn view(&self) -> Result<String, String> {
        let snapshot = self.snap()?;
        let mut md = format!("# {}\nURL: {}\n\n", snapshot.title, snapshot.url);
        
        let noise_patterns = [
            "skip to", "accessibility", "feedback", "sign in", "privacy", "terms", "cookie",
            "log in", "sign up", "help", "about", "contact", "support"
        ];

        let mut seen_urls = std::collections::HashSet::new();

        for el in snapshot.elements {
            let role = el.role.as_str();
            // Prune layout containers and overly generic roles
            if ["html", "body", "div", "span", "section", "article", "main", "header", "footer", "nav", "paragraph"].contains(&role) {
                continue;
            }

            let text = el.text.as_deref().unwrap_or("").trim();
            let href = el.href.as_deref().unwrap_or("").trim();
            let value = el.value.as_deref().unwrap_or("").trim();

            // Prune empty interactive elements
            if text.is_empty() && href.is_empty() && value.is_empty() {
                continue;
            }

            // Prune utility noise
            let lower_text = text.to_lowercase();
            if noise_patterns.iter().any(|p| lower_text.contains(p)) {
                continue;
            }

            // Prune duplicate links (often seen in site logos/navs)
            if role == "link" && !href.is_empty() && text.is_empty() {
                if seen_urls.contains(href) {
                    continue;
                }
                seen_urls.insert(href.to_string());
            }

            match role {
                "heading" => {
                    md.push_str(&format!("## [{}] {}\n", el.id, text));
                }
                "link" => {
                    if !text.is_empty() {
                        md.push_str(&format!("- [{}] (link) [{}]({})\n", el.id, text, href));
                    } else {
                        md.push_str(&format!("- [{}] (link) {}\n", el.id, href));
                    }
                }
                "button" => {
                    let label = if !text.is_empty() { text } else { "Button" };
                    md.push_str(&format!("- [{}] <button> {} </button>\n", el.id, label));
                }
                "input" | "textbox" => {
                    let label = if !text.is_empty() { format!("{}: ", text) } else { "".to_string() };
                    md.push_str(&format!("- [{}] ({}) {}[ {} ]\n", el.id, role, label, if value.is_empty() { "..." } else { value }));
                }
                _ => {
                    if !text.is_empty() {
                        md.push_str(&format!("- [{}] ({}) {}\n", el.id, role, text));
                    }
                }
            }
        }
        
        if md.lines().count() <= 3 {
            md.push_str("_No interactive elements discovered._\n");
        }

        Ok(md)
    }

    pub fn text(&self, element_id: &str) -> Result<String, String> {
        let element_id = serde_json::to_string(element_id)
            .map_err(|error| format!("failed to encode element_id: {error}"))?;
        self.evaluate_string(&format!(
            r#"
            (() => {{
              const id = {element_id};
              const el = document.querySelector(`[data-afox-id="${{id}}"]`);
              if (!el) {{
                throw new Error(`Element with id '${{id}}' not found`);
              }}
              if ("value" in el && typeof el.value === "string" && el.value.trim()) {{
                return el.value;
              }}
              return (el.innerText || el.textContent || "").trim();
            }})()
            "#
        ))
    }

    pub fn summarize(&self) -> Result<String, String> {
        let config_path = get_config_path();
        if !config_path.exists() {
            return Err("AgentFox is not authenticated. Run 'afox auth <key>' first.".to_string());
        }
        let config_json = fs::read_to_string(config_path).map_err(|err| format!("failed to read config: {err}"))?;
        let config: Config = serde_json::from_str(&config_json).map_err(|err| format!("failed to parse config: {err}"))?;

        if config.api_key.is_empty() {
            return Err("API key is empty. Run 'afox auth <key>' to set it.".to_string());
        }

        let main_content = self.evaluate_string(
            r#"
            (() => {
              const selectors = [
                'article', 'main', '[role="main"]', '.main-content', '#content', '.content', '.article'
              ];
              for (const s of selectors) {
                const el = document.querySelector(s);
                if (el && el.innerText.length > 500) return el.innerText;
              }
              return document.body.innerText;
            })()
            "#,
        )?;

        let truncated_content = if main_content.len() > 10000 {
            format!("{}...", &main_content[..10000])
        } else {
            main_content
        };

        let client = reqwest::blocking::Client::new();
        
        #[derive(Serialize)]
        struct Message {
            role: String,
            content: String,
        }
        #[derive(Serialize)]
        struct ChatRequest {
            model: String,
            messages: Vec<Message>,
            max_tokens: u32,
        }

        let chat_request = ChatRequest {
            model: config.model,
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are a concise summarizer for an AI agent. Summarize the following web page content into 2-3 short, highly informative paragraphs. Focus on facts, data, and key actions available. Do not use filler words.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: truncated_content,
                },
            ],
            max_tokens: 500,
        };

        let res = client
            .post(format!("{}/chat/completions", config.api_url))
            .header("Authorization", format!("Bearer {}", config.api_key))
            .json(&chat_request)
            .send()
            .map_err(|err| format!("LLM API request failed: {err}"))?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().unwrap_or_default();
            return Err(format!("LLM API returned error ({}): {}", status, error_text));
        }

        let json: Value = res.json().map_err(|err| format!("failed to parse LLM response: {err}"))?;
        let summary = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| "LLM response did not contain content".to_string())?;

        Ok(summary.to_string())
    }

    fn page_info(&self) -> Result<PageInfo, String> {
        Ok(PageInfo {
            url: self
                .webview
                .uri()
                .map(|value| value.to_string())
                .unwrap_or_default(),
            title: self.evaluate_string("document.title")?.trim_matches('"').to_string(),
        })
    }

    fn pump_for(&self, duration: Duration) {
        let loop_ = MainLoop::new(None, false);
        let timeout_loop = loop_.clone();
        glib::timeout_add_local_once(duration, move || timeout_loop.quit());
        loop_.run();
    }

    fn wait_for_finish(&self, duration: Duration) -> Result<(), String> {
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
        let timeout_id = glib::timeout_add_local_once(duration, move || {
            if timed_out.borrow().is_none() {
                timed_out
                    .borrow_mut()
                    .replace(Err("page load timed out".to_string()));
            }
            timeout_loop.quit();
        });

        loop_.run();

        self.webview.disconnect(load_changed_id);
        self.webview.disconnect(load_failed_id);
        if let Some(source) = glib::MainContext::default().find_source_by_id(&timeout_id) {
            source.destroy();
        }

        match result.borrow_mut().take() {
            Some(result) => result,
            None => Ok(()),
        }
    }

    fn evaluate_string(&self, script: &str) -> Result<String, String> {
        match self.eval(script)? {
            Value::String(value) => Ok(value),
            other => Ok(other.to_string()),
        }
    }
}

fn resolve_search_target(query: &str) -> String {
    let trimmed = query.trim();
    if trimmed.starts_with("http://")
        || trimmed.starts_with("https://")
        || trimmed.starts_with("data:")
        || trimmed.starts_with("file:")
    {
        return trimmed.to_string();
    }

    let looks_like_host = !trimmed.contains(' ')
        && trimmed.contains('.')
        && !trimmed.contains("://");
    if looks_like_host {
        return format!("https://{trimmed}");
    }

    let encoded: String = url::form_urlencoded::byte_serialize(trimmed.as_bytes()).collect();
    format!("https://www.google.com/search?q={encoded}")
}
