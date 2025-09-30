use clap::{Args, Parser, Subcommand};
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::{Path, PathBuf}, sync::mpsc};

#[derive(Parser, Debug)]
#[command(version, about, long_about = "a file checker that helps automates activities in the background")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "print events to stdout")]
    Watch(WatchArgs),

    #[command(about = "copy files on change")]
    Sync(SyncArgs),

    #[command(about = "move files by type")]
    Oragnize(OrganizeArgs),

    #[command(about = "run commands on change")]
    Exec(ExecArgs),

}

#[derive(Args, Debug)]
struct  WatchArgs {
    
    path: PathBuf
}


#[derive(Args, Debug)]
struct  SyncArgs {
    
}

#[derive(Args, Debug)]
struct  OrganizeArgs {
    
}

#[derive(Args, Debug)]
struct  ExecArgs {
    
}

fn main() {
    let cli = Cli::parse();
    // // setup clap for users to be able to get things running
    // let (tx, rx) = mpsc::channel::<Result<Event>>();

    // // Use recommended_watcher() to automatically select the best implementation
    // // for your platform. The `EventHandler` passed to this constructor can be a
    // // closure, a `std::sync::mpsc::Sender`, a `crossbeam_channel::Sender`, or
    // // another type the trait is implemented for.
    // let mut watcher = notify::recommended_watcher(tx)?;

    // // Add a path to be watched. All files and directories at that path and
    // // below will be monitored for changes.
    // watcher.watch(Path::new("."), RecursiveMode::Recursive)?;
    // // Block forever, printing out events as they come in
    // for res in rx {
    //     match res {
    //         Ok(event) => println!("event: {:?}", event),
    //         Err(e) => println!("watch error: {:?}", e),
    //     }
    // }

    // Ok(())
}