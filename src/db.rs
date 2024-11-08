use crate::general::{color_print, get_user_fullname};
use crate::general::{entropy_gen, get_user_id};
use crate::user_history_data;
use chrono::Utc;
use csv::Writer;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::path::Path;
use std::time::SystemTime;
use std::{env, fs};
use user_history_data::UserHistory;

#[derive(Debug, Serialize, Deserialize)] 
pub struct Users {
    pub id: String,
    pub full_name: String,
    pub team: String,
    pub creation_date: String, //DateTime<Utc>,
    pub active: i8,
    pub admin: i8,
}

#[derive(Debug, Serialize, Deserialize)]
struct LiveData {
    full_name: String,
    remote_pc: i32,
}

//#[derive(Serialize)]
pub struct UserData {
    pub user_id: String,
    pub remote_pc: i32,
    pub rdp_start_time: String, //DateTime<Utc>
    pub rdp_end_time: String,   // DateTime<Utc>
}

#[derive(Deserialize)]
struct IsUserAdmin {
    admin: i32,
}
pub async fn create_new_user(user_id: &str)-> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}:{}/api/create_user",
        env::var("BACKEND_URL").expect("BACKEND_URL is not set in .env"),
        env::var("BACKEND_PORT").expect("BACKEND_PORT is not set in .env")
    );
    if let Some((fullname, team)) = get_user_fullname() {
        let user_data = Users {
            id: user_id.to_string(),
            full_name: fullname,
            team,
            creation_date: Utc::now()
                .naive_utc()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            active: 1,
            admin: 0,
        };
        let client = Client::new();
        let response = client
            .post(url)
            .json(&user_data)
            .send()
            .await?;
        if response.status().is_success(){
            println!("User found!");
        } else {
            println!("Failed to add new user to database! Status: {:?}", response.status());
        }
    } else {
        println!("Problem getting fullname or team!");
    }
    Ok(())
}

async fn user_exists(user_id: &str)-> Result<bool, Box<dyn std::error::Error>> {
    let url = format!(
        "{}:{}/api/get_user_info?id={}",
        env::var("BACKEND_URL").expect("BACKEND_URL is not set in .env"),
        env::var("BACKEND_PORT").expect("BACKEND_PORT is not set in .env"),
        user_id.to_string()
    );
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await?;
    match response.status().is_success() {
        true => Ok(true),
        false => Err("User Not Found!".into()),
    }
}

async fn is_user_admin(user_id: &str)-> Result<bool, Box<dyn std::error::Error>> {
    let url = format!(
        "{}:{}/api/get_user_info?id={}",
        env::var("BACKEND_URL").expect("BACKEND_URL is not set in .env"),
        env::var("BACKEND_PORT").expect("BACKEND_PORT is not set in .env"),
        user_id.to_string()
    );
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await?;
    if response.status().is_success() {
        let user_info: Vec<IsUserAdmin> = response.json().await?;
        if let Some(user) = user_info.get(0) {
            return Ok(user.admin == 1)
        }
    }
    Ok(false)
}

pub async fn user_auth() {
    let user_id = match get_user_id() {
        Ok(id) => id,
        Err(_) => "Failed to get user_id".to_string(),
    };
    // Check if entory file exists
    let foldername = ".data";
    let filename = "entropy.png";
    let file_path = format!("{}/{}", foldername, filename);
    if !Path::new(&file_path).exists() {
        match user_exists(&user_id).await {
            Ok(exists) => {
                if exists{
                    println!("User found in DB.\nEntropy file not found! Generating one...");
                    entropy_gen(&file_path)
                } else {
                    println!("User Not Found in DB!");
                    let _ = create_new_user(&user_id).await;
                }
            }
                Err(e) => {
                    println!("{} | {}", e, user_id);
                    let _ = create_new_user(&user_id).await;
                }
            }
    }
    if let Ok(data) = fs::metadata(&file_path) {
        if data.is_file() {
            let modified_time = data.modified().unwrap();
            let now = SystemTime::now();
            if let Ok(duration) = now.duration_since(modified_time) {
                if duration.as_secs() < 24 * 60 * 60 {
                    println!("User authenticated.")
                } else {
                    println!("User found in DB.\nBut entropy file is expired! Generating a new one...");
                    entropy_gen(&file_path)
                }
            }
        } else {
            entropy_gen(&file_path);
        }
    }
}

