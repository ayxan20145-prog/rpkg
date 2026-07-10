use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

struct Package {
    name: String,
    description: String,
    version: String,
    url: String,
}

impl Package {
    fn print(&self) {
        println!(
            "Package found\n Name: {}\n Description: {}\n Version: {}",
            self.name, self.description, self.version
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("install") => match args.get(2).map(String::as_str) {
            Some(pkg) => {
                let package = parse_index(pkg)?;

                package.print();

                print!("\nInstall? (y/n): ");
                io::stdout().flush()?;
                let mut choice = String::new();
                io::stdin().read_line(&mut choice)?;
                if choice.trim() == "y" {
                    println!("Installing {}...", pkg);
                    let bytes = reqwest::blocking::get(&package.url)?.bytes()?;
                    let pkg_name = format!("{}.tar.gz", package.name);

                    fs::write(&pkg_name, &bytes)?;

                    println!("Extracting...");
                    let extract = Command::new("tar").arg("-xvf").arg(&pkg_name).status()?;

                    if !extract.success() {
                        return Err("Extract failed".into());
                    }

                    println!("Removing tar.gz file...");
                    fs::remove_file(&pkg_name)?;

                    println!("Moving executable to .local/bin...");
                    let dir = PathBuf::from(std::env::var("HOME")?)
                        .join(".local")
                        .join("bin");

                    fs::create_dir_all(&dir)?;

                    let path_to_executable = format!("{}/bin/{}", package.name, package.name);
                    let path_to_bin = PathBuf::from(std::env::var("HOME")?)
                        .join(".local")
                        .join("bin")
                        .join(&package.name);
                    if path_to_bin.exists() {
                        fs::remove_file(&path_to_bin)?;
                    }
                    fs::rename(&path_to_executable, &path_to_bin)?;

                    let status = Command::new("chmod").arg("+x").arg(&path_to_bin).status()?;

                    if !status.success() {
                        return Err("chmod failed".into());
                    }

                    println!("Cleaning up...");
                    fs::remove_dir_all(&package.name)?;

                    let dir = PathBuf::from(std::env::var("HOME")?)
                        .join(".local")
                        .join("bin");

                    let in_path = env::var_os("PATH")
                        .map(|path| env::split_paths(&path).any(|p| p == dir))
                        .unwrap_or(false);

                    if !in_path {
                        println!("Warning: {} not in path", dir.display());
                    }

                    println!("\nDone!");
                }
            }
            None => println!("Usage: rpkg install <package>"),
        },
        Some("search") => match args.get(2).map(String::as_str) {
            Some(pkg) => {
                let package = parse_index(pkg)?;

                package.print();
            }
            None => println!("Usage: rpkg search <package>"),
        },
        Some("update") => {
            println!("Updating index...");
            let dir = PathBuf::from(std::env::var("HOME")?)
                .join(".local")
                .join("share")
                .join("rpkg");

            fs::create_dir_all(&dir)?;

            let url = "https://raw.githubusercontent.com/ayxan20145-prog/rpkgs/main/index";
            let bytes = reqwest::blocking::get(url)?.bytes()?;

            fs::write(dir.join("index"), &bytes)?;

            println!("\nDone!");
        }
        Some("remove") => match args.get(2).map(String::as_str) {
            Some(pkg) => {
                let path = PathBuf::from(std::env::var("HOME")?)
                    .join(".local")
                    .join("bin")
                    .join(pkg);
                if path.exists() {
                    println!("Removing {}...", pkg);
                    fs::remove_file(path)?;
                    println!("\nDone!");
                } else {
                    println!("Unknown package");
                }
            }
            None => println!("Usage: rpkg remove <package>"),
        },
        Some(cmd) => println!("Unknown command: {}", cmd),
        None => println!(
            "Usage: \n rpkg install <package>\n rpkg search <package>\n rpkg remove <package>\n rpkg update"
        ),
    }

    Ok(())
}
fn parse_index(pkg: &str) -> Result<Package, Box<dyn std::error::Error>> {
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
        return Err(format!("Package '{}' not found.", pkg).into());
    }

    Ok(Package {
        name,
        description,
        version,
        url,
    })
}
