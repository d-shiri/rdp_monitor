use clap::{CommandFactory, Parser, ValueEnum};
use crate::general::{color_print, mailto_qr_gen};
use std::str::FromStr;

#[derive(Parser)]
#[command(arg_required_else_help(true))]
pub struct NBT {
    #[arg(
        short = 'r',
        long,
        value_name = "REMOTE",
        help = "Specify the IFOS device number when asked.\n(e.g., nct -r or nct --rdp)"
    )]
    pub rdp: bool,

    #[arg(
        short,
        long,
        help = "Access live data to monitor your connection and others to IFOS machines.\n(e.g., nct -l or nct --live)"
    )]
    pub live: bool,

    #[arg(
        short,
        long,
        value_name = "DAYS",
        help = "Retrieve history for the current user for the past n days.\n(e.g., nct -u 5 or nct --user_history 5)"
    )]
    pub user_history: Option<i32>,

    #[arg(
        short,
        long,
        help = "Access live data to monitor your connection and others to IFOS machines.\n(i.e., This is used in ui.exe)"
    )]
    pub get_live_ui: bool,

    #[arg(
        short,
        long,
        value_name = "USERNAME DAY",
        help = "Get other users' history. You can use this only if you have admin access.\n(e.g.,nct -o \"username day\" or nct --other_user_history \"username day\")",
    )]
    pub other_user_history: Option<OtherUserHistory>,
}
impl NBT {
    pub fn new() -> Self {
        NBT::parse()
    }
    
    pub fn trigger_help(){
        NBT::command().print_help().unwrap();
        println!("\nRepo: https://github.com/d-shiri/rdp_monitor");
        color_print("\nNeed help or want to give feedbak? Scan to send an email:\n", "green");
        mailto_qr_gen();
    }
}
impl Default for NBT {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum RDPUserInput {
    TESTRUNNER,
    FetchData,
    UserHistory,
    OtherUserHistory
}

#[derive(Debug, Clone)]
pub struct OtherUserHistory {
    pub username: String,
    pub days: i32,
}

impl FromStr for OtherUserHistory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Expected two arguments: username and day".to_string());
        }

        let username = parts[0].to_string();
        let days = parts[1].parse::<i32>().map_err(|_| "Invalid number for day".to_string())?;

        Ok(OtherUserHistory { username, days })
    }
}

