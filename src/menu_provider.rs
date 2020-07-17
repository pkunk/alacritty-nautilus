use glib::filename_from_uri;
use glib_sys::gpointer;
use gobject_sys::GObject;
use gtk_sys::GtkWidget;
use nautilus_extension::nautilus_menu_background_activate_cb;
use nautilus_extension::{FileInfo, MenuItem, MenuProvider};
use std::path::PathBuf;
use std::process::Command;

pub struct AMenuProvider {}

impl MenuProvider for AMenuProvider {
    fn get_file_items(&self, _window: *mut GtkWidget, _files: &Vec<FileInfo>) -> Vec<MenuItem> {
        Vec::new()
    }

    fn get_background_items(
        &self,
        _window: *mut GtkWidget,
        current_folder: &FileInfo,
    ) -> Vec<MenuItem> {
        if current_folder.get_uri_scheme() != "file" {
            return Vec::new();
        }

        vec![terminal_nautilus_menu_item_new()]
    }
}

fn terminal_nautilus_menu_item_new() -> MenuItem {
    let action_name = "AlacrittyNautilusExtension:OpenFolderLocal";
    let name = "Open in Terminal";
    let tooltip = "Open the currently selected folder in a terminal";

    let mut terminal_menu_item = MenuItem::new(action_name, name, tooltip, None);

    terminal_menu_item.set_activate_cb(terminal_activate_cb);

    terminal_menu_item
}

nautilus_menu_background_activate_cb!(terminal_activate_cb, terminal_nautilus_menu_item_activate);

fn terminal_nautilus_menu_item_activate(file: FileInfo) {
    if file.get_uri_scheme() != "file" {
        return;
    }

    let path = filename_from_uri(&file.get_uri());
    if path.is_err() {
        return;
    }
    let path = path.unwrap();

    create_terminal(path.0);
}

fn create_terminal(path: PathBuf) {
    if let Err(e) = Command::new("alacritty").current_dir(path).spawn() {
        eprintln!("Failed to start alacritty: {}", e);
    };
}
