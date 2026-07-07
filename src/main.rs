use std::{
    env, fs,
    io::{self, Write},
    process::Command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("install") => match args.get(2).map(String::as_str) {
            Some(pkg) => {
                let url = format!(
                    "https://raw.githubusercontent.com/ayxan20145-prog/tarman-pkgs/main/packages/{}.tar.gz",
                    pkg
                );
                let bytes = reqwest::blocking::get(&url)?.bytes()?;
                let pkg_name = format!("{}.tar.gz", pkg);

                fs::write(&pkg_name, &bytes)?;

                print!("Extract tar.gz file? (y/n): ");
                io::stdout().flush()?;
                let mut choice = String::new();
                io::stdin().read_line(&mut choice)?;

                if choice.trim().to_lowercase() == "y" {
                    let extract = Command::new("tar").arg("-xf").arg(&pkg_name).status()?;

                    if !extract.success() {
                        eprintln!("Extract failed");
                    }
                }

                print!("Remove tar.gz file? (y/n): ");
                io::stdout().flush()?;
                let mut choice = String::new();
                io::stdin().read_line(&mut choice)?;

                if choice.trim().to_lowercase() == "y" {
                    let remove = Command::new("rm").arg(&pkg_name).status()?;

                    if !remove.success() {
                        eprintln!("Remove failed");
                    }
                }
            }
            None => println!("Usage: tarman install <package>"),
        },
        Some(cmd) => println!("Unknown command: {}", cmd),
        None => println!("Usage: tarman install <package>"),
    }

    Ok(())
}
