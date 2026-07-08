use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("install") => match args.get(2).map(String::as_str) {
            Some(pkg) => {
                let dir = PathBuf::from(std::env::var("HOME")?)
                    .join(".local")
                    .join("share")
                    .join("rpkg");

                let index = fs::read_to_string(dir.join("index"))?;

                let mut found = false;

                let mut name = String::new();
                let mut description = String::new();
                let mut version = String::new();
                let mut url = String::new();

                for line in index.lines() {
                    let line = line.trim();

                    if !found {
                        if line == pkg {
                            found = true;
                            name = line.to_string();
                        }
                        continue;
                    }

                    if line == "end" {
                        break;
                    }

                    if let Some(value) = line.strip_prefix("description ") {
                        description = value.to_string();
                    } else if let Some(value) = line.strip_prefix("version ") {
                        version = value.to_string();
                    } else if let Some(value) = line.strip_prefix("url ") {
                        url = value.to_string();
                    }
                }

                if !found {
                    println!("Package '{}' not found.", pkg);
                    return Ok(());
                }

                println!(
                    "Package found\n Name: {}\n Description: {}\n Version: {}",
                    name, description, version
                );

                print!("\nInstall? (y/n): ");
                io::stdout().flush()?;
                let mut choice = String::new();
                io::stdin().read_line(&mut choice)?;
                if choice.trim() == "y" {
                    println!("Installing...");
                    let bytes = reqwest::blocking::get(&url)?.bytes()?;
                    let pkg_name = format!("{name}.tar.gz");

                    fs::write(&pkg_name, &bytes)?;

                    println!("Extracting...");
                    let extract = Command::new("tar").arg("-xf").arg(&pkg_name).status()?;

                    if !extract.success() {
                        return Err("Extract failed".into());
                    }

                    println!("Removing tar.gz file...");
                    fs::remove_file(&pkg_name)?;

                    println!("\nDone!");
                }
            }
            None => println!("Usage: rpkg install <package>"),
        },
        Some("update") => {
            let dir = PathBuf::from(std::env::var("HOME")?)
                .join(".local")
                .join("share")
                .join("rpkg");

            fs::create_dir_all(&dir)?;

            let url = "https://raw.githubusercontent.com/ayxan20145-prog/rpkgs/main/index";
            let bytes = reqwest::blocking::get(url)?.bytes()?;

            fs::write(dir.join("index"), &bytes)?;
        }
        Some(cmd) => println!("Unknown command: {}", cmd),
        None => println!("Usage: \nrpkg install <package>\nrpkg update"),
    }

    Ok(())
}
