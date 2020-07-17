mod menu_provider;

use glib_sys::GType;
use gobject_sys::GTypeModule;
use libc::c_int;
use nautilus_extension::nautilus_module;
use nautilus_extension::NautilusModule;

nautilus_module!(init);

fn init(module: *mut GTypeModule) -> GType {
    println!(
        "Initializing Alacritty Nautilus {}",
        env!("CARGO_PKG_VERSION")
    );

    NautilusModule::new(module, "AlacrittyNautilusExtension")
        .add_menu_provider(menu_provider::AMenuProvider {})
        .register()
}
