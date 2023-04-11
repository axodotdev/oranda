use std::{sync::mpsc::channel, time::Duration};

use camino::{Utf8Path, Utf8PathBuf};
use crossbeam_channel::{unbounded, Sender};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

use clap::Parser;

use crate::commands::{Build, Serve};
use oranda::errors::*;

#[derive(Clone, Debug, Parser)]
pub struct Dev {
    #[arg(long)]
    port: Option<u16>,
    #[arg(long)]
    project_root: Option<Utf8PathBuf>,
    #[arg(long)]
    config_path: Option<Utf8PathBuf>,
}

impl Dev {
    pub fn run(self) -> Result<()> {
        let (tx, rx) = unbounded();

        let watch_path = self.project_root.clone().unwrap_or(Utf8PathBuf::from("./"));
        watch_and_rebuild(watch_path, tx);
        rayon::spawn(move || loop {
            rx.recv().unwrap();
            Build::new(self.project_root.clone(), self.config_path.clone())
                .run()
                .unwrap_or_else(|e| panic!("{e}"));
            println!("file changed!");
        });

        Serve::new(self.port).run()
    }
}

/// spawns a file watcher for a given file, sending events over the channel
/// whenever the file should be re-read
///
/// Example:
/// let (tx, rx) = crossbeam_channel::unbounded();
/// let path = "./test.txt";
/// rayon::spawn(move || {
///   Fs::spawn_file_watcher(&path, tx)?;
///   rayon::spawn(move || loop {
///     rx.recv();
///     println!("file contents:\n{}", Fs::read_file(&path)?);
///   });
/// });
pub fn watch_and_rebuild<P>(path: P, tx: Sender<()>)
where
    P: AsRef<Utf8Path>,
{
    let path = path.as_ref().to_string();
    rayon::spawn(move || {
        eprintln!("watching {} for changes", &path);

        let (fs_tx, fs_rx) = channel();
        let mut watcher = watcher(fs_tx, Duration::from_secs(1))
            .unwrap_or_else(|_| panic!("could not watch {} for changes", &path));
        watcher
            .watch(&path, RecursiveMode::NonRecursive)
            .unwrap_or_else(|_| panic!("could not watch {} for changes", &path));

        loop {
            match fs_rx.recv().unwrap_or_else(|_| {
                panic!(
                    "an unexpected error occurred while watching {} for changes",
                    &path
                )
            }) {
                DebouncedEvent::NoticeWrite(_) => {
                    eprintln!("change detected in {}...", &path);
                }
                DebouncedEvent::Write(_) => {
                    tx.send(()).unwrap_or_else(|_| {
                        panic!(
                            "an unexpected error occurred while watching {} for changes",
                            &path
                        )
                    });
                }
                _ => {}
            }
        }
    })
}
