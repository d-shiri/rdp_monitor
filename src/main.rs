use ctrlc;
use nct::db::{fetch_live_data, insert_data, user_auth, is_someone_connected};
use nct::general::{color_print, create_folder, welcome};
use nct::remote_server::RemoteServer;
use nct::{db::fetch_user_history, nbt::NBT, remote_server::get_pc_num};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::{env, thread};
use dotenv::dotenv;
use atty::Stream;

#[tokio::main]
async fn main() {
    dotenv().ok();
    if atty::isnt(Stream::Stdout){
        println!("WARNING: your terminal likely does not support color!");
        println!("Please use a modern terminal such as Windows Terminal or Powrshell v7.4.5 or above for better user experience!");
        println!("Download Windows Terminal: https://github.com/microsoft/terminal");
        println!("Download Modern Powershell: https://github.com/powershell/powershell/releases");
    }
    create_folder(".data");
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--help".to_string()) | args.contains(&"-h".to_string()){
        NBT::trigger_help();
        return;
    } else if args.contains(&"--get-live-ui".to_string()){
        let _ = fetch_live_data(false).await;
        return;
    }
    user_auth().await;
    let args: NBT = NBT::new();
    let mut tasks = vec![];

    static mut RDP_ACTIVE: bool = false;

    if args.rdp {

        welcome();
        ctrlc::set_handler(move || {
            unsafe {
                if RDP_ACTIVE == false {
                    color_print("\rExiting NBT CLI Tool ...            ", "yellow");
                    
                    std::process::exit(0);
                } else {
                    color_print("\nRDP session is active. Please close it before exiting.", "red");
                    thread::yield_now();
                }
            }

        })
        .expect("Error Setting Ctrl+C handler!");
        color_print("Press Ctrl+C to break", "cyan");
        loop {
            let pc_num = get_pc_num();
            match is_someone_connected(pc_num).await{
                Ok(connected) => {
                    if connected{
                        color_print(&format!("WARNING! Someone is already connected to IFOS-TE-{}", pc_num), "red");
                        let current_dir = env::current_dir().expect("Faild to get the current dir!");
                        let ui_path = current_dir.join("ui.exe");
                        let _ = Command::new(ui_path)
                            .stdin(Stdio::null())  // Optional: Prevents the child process from inheriting input
                            .stdout(Stdio::null())  // Optional: Prevents output from being printed to console
                            .stderr(Stdio::null())  // Optional: Prevents error messages from being printed to console
                            .spawn()
                            .expect("Failed to execute the process!");
                        print!("Do you want to kick 'em out and connect anyway? (y/N): ");
                        io::stdout().flush().unwrap();
                        let mut user_input = String::new();
                        io::stdin().read_line(&mut user_input)
                            .expect("Sth went wrong while reading user input");
                        let input = user_input.trim().to_lowercase();
                        if input == "yes" || input == "y" {
                            println!("IFOS-TE{}'s user kicked out! 😁\n", pc_num);
                            let remote_server = RemoteServer::new(pc_num);
                            remote_server.set_credentials();
                            let start_time = insert_data(pc_num).await.unwrap();
                            unsafe {RDP_ACTIVE = true;}
                            // Spawn each connection attempt as a separate task
                            let task = tokio::spawn(async move {
                                remote_server
                                    .open_remote_desktop(&start_time)
                                    .await;
                                unsafe {RDP_ACTIVE = false;}
                            });
                            tasks.push(task);
                        }
                    } else {
                        let remote_server = RemoteServer::new(pc_num);
                        remote_server.set_credentials();
                        let start_time = insert_data(pc_num).await.unwrap();
                        unsafe {RDP_ACTIVE = true;}
                        // Spawn each connection attempt as a separate task
                        let task = tokio::spawn(async move {
                            remote_server
                                .open_remote_desktop(&start_time)
                                .await;
                            unsafe {RDP_ACTIVE = false;}

                        });
                        tasks.push(task);
                    }
                }
                Err(e) =>{
                    eprintln!("Error: {}", e);
                }
                
            }
        }
    }

    if args.live {
            let _ = fetch_live_data(true).await;
    }

    if args.user_history > Some(0) {
        let day = args.user_history.unwrap_or(365);
        let _ = fetch_user_history(day, None).await;
    }

    if let Some(user_input) = args.other_user_history {
        let day = user_input.days;
        let other_user = user_input.username;
        // Assuming `fetch_user_history` is an async function
        let _ = fetch_user_history(day, Some(&other_user)).await;
    }


}
