use std::{env, fs};

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

                fs::write(pkg_name, &bytes)?;
            }
            None => println!("Usage: tarman install <package>"),
        },
        Some(cmd) => println!("Unknown command: {}", cmd),
        None => println!("Usage: tarman install <package>"),
    }

    Ok(())
}
