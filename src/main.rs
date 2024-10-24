use ctrlc::set_handler;
use nct::db::{fetch_live_data, insert_data, user_auth};
use nct::general::{color_print, create_folder, welcome};
use nct::remote_server::RemoteServer;
use nct::{db::fetch_user_history, nbt::NBT, remote_server::get_pc_num};

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

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

    if args.rdp {
        welcome();
        set_handler(move || {
            color_print("\rExiting NBT CLI Tool ...            ", "yellow");
            std::process::exit(0);
        })
        .expect("Error Setting Ctrl+C handler!");
        color_print("Press Ctrl+C to break", "cyan");
        loop {
            let pc_num = get_pc_num();
            let remote_server = RemoteServer::new(pc_num);
            remote_server.set_credentials();
            let start_time = insert_data(pc_num).await.unwrap();
            // Spawn each connection attempt as a separate task
            let task = tokio::spawn(async move {
                remote_server
                    .open_remote_desktop(&start_time)
                    .await;
            });

            tasks.push(task);
        }
    }

    if args.live {
            let _ = fetch_live_data(true).await;
    }

    if args.user_history > Some(0) {
        let day = args.user_history.unwrap_or(365);
        let _ = fetch_user_history(day).await;
    }


}
