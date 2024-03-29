#[test]
fn main() {
    use notify::*;
    use std::path::Path;
    // use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
    let path = "./tmp";
    fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

        for res in rx {
            match res {
                Ok(event) => println!("Change: {event:?}"),
                Err(error) => println!("Error: {error:?}"),
            }
        }

        Ok(())
    }
    if let Err(error) = watch(path) {
        panic!("Error: {error:?}");
    }
}