pub async fn fetch_live_data(verbose: bool ) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "{}:{}/api/get_live_data",
        env::var("BACKEND_URL").expect("BACKEND_URL is not set in .env"),
        env::var("BACKEND_PORT").expect("BACKEND_PORT is not set in .env"),
    );
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await?;
    if response.status().is_success() {
        let data: Vec<LiveData> = response.json()
            .await.expect("Failed getting json data");
        let file_path = format!("{}.csv", "live_data");
        let mut wtr = Writer::from_path(&file_path)?;
        wtr.write_record(&["first_name", "remote_pc"])?;
        for row in &data {
            let temp = row.full_name.split(',')
                .nth(1)
                .ok_or("Failed to extract first name!")? 
                .trim();
            let firstname = temp.split_whitespace()
                .next()
                .ok_or("Failed to get the first name!")?
                .trim();
            wtr.write_record(&[firstname, &row.remote_pc.to_string()])?;
        }
        wtr.flush()?;
        if verbose{ 
            if data.is_empty(){
            color_print("No connection found. Go ahead and start a connection ;)\n", "green");
            } else {
                let remote_pre = env::var("Remote_SERVER_PREFIX")
                    .expect("Remote_SERVER_PREFIX is not set in .env");
                println!("{}\t\t   {}","Name", "Remote PC");
                println!("{}", "-".repeat(67));
                for row in data {
                    println!(
                        "{}\t   {}{}",
                        &row.full_name, &remote_pre, &row.remote_pc
                    );
                }
                println!("{} END {}", "-".repeat(31), "-".repeat(31));
                color_print(&format!("Info also saved in {}\n", file_path), "cyan");
            }
        }
        Ok(())
    } else {
        print!("ERROR");
        Err("Failed to fetch data".into())
    }
}

pub async fn is_someone_connected(pc_num: i32) -> Result<bool, String>{
    let url = format!(
        "{}:{}/api/get_live_data",
        env::var("BACKEND_URL").expect("BACKEND_URL is not set in .env"),
        env::var("BACKEND_PORT").expect("BACKEND_PORT is not set in .env"),
    );
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to get response! | is_someone_connected");
    if response.status().is_success() {
        let data: Vec<LiveData> = response.json()
            .await.expect("Failed getting json data");
        let mut connected = false;
        for row in &data{
            let remote_pc = row.remote_pc;
            if pc_num == remote_pc {
                connected = true;
            } else {
                connected = false;
            }
        }
        Ok(connected)
    } else {
        let remote_pre = env::var("Remote_SERVER_PREFIX")
            .expect("Remote_SERVER_PREFIX is not set in .env");
        Err(format!("Don't know if is_someone_connected to {}{})\n{}", remote_pre, response.status(), pc_num))
    }
}

