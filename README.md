# rpkg
rpkg is a simple package manager written in rust

---

## Installation
### Automatic:
```bash
git clone https://github.com/ayxan20145-prog/rpkg.git
cd rpkg
chmod +x install.sh
./install.sh
```
### Manual:
```bash
git clone https://github.com/ayxan20145-prog/rpkg.git
cd rpkg
cargo build --release
mkdir -p ~/.local/bin
mv target/release/rpkg ~/.local/bin
# Add ~/.local/bin to PATH
rpkg update
```

---

## Usage
### Install a package:
```bash
rpkg install <package>
```

### Remove a package:
```bash
rpkg remove <package>
```

### Search a package:
```bash
rpkg search <package>
```

### Update index
```bash
rpkg update
```

### Upgrade packages
```bash
rpkg upgrade
```

---

## Packages
https://github.com/ayxan20145-prog/rpkgs
