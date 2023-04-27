use std::{error::Error, process};
use tauri::{api, AppHandle, CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

pub struct Tray {}

impl Tray {
    pub fn tray_menu() -> SystemTrayMenu {
        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("open", "打开面板"))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("quit", "退出").accelerator("CmdOrControl+Q"))
    }

    pub fn update_system_tray(app_handle: &AppHandle) -> Result<(), Box<(dyn Error + 'static)>> {
        app_handle.tray_handle().set_menu(Tray::tray_menu())?;
        Ok(())
    }

    pub fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
        match event {
            SystemTrayEvent::MenuItemClick { tray_id, id, .. } => match id.as_str() {
                "open" => {}
                "quit" => {
                    api::process::kill_children();
                    app_handle.exit(0);
                    process::exit(0);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
