use crate::db::update_data;
use std::{env,
    io::{self, Write},
    process::Command,
};

pub struct RemoteServer {
    username: String,
    password: String,
    server_pre: String,
    server_post: String,
    pc_num: i32,
}

impl RemoteServer {
    pub fn new(pc_num: i32) -> Self {
        let username = env::var("REMOTE_USERNAME").expect("REMOTE_USERNAME is not set in .env").replace("\\\\", "\\");
        let password = env::var("REMOTE_PASS").expect("REMOTE_PASS is not set in .env");
        let server_pre = env::var("REMOTE_SERVER_PRE").expect("REMOTE_SERVER_PRE is not set in .env");
        let server_post = env::var("REMOTE_SERVER_POST").expect("REMOTE_SERVER_POST is not set in .env");
        // println!("{}|{}|{}|{}", username, password, server_pre, server_post);
        // let pc_num = pc_num.unwrap_or_else(get_pc_num);
        let pc_num = pc_num;
        Self {
            username,
            password,
            server_pre,
            server_post,
            pc_num,
        }
    }
    /// Constructs the server name
    pub fn get_server_name(&self) -> String {
        format!("{}{}{}", self.server_pre, self.pc_num, self.server_post)
    }
    /// Sets the credentials for the RDP connection
    pub fn set_credentials(&self) {
        let _ = Command::new("cmdkey")
            .arg(format!("/generic:{}", self.get_server_name()))
            .arg(format!("/user:{}", self.username))
            .arg(format!("/pass:{}", self.password))
            .output()
            .expect("Failed to add cmdkey");
        //println!("Output: {:#?}", cmdkout.stderr);
    }
    /// Makes the actual remote connection
    pub async fn open_remote_desktop(
        &self,
        rdp_start_time: &str,
    ) {
        // let start_time = Instant::now();
        let server_name = self.get_server_name();
        let _output = Command::new("mstsc")
            .arg("/console")
            .arg("/v")
            .arg(&server_name)
            .arg("/f")
            .arg("/admin")
            .output()
            .expect("Failed to execute mstsc.exe");

        // let elapsed_time = start_time.elapsed();
        let _ = update_data(self.pc_num, &rdp_start_time).await;

        // print!(
        //     "\nConnection lasted {:?}s for {}",
        //     elapsed_time,
        //     server_name,
        // );
        // io::stdout().flush().expect("Failed flushing!");
    }
}

#[allow(dead_code)]
pub fn get_pc_num() -> i32 {
    'user: loop {
        let remote_pre = env::var("Remote_SERVER_PREFIX")
            .expect("Remote_SERVER_PREFIX is not set in .env");
        print!("Enter remote PC's number {remote_pre}");
        io::stdout().flush().expect("Failed to flush the output.");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the line!");
        match user_input.trim().parse::<i32>() {
            Ok(parsed_input) => {
                if parsed_input.to_string().len() == 3 {
                    break 'user parsed_input;
                } else {
                    println!("Please enter a 3 digit number. e.g.: 100");
                }
            }
            Err(e) => {
                println!(
                    "Invalid Input! Please enter a valid number. e.g., 100\n{}",
                    e
                );
                continue 'user;
            }
        }
    }
}
