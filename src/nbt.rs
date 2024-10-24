use clap::{CommandFactory, Parser, ValueEnum};
use crate::general::{color_print, mailto_qr_gen};

#[derive(Parser)]
#[command(arg_required_else_help(true))]
pub struct NBT {

    #[arg(
        short = 'r',
        long,
        value_name = "REMOTE",
        help = "Specify the IFOS device number when asked.\n(e.g., nct -r or nct --live)"
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
        help = "Access live data to monitor your connection and others to IFOS machines.\n(e.g., nct -l or nct --live)"
    )]
    pub get_live_ui: bool,
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
}
