//! Entry points.
//!
//! The web build is the primary target; the native build exists so the renderer
//! can be iterated on without a browser in the loop.

/// Shared between both targets: hand eframe a [`three_dm::App`], or a clear
/// error if wgpu could not be brought up.
fn build_app(
    cc: &eframe::CreationContext<'_>,
) -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
    three_dm::App::new(cc)
        .map(|app| Box::new(app) as Box<dyn eframe::App>)
        .ok_or_else(|| "3DM needs the wgpu backend, but eframe started without it".into())
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([640.0, 400.0])
            .with_title("3DM"),
        ..Default::default()
    };

    eframe::run_native("3DM", options, Box::new(build_app))
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    three_dm::diag::install_panic_hook();
    eframe::WebLogger::init(log::LevelFilter::Info).ok();
    three_dm::diag::note(&format!(
        "3DM starting | user agent: {}",
        web_sys::window()
            .and_then(|w| w.navigator().user_agent().ok())
            .unwrap_or_else(|| "unknown".into())
    ));

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document");

        let canvas = document
            .get_element_by_id("3dm_canvas")
            .expect("index.html is missing #3dm_canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("#3dm_canvas is not a <canvas>");

        let result = eframe::WebRunner::new()
            .start(canvas, eframe::WebOptions::default(), Box::new(build_app))
            .await;

        // Replace the loading text with whatever happened.
        if let Some(loading) = document.get_element_by_id("3dm_loading") {
            match &result {
                Ok(()) => loading.remove(),
                Err(e) => loading.set_inner_html(&format!(
                    "<p class='err'>3DM failed to start.</p>\
                     <p class='err-detail'>{e:?}</p>\
                     <p class='err-detail'>This build needs WebGPU — try the latest \
                     Chrome, Edge, or Safari 26+.</p>"
                )),
            }
        }

        match &result {
            Ok(()) => three_dm::diag::note("eframe start: ok"),
            Err(e) => three_dm::diag::error("3DM failed to start.", &format!("{e:?}")),
        }
    });
}
