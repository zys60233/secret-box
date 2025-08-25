slint::include_modules!();

use rand::prelude::*;
use slint::{SharedString, VecModel};

pub mod db;

fn main() {
    let main_window = MainWindow::new().unwrap();
 
    //程序初始化
    db::init();

    //登录
    main_window.on_login(move |pwd| {
        db::login(pwd.to_string())
    });

    //新增
    let main_window_create = main_window.clone_strong();
    main_window.on_create(move |website, account, email, phone, password| {
        let create_result = db::create_account(website.to_string(), account.to_string(), email.to_string(), phone.to_string(), password.to_string());
        if create_result {
            set_list_data(&main_window_create);
        }
        create_result
    });

    //编辑
    let main_window_edit = main_window.clone_strong();
    main_window.on_edit(move |id, website, account, email, phone, password| {
        let edit_result = db::edit_account(id as i64, website.to_string(), account.to_string(), email.to_string(), phone.to_string(), password.to_string());
        if edit_result {
            set_list_data(&main_window_edit);
        }
        edit_result
    });

    //生成随机密码
    main_window.on_generate(move || {
        generate_password()
    });

    //修改密码
    let main_window_save = main_window.clone_strong();
    main_window.on_save(move |pwd| { 
        let save_result = db::update_admin_password(pwd.to_string());
        if save_result {
            set_list_data(&main_window_save);
        }
        save_result
    });

    //删除
    let main_window_delete = main_window.clone_strong();
    main_window.on_delete(move |id| {
        let delete_result = db::delete_account(id as i64);
        if delete_result {
            set_list_data(&main_window_delete);
        }
        delete_result
    });
    
    //列表
    set_list_data(&main_window);

    main_window.run().unwrap();
}

//生成随机密码
fn generate_password() -> SharedString {
    let seed: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')'];
    
    let mut rng = rand::rng();
    let mut password = SharedString::new();
    for _ in 0..15 {
        let index = rng.random_range(0..62);
        password.push_str(seed[index].to_string().as_str());
    }

    password
}

//设置列表数据
fn set_list_data(app: &MainWindow) {
    let data = db::list_data();

    let mut list_data: Vec<slint_generatedMainWindow::Account> = Vec::new();
    for account in data {
        let item = slint_generatedMainWindow::Account {
            id: account.id as i32,
            website: account.website.into(),
            account: account.account.into(),
            email: account.email.into(),
            phone: account.phone.into(),
            password: account.password.into()
        };
        list_data.push(item);
    }

    let model = VecModel::from_slice(&list_data);
    app.invoke_set_list(model);
}
