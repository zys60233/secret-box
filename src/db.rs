use sqlite::Connection;

//链接数据库
fn connect() -> Connection {
    sqlite::open("secret.db").expect("数据库连接失败，请重启")
}

//数据库初始化
pub fn init() {
    let con = connect();

    println!("初始化");

    //查询sqlite_master表，判断password表是否存在
    let table_exist_sql = "SELECT * FROM sqlite_master WHERE type = 'table' AND name = 'password';";
    let table_count = con.prepare(table_exist_sql).unwrap().iter().count();

    if table_count == 0 {
        //创建密码表
        let create_table_sql = "CREATE TABLE password (id INTEGER, website TEXT, account TEXT, email TEXT, phone INTEGER, password TEXT);";
        let create_result = con.execute(create_table_sql);
        match create_result {
            Ok(res) => println!("数据表创建成功,{:?}", res),
            Err(err) => println!("数据表创建失败,{:?}", err)
        } 
    }

    //判断初始数据是否创建
    let init_data_exist_sql = "SELECT * FROM password WHERE id = ?";
    let mut init_data_exist_statement = con.prepare(init_data_exist_sql).unwrap();
    init_data_exist_statement.bind((1, 1)).unwrap();
    init_data_exist_statement.next().unwrap();
    let init_id = init_data_exist_statement.read::<i64, _>("id").unwrap();
    if init_id == 0 {
        //创建初始数据
        let create_main_password = "INSERT INTO password VALUES (1, 'secret', 'admin', '', '', '123456');";
        let insert_result = con.execute(create_main_password);
        match insert_result {
            Ok(res) => println!("初始数据插入成功,{:?}", res),
            Err(err) => println!("初始数据插入失败,{:?}", err)
        }
    }
}

//登录
pub fn login(password: String) -> bool {
    let con = connect();
    
    println!("开始获取数据");
    //获取主账户数据
    let main_account_sql = "SELECT * FROM password WHERE id = ?;";
    let mut statement = con.prepare(main_account_sql).unwrap();
    statement.bind((1, 1)).unwrap();
    let _ = statement.next();
    let old_password = statement.read::<String, _>("password").unwrap();
    if password == old_password {
        true
    } else {
        false
    }
}

//获取列表数据
pub fn list_data() -> Vec<(i64, String, String, String, i64, String)> {
    let con = connect();
    let mut list_data: Vec<(i64, String, String, String, i64, String)> = Vec::new();

    println!("这是列表数据");

    //查询账号密码
    let search_sql = "SELECT * FROM password WHERE id > ?";
    let search_result = con.prepare(search_sql)
        .unwrap()
        .into_iter()
        .bind((1, 1))
        .unwrap()
        .map(|row| row.unwrap());

    for row in search_result {
        let account_data = (
            row.read::<i64, _>("id"),
            String::from(row.read::<&str, _>("website")),
            String::from(row.read::<&str, _>("account")),
            String::from(row.read::<&str, _>("email")),
            row.read::<i64, _>("phone"),
            String::from(row.read::<&str, _>("password"))
        );

        list_data.push(account_data);
    } 

    list_data
}

//创建账号
pub fn create_account(website: String, account: String, email: String, phone: String, password: String) -> bool {
    //连接数据库
    let con = connect();

    //查询所有数据
    let search_sql = "SELECT * FROM password ORDER BY id DESC";
    let mut search_result = con.prepare(search_sql).unwrap();
    let _ = search_result.next();
    let mut last_id = search_result.read::<i64, _>("id").unwrap();

    //新的ID
    last_id += 1;

    //插入新数据
    let insert_sql = "INSERT INTO password VALUES (?, ?, ?, ?, ?, ?);";
    let mut insert_statement = con.prepare(insert_sql).unwrap();

    insert_statement.bind((1, last_id)).unwrap();
    insert_statement.bind((2, website.as_str())).unwrap();
    insert_statement.bind((3, account.as_str())).unwrap();
    insert_statement.bind((4, email.as_str())).unwrap();
    insert_statement.bind((5, phone.parse::<f64>().unwrap())).unwrap();
    insert_statement.bind((6, password.as_str())).unwrap();

    let insert_result = insert_statement.next();

    match insert_result {
        Ok(_) => true,
        Err(_) => false
    }
}

//更新管理员密码
pub fn update_admin_password(pwd: String) -> bool {
    //连接数据库
    let con = connect();

    //更新密码
    let update_sql = "UPDATE password SET password = ? WHERE id = ?;";
    let mut insert_statement = con.prepare(update_sql).unwrap();

    insert_statement.bind((1, pwd.as_str())).unwrap();
    insert_statement.bind((2, 1)).unwrap();

    let update_result = insert_statement.next();

    match update_result {
        Ok(_) => true,
        Err(_) => false
    }
}

//删除账号
pub fn delete_account(id: i64) -> bool {
    //连接数据库
    let con = connect();

    //删除密码
    let delete_sql = "DELETE FROM password WHERE id = ?;";
    let mut delete_statement  = con.prepare(delete_sql).unwrap();

    delete_statement.bind((1, id)).unwrap();

    let delete_result = delete_statement.next();

    match delete_result {
        Ok(_) => true,
        Err(_) => false
    }
}