pub mod exploits;

use std::io::{self, Read};
//use std::time::{SystemTime, UNIX_EPOCH};
use clap::{Parser, Subcommand};
//use chrono::{FixedOffset, Local, Offset, Utc, TimeZone};
//use chrono_tz::Tz;
use adb_client::ADBDeviceExt;

use crate::exploits::{dualapp, fixbedtime, forcehl, has_package, removefl, removegms, seconduser};

#[derive(Parser)]
#[command(name = "pun", version = "1.0")]
#[command(about = "All in one ADB tool that has multiple FL (Family Link) bypasses")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(visible_alias = "chronolink",about = "for chronolink exploit (time travel to a time where you didnt have downtime with hrs)")]
    DE {
        #[arg(help = "Subtract hours")]
        hours: u64,
    },
    #[command(about = "for DNS exploit (hostname is optional)")]
    DNS {
        #[arg(help = "Optional DNS hostname")]
        host: Option<String>,
    },
    #[command(about = "for TOTP exploit (needs shared secret) (timestamp is optional)")]
    TOTP {
        #[arg(help = "Shared secret ")]
        secret: String,

        #[arg(help = "Optional timestamp for generating codes for the future")]
        ts: Option<u64>,
    },
    #[command(visible_alias = "removefl", about = "to remove FL and mi security center")]
    RFL,
    #[command(visible_alias = "2space", about = "for second space exploit (hyperos)")]
    SECONDSPACE,
    #[command(about = "to remove gms.supervision")]
    REMOVEGMS,
    #[command(about = "to fix the bedtime screen")]
    FIXBEDTIME,
    #[command(about = "to force the hyper launcher (hyperos?)")]
    FORCEHL,
    #[command(visible_alias = "2user", about = "to make a second user and then switch the current user to the second one")]
    SECONDUSER,
    #[command(about = "to reset every exploit (almost)")]
    Reset,
}

// #[derive(Args)]
// pub struct Totpa {
//     pub secret: String,

//     pub ts: Option<u64>
// }

// pub struct Dns {
//     pub host: String,
// }

// pub struct Dea {
//     pub hours: u64,
// }

fn main() {
    for line in alogo() {
        println!("{}", line);
    }
    
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::DE {
            hours,
        }) => exploits::de(hours),
        Some(Commands::DNS {
            host,
        }) => exploits::dns(host),
        Some(Commands::TOTP {
            secret,
    
            ts
        }) => exploits::totp(secret, ts),
        Some(Commands::RFL) => removefl(),
        Some(Commands::SECONDSPACE) => dualapp(),
        Some(Commands::REMOVEGMS) => removegms(),
        Some(Commands::FIXBEDTIME) => fixbedtime(),
        Some(Commands::FORCEHL) => forcehl(),
        Some(Commands::SECONDUSER) => seconduser(),
        Some(Commands::Reset) => reset(),
        None => {
            // println!("Usage: 'pun chronolink <hrs>' for chronolink exploit (time travel to a time where you didnt have downtime with hrs)");
            // println!("       'pun dns <hostname>' for DNS exploit (hostname is optional)");
            // println!("       'pun totp <SECRET> <TS>' for TOTP exploit (needs shared secret) (timestamp is optional)");
            // println!("       'pun removefl' to remove FL and mi security center");
            // println!("       'pun 2space' for second space exploit (hyperos)");
            // println!("       'pun removegms' to remove gms.supervision");
            // println!("       'pun fixbedtime' to fix the bedtime screen");
            // println!("       'pun forcehl' to force the hyper launcher (hyperos?)");
            // println!("       'pun 2user' to make a second user and then switch the current user to the second one");
            // println!("       'pun reset' to reset every exploit (almost)");
            println!("use --help, -h or help for usage");
            println!("use --version or -V for version number");
            std::process::exit(0);
        }
    }
}

fn alogo() -> Vec<String> {
    include_str!("logo").lines().map(String::from).collect()
}

// fn de() {
//     let mut device = get_device();
//     let cc = format("'{}'", Local::now());

