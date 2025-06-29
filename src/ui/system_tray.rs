use tauri::{SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

pub fn create_tray() -> SystemTray {
  // let tray_menu = SystemTrayMenu::new()
  //   .add_item(CustomMenuItem::new("show".to_string(), "Show"))
  //   .add_native_item(SystemTrayMenuItem::Separator)
  //   .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

  let tray = SystemTray::new(); // 移除托盘菜单
  tray
}