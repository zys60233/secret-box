slint::include_modules!();

use rand::{rng, Rng};
use slint::{SharedString, VecModel};
use std::rc::Rc;

pub mod db;

fn main() {
    let main_window = MainWindow::new().unwrap();

    //链接数据库
    let db_handle = match db::connect() {
        Ok(db) => Rc::new(db),
        Err(msg) => {
            eprintln!("{}", msg);
            // 友好退出，避免 panic
            return;
        }
    };

    //数据库初始化
    db_handle.init();

    //登录
    let db_handle_login = Rc::clone(&db_handle);
    main_window.on_login(move |pwd| {
        db_handle_login.login(pwd.to_string())
    });

    
    //新增
    let db_handle_create = Rc::clone(&db_handle);
    let main_window_create = main_window.clone_strong();
    main_window.on_create(move |website, account, email, phone, password| {
        let create_result = db_handle_create.create_account(website.to_string(), account.to_string(), email.to_string(), phone.to_string(), password.to_string());
        if create_result {
            set_list_data(&main_window_create, &db_handle_create);
        }
        create_result
    });

    //编辑
    let db_handle_edit = Rc::clone(&db_handle);
    let main_window_edit = main_window.clone_strong();
    main_window.on_edit(move |id, website, account, email, phone, password| {
        let edit_result = db_handle_edit.edit_account(id as i64, website.to_string(), account.to_string(), email.to_string(), phone.to_string(), password.to_string());
        if edit_result {
            set_list_data(&main_window_edit, &db_handle_edit);
        }
        edit_result
    });

    //生成随机密码
    main_window.on_generate(move || {
        generate_password()
    });

    //修改密码
    let db_handle_save = Rc::clone(&db_handle);
    let main_window_save = main_window.clone_strong();
    main_window.on_save(move |pwd| { 
        let save_result = db_handle_save.update_admin_password(pwd.to_string());
        if save_result {
            set_list_data(&main_window_save, &db_handle_save);
        }
        save_result
    });

    //删除
    let db_handle_delete = Rc::clone(&db_handle);
    let main_window_delete = main_window.clone_strong();
    main_window.on_delete(move |id| {
        let delete_result = db_handle_delete.delete_account(id as i64);
        if delete_result {
            set_list_data(&main_window_delete, &db_handle_delete);
        }
        delete_result
    });
    
    //列表
    set_list_data(&main_window, &Rc::clone(&db_handle));

    main_window.run().unwrap();
}


//生成随机密码
fn generate_password() -> SharedString {
    let seed: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')'];
    
    let mut rng = rng();
    let mut password = SharedString::new();
    for _ in 0..15 {
        let index = rng.random_range(0..seed.len());
        password.push_str(&seed[index].to_string());
    }

    password
}

//设置列表数据
fn set_list_data(app: &MainWindow, db_handle: &Rc<db::DB>) {
    let data = db_handle.list_data();

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