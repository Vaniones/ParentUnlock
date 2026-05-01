pub mod cg;

use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};
use clap::{Args, Parser, Subcommand};
use chrono::{Utc, TimeZone};
use chrono_tz::Tz;
use adb_client::{ADBServer, ADBDeviceExt};

#[derive(Parser)]
#[command(name = "pun", version = "1.0")]
#[command(about = "All in one ADB tool that has multiple FL (Family Link) bypasses")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(visible_alias = "chronolink")]
    DE(Dea),
    #[command(visible_alias = "dns")]
    DNS(Dnsa),
    #[command(visible_alias = "totp")]
    TOTP(Totpa),
}

#[derive(Args)]
pub struct Totpa {
    pub secret: String,

    pub ts: Option<u64>
}

pub struct Dns {
    pub host: String,
}

pub struct Dea {
    pub hours: u64,
}

fn main() {
    for l in alogo() {
        println!("{}", line);
    }
    
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::DE) => de(),
        Some(Commands::DNS) => dns(),
        Some(Commands::TOTP) => totp(),
        None => {
            println!("Usage: 'pun chronolink <hrs>' for chronolink exploit (time travel to a time where you didnt have downtime with hrs)");
            println!("       'pun dns <hostname> (hostname is optional)' for DNS exploit");
            println!("       'pun totp <SECRET> <TS>' for TOTP exploit (needs shared secret) (timestamp is optional)");
            println!("       'pun reset' to reset dns and chronolink");
            std::process::exit(0);
        }
    }
}

fn alogo() -> Vec<String> {
    include_str!("logo").lines().map(String::from).collect()
}

fn de() {

}

fn dns() {
    let mut device = get_device();
    let host:String = "https://dns.nextdns.io/4a6e8f";//default dns hostname

    match args.host {
        Some(x) => host=x,
    }

    device.shell_command(&["settings", "put", "global", "private_dns_mode", "hostname"], &mut std::io::stdout());
    println!("Private DNS mode set to hostname");
    device.shell_command(&["settings", "put", "global", "private_dns_specifier", host], &mut std::io::stdout());
    println!("Private DNS specifier set to exploited dns");
    eq();
}

fn totp() {
    let ast:u64 = 0;

    match args.ts {
        Some(x) => ast=x,
        None => ast=SystemTime::now(),
    }

    println!("parent code: {}", generate_fl_code(args.secret, ast));
    eq();
}

fn reset() {
    let mut device = get_device();

    //reset dns
    println!("Resetting Private DNS settings");
    device.shell_command(&["settings", "put", "global", "private_dns_mode", "off"], &mut std::io::stdout());
    println!("Done!")

    //reset tz

}

fn eq() {
    println!("Press Enter to quit...");
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn get_device() {
    let mut server = ADBServer::default();
    let mut device = server.get_device();

    match device {
        None => {
            println!("Device not found");
            std::process::exit(0);
        }
    }

    return device;
}