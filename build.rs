extern crate windows_exe_info;

fn main() {
    slint_build::compile("ui/main.slint").expect("Slint build failed");
    windows_exe_info::icon::icon_ico("ui/icons/logo.ico");
}