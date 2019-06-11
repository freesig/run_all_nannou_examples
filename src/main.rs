use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;
use rayon::prelude::*;


fn main() {
    let path = format!(
        "{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        "../nannou/target/debug/examples"
    );
    let path = Path::new(&path);
    let entrys = WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some(OsStr::new("exe")))
        .filter(|e| {
            if let Some(s) = e.path().file_stem() {
                !s.to_string_lossy().contains("-")
            } else {
                false
            }
        })
        .map(|e| e.into_path())
        .collect::<Vec<_>>();
    /*
    if let Some(f) = entrys.into_iter().next() {
        let mut child = Command::new(f).spawn().expect("failed to execute child");

        let ecode = child.wait().expect("failed to wait on child");
    }
    */
    entrys.par_iter()
        .for_each(|f|{
            let mut child = Command::new(f).spawn().expect("failed to execute child");
            thread::sleep(Duration::from_secs(10));
            if let Some(ref o) = child.stdout {
                println!("{:?} std out: {:?}", f.file_stem(), o);
            }
            if let Some(ref o) = child.stderr {
                println!("{:?} std err: {:?}", f.file_stem(), o);
            }

            let ecode = child.kill().ok();
            println!("{:?} was running?: {:?}", f.file_stem(), ecode);

        })

    /*
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--example")
        .spawn()
        .expect("failed to execute child");

    let ecode = child.wait().expect("failed to wait on child");
    */
}
