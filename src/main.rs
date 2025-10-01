use clap::{Args, Parser, Subcommand};
use notify::{
    Event, EventKind, RecursiveMode, Result, Watcher,
    event::{CreateKind, ModifyKind},
};
use std::{
    path::{self, Path, PathBuf},
    sync::mpsc,
};

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "a file checker that helps automates activities in the background"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
struct WatchArgs {
    #[arg(short, long, help = "path to watch")]
    path: PathBuf,
    #[arg(short, long, help = "include watching create")]
    create: Option<bool>,
    #[arg(short, long, help = "include watching modify")]
    modify: Option<bool>,
    #[arg(short, long, help = "include watching access")]
    remove: Option<bool>,
}

#[derive(Args, Debug)]
struct SyncArgs {}

#[derive(Args, Debug)]
struct OrganizeArgs {}

#[derive(Args, Debug)]
struct ExecArgs {}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Watch(WatchArgs {
            path,
            create, // Create(File) |  Modify(Name(Any))
            modify, //  Modify(Data(Content))
            remove, // Remove(File)
        }) => {
            println!("Watching {}", path.display());
            watch_path(
                path,
                create.unwrap_or(true),
                modify.unwrap_or(true),
                remove.unwrap_or(true),
            )?;
        }
        _ => println!("others recieved, ignoring for now"),
    }
    // Ok(())
    // // setup clap for users to be able to get things running
    // let (tx, rx) = mpsc::channel::<Result<Event>>();

    // // Use recommended_watcher() to automatically select the best implementation
    // // for your platform. The `EventHandler` passed to this constructor can be a
    // // closure, a `std::sync::mpsc::Sender`, a `crossbeam_channel::Sender`, or
    // // another type the trait is implemented for.
    // let mut watcher = notify::reco mmended_watcher(tx)?;

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

    Ok(())
}

fn watch_path(path: &PathBuf, _create: bool, _modify: bool, _remove: bool) -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(path, RecursiveMode::Recursive)?;
    for res in rx {
        match res {
            Ok(res) => {
                for paths in &res.paths {
                    if !(paths.to_string_lossy().ends_with("swp") || paths.to_string_lossy().ends_with("swx")) {
                        handle_event(&res);
                    }
                }
            }
            Err(e) => println!( 
                "an error occured while processing events {} ",
                e.to_string()
            ),
        }
    }
    Ok(())
}

fn handle_event(res: &Event) {
    match &res.kind {
        EventKind::Create(CreateKind::File) | EventKind::Modify(ModifyKind::Name(_)) => {
            println!("A file was created: {:?}", &res.paths)
        }
        _ => println!("\n"),
    }
}
