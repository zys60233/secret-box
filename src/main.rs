slint::include_modules!();

pub mod operation_file;

fn main() {
    let main_window = MainWindow::new().unwrap();

    main_window.on_login(move || {
        println!("登录逻辑");
    });

    let file_name = operation_file::add(1, 2);
    println!("{}", file_name);

    main_window.run().unwrap();
}