pub async fn fetch_user_history(day: i32, other_user: Option<&str>) -> Result<(), Box<dyn std::error::Error>>{
    let my_user_id = match get_user_id() {
        Ok(id) => id,
        Err(_) => "Failed to get user_id".to_string(),
    };
    if let Some(user) = other_user{
        match is_user_admin(&my_user_id).await {
            Ok(is_admin) => {
                if is_admin {
                    color_print("Access granted.", "green");
                    color_print(&format!("Fetching history for user: {}", user), "green");

                } else {
                    color_print("Admin access needed to complete the request!", "red");
                    return Err("Could not fetch user history! access denied!".into())
                }
            }
            Err(e) => {
                eprintln!("Error checking user admin status: {}", e);
            }
        }
    }
    let user_id = other_user.unwrap_or(&my_user_id);
    let url = format!(
        "{}:{}/api/get_user_history?id={}&day={}",
        env::var("BACKEND_URL").expect("BACKEND_URL is not set in .env"),
        env::var("BACKEND_PORT").expect("BACKEND_PORT is not set in .env"),
        user_id,
        day
    );
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .expect("Error while fetching user history");
    if response.status().is_success() {
        let data: Vec<UserHistory> = response.json()
            .await.expect("Failed getting json data - user history");
        let file_path = format!("{}_{}.csv", "user_history", user_id);
        let mut wtr = Writer::from_path(&file_path)?;
        wtr.write_record(&["user_id", "remote_pc", "rdp_start_time", "rdp_end_time"])?;
        for row in &data {
            wtr.write_record(&[&row.user_id, &row.remote_pc.to_string(), 
                &row.rdp_start_time, &row.rdp_end_time])?;
        }
        wtr.flush()?;
        if data.is_empty(){
            color_print("No user history found!\n", "green");
        } else {
            let remote_pre = env::var("Remote_SERVER_PREFIX")
                .expect("Remote_SERVER_PREFIX is not set in .env");
            println!(
                "{}\t   {}\t{}\t\t\t{}",
                "User", "Remote PC", "Start Time", "End Time"
            );
            println!("{}", "-".repeat(67));
            for row in data {
                println!(
                    "{}\t   {}{}\t{}\t{}",
                    &row.user_id, &remote_pre, &row.remote_pc, &row.rdp_start_time, &row.rdp_end_time
                );
            }
            println!("{} END {}", "-".repeat(31), "-".repeat(31));
            color_print(&format!("Info also saved in {}\n", file_path), "cyan");
        }
        Ok(())
    } else {
        print!("ERROR! out of bound input for day: 365 > day > 1");
        Err("Failed to fetch data".into())
    }
}

pub async fn insert_data(pc_num: i32)  -> Result<String, String> {
    let user_id = match get_user_id() {
        Ok(id) => id,
        Err(_) => return Err("Failed to get user_id".to_string().into()),
    };
    let url = format!(
        "{}:{}/api/insert_data",
        env::var("BACKEND_URL").expect("BACKEND_URL is not set in .env"),
        env::var("BACKEND_PORT").expect("BACKEND_PORT is not set in .env"),
    );

    let user_data = UserData {
        user_id,
        remote_pc: pc_num,
        rdp_start_time: Utc::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
        rdp_end_time: Utc::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
    };
    let user_data_json = json!({
        "id": user_data.user_id, 
        "remote_pc": user_data.remote_pc,
        "rdp_start_time": user_data.rdp_start_time,
        "rdp_end_time": user_data.rdp_end_time
    });
    let client = Client::new();
    let respond = client.post(url)
        .json(&user_data_json)
        .send()
        .await
        .expect("Failed to insert data!");
    if respond.status().is_success(){
        Ok(user_data.rdp_start_time.clone())
    } else {
        println!("user_data:\n{:?}", user_data_json);
        Err(format!("Inserting data failed!\n{}", respond.status()))
    }
}

pub async fn update_data(pc_num: i32, rdp_start_time: &str) -> Result<String, String> {
    let url = format!(
        "{}:{}/api/update_data",
        env::var("BACKEND_URL").expect("BACKEND_URL is not set in .env"),
        env::var("BACKEND_PORT").expect("BACKEND_PORT is not set in .env"),
    );
    let user_id = get_user_id().unwrap();
    let user_data = UserData {
        user_id,
        remote_pc: pc_num,
        rdp_start_time: rdp_start_time.to_string(),
        rdp_end_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    let user_data_json = json!({
        "rdp_end_time": user_data.rdp_end_time,
        "rdp_start_time": user_data.rdp_start_time,
        "id": user_data.user_id,
        "remote_pc": pc_num,
        "rdp_start_time": user_data.rdp_start_time,
    });
    let client = Client::new();
    let respond = client.post(url)
        .json(&user_data_json)
        .send()
        .await
        .expect("Failed to update data!");
    if respond.status().is_success(){
        Ok("200".to_string())

    } else {
        Err(format!("Error updating data: {}", respond.status()))
    }
    
}
