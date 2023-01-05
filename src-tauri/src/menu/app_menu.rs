/*
 * @Author: qyzzs qyzzzs@163.com
 * @Date: 2023-01-04 17:17:40
 * @LastEditors: qyzzs qyzzzs@163.com
 * @LastEditTime: 2023-01-05 17:38:43
 * @FilePath: \tauri-app\src-tauri\src\menu\app_menu.rs
 * @Description: 应用菜单
 */
use tauri::utils::assets::EmbeddedAssets;
use tauri::{ AboutMetadata, Context, CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent };

// 应用菜单项
pub fn init(context: &Context<EmbeddedAssets>) -> Menu {
    // 应用名称
    let name = &context.package_info().name;
    // tauri::Menu::os_default(name)
    // 应用主菜单
    let app_menu = Submenu::new(
        "",
        // MenuItem::About 为原生菜单
        Menu::new().add_native_item(MenuItem::About(name.into(), AboutMetadata::new()))
    );
    // 文件菜单（自定义菜单）
    let file_menu = Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("new_file".to_string(), "New File"))
            .add_item(CustomMenuItem::new("edit_file".to_string(), "Edit File"))
    );
    // 编辑菜单（自定义菜单）
    let edit_menu = Submenu::new(
        "Edit",
        Menu::new()
            .add_item(CustomMenuItem::new("undo".to_string(), "Undo"))
            .add_item(CustomMenuItem::new("redo".to_string(), "Redo"))
    );

    Menu::new().add_submenu(app_menu).add_submenu(file_menu).add_submenu(edit_menu)
}

// 应用菜单处理事件
pub fn handler(event: WindowMenuEvent) {
    // 菜单所属的窗口
    let _win = Some(event.window());
    // 匹配菜单 id
    match event.menu_item_id() {
        "new_file" => {
            // debug 信息（终端输出）
            dbg!("new file");
        }
        "edit_file" => {
            // 发送信息到菜单所属窗口（弹窗形式）
            
        }
        "undo" => {
            dbg!("undo");
        }
        "redo" => {
            dbg!("redo");
        }
        _ => {}
    }
}