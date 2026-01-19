use rusqlite::{Connection, Error, named_params, params};

#[derive(Debug)]
pub struct Account {
    pub id: i64,
    pub website: String,
    pub account: String,
    pub email: String,
    pub phone: String,
    pub password: String
}

#[derive(Debug)]
pub struct DB {
    pub conn: Connection
}

//链接数据库
pub fn connect () -> Result<DB, String> {
    //建立数据链接
    let conn = Connection::open("secret.db").map_err(|e| format!("数据库链接失败：{}", e))?;

    //加密（使用 SQLCipher 的 `PRAGMA key`）
    //Note: use `PRAGMA key = 'your_key';` to open an encrypted database.
    //If the database file already exists unencrypted, `PRAGMA key` alone
    //won't encrypt it — use `PRAGMA rekey` or recreate the DB.
    let key = "123421321";
    conn.execute_batch(&format!("PRAGMA key = '{}';", key)).map_err(|e| format!("设置加密密钥失败：{}", e))?;

    // 验证提供的 key 是否能正确打开/解密数据库；如果失败，返回友好错误信息。
    let test: Result<i64, Error> = conn.query_row("SELECT count(*) FROM sqlite_master;", [], |r| r.get(0));
    match test {
        Ok(_) => Ok(DB { conn }),
        Err(e) => Err(format!("无法使用提供的密钥打开数据库：{}。如果这是未加密的数据库，删除 secret.db 并重启应用以用新密钥创建加密数据库；如果数据库用其他密钥加密，请使用正确密钥或先 rekey。", e))
    }
}

impl DB { 
    //数据库初始化
    pub fn init(&self) -> bool {
        // Ensure table exists and create it if needed. Use TEXT for phone to simplify input handling.
        let create_table_sql = "CREATE TABLE IF NOT EXISTS password (
            id INTEGER PRIMARY KEY,
            website TEXT,
            account TEXT,
            email TEXT,
            phone TEXT,
            password TEXT
        );";

        if let Err(_) = self.conn.execute(create_table_sql, []) {
            return false;
        }

        // Insert initial admin row if not present
        let insert_admin_sql = "INSERT INTO password(id, website, account, email, phone, password)
            SELECT 1, 'secret', 'admin', '', '', '123456' WHERE NOT EXISTS (SELECT 1 FROM password WHERE id = 1);";

        match self.conn.execute(insert_admin_sql, []) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    //登录
    pub fn login(&self, password: String) -> bool {
        //获取主账户数据
        let main_account_sql = "SELECT password FROM password WHERE id = ?;";
        let main_account_result: Result<String, Error> = self.conn.query_row(main_account_sql, params![1], |row| row.get(0));
        match main_account_result {
            Ok(old_password) => {
                if password == old_password {
                    true
                } else {
                    false
                }
            },
            Err(_) => false
        }
    }

    //获取列表数据
    pub fn list_data(&self) -> Vec<Account> {
        let mut list_data: Vec<Account> = Vec::new();

        //查询账号密码
        let mut stmt = self.conn.prepare("SELECT id, website, account, email, phone, password FROM password WHERE id > ?") .unwrap();
        let rows = stmt.query_map(params![1], |row| {
            Ok(Account {
                id: row.get(0)?,
                website: row.get(1)?,
                account: row.get(2)?,
                email: row.get(3)?,
                phone: row.get(4)?,
                password: row.get(5)?
            })
        }).unwrap();

        for acc in rows {
            if let Ok(a) = acc {
                list_data.push(a);
            }
        }

        list_data
    }

    //创建账号
    pub fn create_account(&self, website: String, account: String, email: String, phone: String, password: String) -> bool {
        //查询所有数据
        // get max id
        let max_id_res: Result<i64, _> = self.conn.query_row("SELECT IFNULL(MAX(id), 0) FROM password", [], |r| r.get(0));
        let mut last_id = match max_id_res {
            Ok(id) => id,
            Err(_) => 0
        };
        last_id += 1;

        let insert_sql = "INSERT INTO password(id, website, account, email, phone, password) VALUES (?, ?, ?, ?, ?, ?);";
        let res = self.conn.execute(insert_sql, params![last_id, website.as_str(), account.as_str(), email.as_str(), phone.as_str(), password.as_str()]);

        match res {
            Ok(_) => true,
            Err(_) => false
        }
    }

    //更新管理员密码
    pub fn update_admin_password(&self, pwd: String) -> bool {
        //更新密码
        let update_sql = "UPDATE password SET password = :password WHERE id = :id;";
        let mut update_statement = self.conn.prepare(update_sql).unwrap();
        let update_result = update_statement.execute(named_params! {
            ":id": 1,
            ":password": pwd.as_str()
        });

        match update_result {
            Ok(_) => true,
            Err(_) => false
        }
    }

    //删除账号
    pub fn delete_account(&self, id: i64) -> bool {
        //删除密码
        let delete_sql = "DELETE FROM password WHERE id = ?;";
        match self.conn.execute(delete_sql, params![id]) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    //编辑
    pub fn edit_account(&self, id: i64, website: String, account: String, email: String, phone: String, password: String) -> bool {
        //更新账号
        let edit_sql = "UPDATE password set website = :website, account = :account, email = :email, phone = :phone, password = :password WHERE id = :id;";
        let mut edit_statement = self.conn.prepare(edit_sql).unwrap();
        let edit_result = edit_statement.execute(named_params! {
            ":id": id,
            ":website": website.as_str(),
            ":account": account.as_str(),
            ":email": email.as_str(),
            ":phone": phone.as_str(),
            ":password": password.as_str()
        });
        
        match edit_result {
            Ok(_) => true,
            Err(_) => false
        }
    }
}