//     device.shell_command(&["echo", cc, ">", "/storage/emulated/0/tz.tmp"], &mut std::io::stdout());//cache on device or pc?
//     println!("Cached current timezone");
//     //get tz automatically
//     let utc = Local::now().offset().fix().local_minus_utc()/3600-args.hours;

//     let sign = if utc>=0{"+"} else {"-"};
//     let tz = format!("Etc/GMT{}{}",if utc>=0{"-"}else{"+"},utc.abs());
//     match FixedOffset::east_opt(target_offset_hours*3600){
//         Some(o)=>println!("Offset valid"),
//         None=>{println!("Offset invalid");std::process::exit(0);}
//     }
//     println!("Got timezone!");
//     //set tz
//     device.shell_command(&["service", "call", "alarm", "3", "s16", tz], &mut std::io::stdout());
//     println!("Done!");
//     eq();
// }

// fn dns() {
//     let mut device = get_device();
//     let host:&str = "https://dns.nextdns.io/4a6e8f";//default dns hostname

//     match args.host {
//         Some(x) => host=x,
//     }

//     device.shell_command(&["settings", "put", "global", "private_dns_mode", "hostname"], &mut std::io::stdout());
//     println!("Private DNS mode set to hostname");
//     device.shell_command(&["settings", "put", "global", "private_dns_specifier", host], &mut std::io::stdout());
//     println!("Private DNS specifier set to exploited dns");
//     println!("Done!");
//     eq();
// }

// fn totp() {
//     let ast:u64 = 0;

//     match args.ts {
//         Some(x) => ast=x,
//         None => ast=SystemTime::now(),
//     }

//     println!("parent code: {}", generate_fl_code(args.secret, ast));
//     println!("Done!");
//     eq();
// }

fn reset() {
    let mut device = exploits::get_device();

    //reset dns
    println!("Resetting Private DNS settings");
    let _ = device.shell_command(&["settings", "put", "global", "private_dns_mode", "off"], &mut std::io::stdout());
    println!("Done!");

    //reset tz
    println!("Resetting timezone");
    let mut cachedtz = Vec::new();
    let _ = device.shell_command(&["cat", "/storage/emulated/0/tz.tmp"], &mut cachedtz);
    let _ = device.shell_command(&["service", "call", "alarm", "3", "s16", &exploits::vec2str(cachedtz)], &mut std::io::stdout());
    println!("Done!");

    //reset fix bedtime
    // println!("Resetting fixbedtime");
    // device.shell_command(&["settings", "put", "secure", "accessibility_display_daltonizer_enabled", "1"], &mut std::io::stdout());
    // println!("Done!");
    // these docs are confusing me https://gist.github.com/mrk-han/67a98616e43f86f8482c5ee6dd3faabe

    //reset removegms
    println!("Resetting removegms");
    if !has_package("com.google.android.gms.supervision") {
        device.shell_command(&["pm", "install-existing", "com.google.android.gms.supervision"], &mut std::io::stdout());
    }
    println!("Done!");
 
    //reset removefl
    println!("Resetting removefl");
    if !has_package("com.miui.securitycenter") {
        device.shell_command(&["pm", "install-existing", "com.miui.securitycenter"], &mut std::io::stdout());
    }
    if !has_package("com.google.android.apps.kids.familylinkhelper") {
        device.shell_command(&["pm", "install-existing", "com.google.android.apps.kids.familylinkhelper"], &mut std::io::stdout());
    }
    println!("Done!");

    //reset 2user
    println!("Resetting 2user");
    device.shell_command(&["am", "switch-user", "0"], &mut std::io::stdout());
    println!("Switched user to 0");
    println!("Removing  second user");
    device.shell_command(&["pm", "remove-user", "10"], &mut std::io::stdout());
    println!("Done!");
    eq();
}

fn eq() {
    println!("Press Enter to quit...");
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

// fn get_device() {
//     let mut server = ADBServer::default();
//     let mut device = server.get_device();

//     match device {
//         None => {
//             println!("Device not found");
//             std::process::exit(0);
//         }
//     }

//     return device;
// }