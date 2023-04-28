use tauri::{WindowBuilder, WindowUrl, AppHandle, Manager};

pub fn create_window(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_window("main") {
        window.unminimize();
        window.show();
        window.set_focus();
        return;
    }

    let window = WindowBuilder::new(
        app_handle,
        "main".to_string(),
        WindowUrl::App("index.html".into()),
    )
    .title("Hello World")
    .center()
    .fullscreen(false);
}
