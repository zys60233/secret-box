slint::include_modules!();

use rand::prelude::*;
use slint::SharedString;

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
    main_window.on_create(move |website, account, email, phone, password| {
        db::create_account(website.to_string(), account.to_string(), email.to_string(), phone.to_string(), password.to_string())
    });

    //编辑
    main_window.on_edit(move |id, website, account, email, phone, password| {
        println!("{}", id);
        println!("{}", website);
        println!("{}", account);
        println!("{}", email);
        println!("{}", phone);
        println!("{}", password);
        false
    });

    //生成随机密码
    main_window.on_generate(move || {
        generate_password()
    });

    //修改密码
    main_window.on_save(move |pwd| {
        db::update_admin_password(pwd.to_string())
    });

    //删除
    main_window.on_delete(move |id| {
        db::delete_account(id as i64)
    });

    //列表
    //let data = db::list_data();
    //main_window.invoke_set_list(data);

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
