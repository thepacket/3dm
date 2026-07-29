//! Getting evidence out of the web build.
//!
//! A failure in a WebGPU app is close to invisible: the canvas stays the colour
//! of the page behind it and the console says nothing, because a device error is
//! delivered to a callback nobody installed and a panic in a spawned future is
//! swallowed. The black page in production looked exactly like a healthy start.
//!
//! So the app narrates itself into the DOM instead of into the canvas. Two
//! elements:
//!
//! * `#3dm_diag` — a `<pre>` positioned just past the bottom of the viewport.
//!   Present in the document and readable as text, but never in the user's way.
//!   Adapter, limits and frame progress land here.
//! * `#3dm_error` — a full-window overlay, created only when something actually
//!   goes wrong, so a failure reads as a message rather than as a black screen.
//!
//! Text, not pixels, is the point: the canvas cannot be inspected from a
//! headless or backgrounded browser, and the DOM can.

/// Record a diagnostic line. Cheap enough to call at startup, not per frame.
pub fn note(line: &str) {
    imp::note(line);
}

/// Surface a failure to the user, and to anything reading the page.
pub fn error(headline: &str, detail: &str) {
    imp::error(headline, detail);
}

/// Chain a DOM-reporting panic hook onto whatever is already installed.
pub fn install_panic_hook() {
    imp::install_panic_hook();
}

/// Report the outcome of a wgpu error scope once the device has resolved it.
///
/// The reason this exists rather than a plain check: a WGSL module the browser
/// rejects does not fail loudly. Pipeline creation returns, the app starts, and
/// every subsequent frame draws nothing — a black page with a clean console,
/// which is exactly the production symptom. An error scope is also the only
/// probe here that does not need a painted frame, and frames are precisely what
/// a backgrounded browser will not give us.
pub fn watch_error_scope(
    what: &'static str,
    scope: impl std::future::Future<Output = Option<eframe::wgpu::Error>> + 'static,
) {
    imp::watch_error_scope(what, scope);
}

/// Everything known about the GPU we were given, as diagnostic lines.
///
/// Written at startup so a device that is merely *different* from the one the
/// app was developed against is visible in the report, not inferred later.
pub fn report_device(adapter: &eframe::wgpu::Adapter, device: &eframe::wgpu::Device) {
    let info = adapter.get_info();
    note(&format!(
        "adapter: {} | backend {:?} | type {:?}",
        info.name, info.backend, info.device_type
    ));
    if !info.driver.is_empty() || !info.driver_info.is_empty() {
        note(&format!("driver: {} {}", info.driver, info.driver_info));
    }

    let l = device.limits();
    note(&format!(
        "limits: max_texture_dimension_2d={} max_buffer_size={} \
         max_storage_buffer_binding_size={} max_uniform_buffer_binding_size={} \
         max_bind_groups={}",
        l.max_texture_dimension_2d,
        l.max_buffer_size,
        l.max_storage_buffer_binding_size,
        l.max_uniform_buffer_binding_size,
        l.max_bind_groups,
    ));
}

#[cfg(target_arch = "wasm32")]
mod imp {
    const DIAG_ID: &str = "3dm_diag";
    const ERROR_ID: &str = "3dm_error";

    fn document() -> Option<web_sys::Document> {
        web_sys::window()?.document()
    }

    /// The offscreen `<pre>`, created on first use so index.html need not carry
    /// an element that is only meaningful once the wasm is running.
    fn diag_element() -> Option<web_sys::Element> {
        let doc = document()?;
        if let Some(existing) = doc.get_element_by_id(DIAG_ID) {
            return Some(existing);
        }
        let el = doc.create_element("pre").ok()?;
        el.set_id(DIAG_ID);
        doc.body()?.append_child(&el).ok()?;
        Some(el)
    }

    pub fn note(line: &str) {
        log::info!("{line}");
        if let Some(el) = diag_element() {
            let mut text = el.text_content().unwrap_or_default();
            text.push_str(line);
            text.push('\n');
            el.set_text_content(Some(&text));
        }
    }

    pub fn error(headline: &str, detail: &str) {
        log::error!("{headline} {detail}");
        note(&format!("ERROR: {headline} {detail}"));

        let Some(doc) = document() else { return };
        let el = match doc.get_element_by_id(ERROR_ID) {
            Some(existing) => existing,
            None => {
                let Some(created) = doc.create_element("div").ok() else {
                    return;
                };
                created.set_id(ERROR_ID);
                let Some(body) = doc.body() else { return };
                if body.append_child(&created).is_err() {
                    return;
                }
                created
            }
        };

        // Text content, not HTML: the detail is an error string from wgpu or a
        // panic payload, and neither is safe to interpolate as markup.
        let mut text = el.text_content().unwrap_or_default();
        if !text.is_empty() {
            text.push_str("\n\n");
        }
        text.push_str(headline);
        text.push('\n');
        text.push_str(detail);
        el.set_text_content(Some(&text));
    }

    pub fn watch_error_scope(
        what: &'static str,
        scope: impl std::future::Future<Output = Option<eframe::wgpu::Error>> + 'static,
    ) {
        // Promises still resolve in a backgrounded tab even though animation
        // frames do not, so this reports where a frame-driven check could not.
        wasm_bindgen_futures::spawn_local(async move {
            match scope.await {
                Some(e) => super::error(&format!("The GPU rejected {what}."), &e.to_string()),
                None => super::note(&format!("{what}: validated clean")),
            }
        });
    }

    pub fn install_panic_hook() {
        console_error_panic_hook::set_once();
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            error("3DM panicked.", &info.to_string());
            previous(info);
        }));
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    pub fn note(line: &str) {
        log::info!("{line}");
    }

    pub fn error(headline: &str, detail: &str) {
        log::error!("{headline} {detail}");
    }

    /// Native builds already print a backtrace to the terminal.
    pub fn install_panic_hook() {}

    /// Native has no executor to hand this to, and a native run shows its
    /// errors on stderr anyway. Dropping the future cancels the scope cleanly.
    pub fn watch_error_scope(
        _what: &'static str,
        _scope: impl std::future::Future<Output = Option<eframe::wgpu::Error>> + 'static,
    ) {
    }
}
