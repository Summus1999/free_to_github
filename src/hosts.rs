use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;

const HOSTS_PATH_WINDOWS: &str = r"C:\Windows\System32\drivers\etc\hosts";
const HOSTS_PATH_UNIX: &str = "/etc/hosts";
const MARKER_START: &str = "# === FREE_TO_GITHUB START ===";
const MARKER_END: &str = "# === FREE_TO_GITHUB END ===";

// GitHub domain and IP mapping
const GITHUB_HOSTS: &[(&str, &str)] = &[
    ("140.82.113.4", "github.com"),
    ("140.82.114.4", "gist.github.com"),
    ("185.199.108.153", "assets-cdn.github.com"),
    ("185.199.109.153", "assets-cdn.github.com"),
    ("185.199.110.153", "assets-cdn.github.com"),
    ("185.199.111.153", "assets-cdn.github.com"),
    ("199.232.69.194", "github.global.ssl.fastly.net"),
    ("185.199.108.133", "raw.githubusercontent.com"),
    ("185.199.109.133", "raw.githubusercontent.com"),
    ("185.199.110.133", "raw.githubusercontent.com"),
    ("185.199.111.133", "raw.githubusercontent.com"),
    ("185.199.108.154", "github.githubassets.com"),
    ("185.199.109.154", "github.githubassets.com"),
    ("185.199.110.154", "github.githubassets.com"),
    ("185.199.111.154", "github.githubassets.com"),
    ("140.82.113.10", "codeload.github.com"),
    ("140.82.114.10", "codeload.github.com"),
    ("185.199.108.133", "cloud.githubusercontent.com"),
    ("185.199.109.133", "cloud.githubusercontent.com"),
    ("185.199.110.133", "cloud.githubusercontent.com"),
    ("185.199.111.133", "cloud.githubusercontent.com"),
    ("185.199.108.153", "githubstatus.com"),
    ("140.82.113.18", "api.github.com"),
    ("140.82.114.18", "api.github.com"),
];

pub fn get_hosts_path() -> &'static str {
    if cfg!(target_os = "windows") {
        HOSTS_PATH_WINDOWS
    } else {
        HOSTS_PATH_UNIX
    }
}

fn read_hosts_file() -> io::Result<Vec<String>> {
    let path = get_hosts_path();
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn is_enabled() -> io::Result<bool> {
    let lines = read_hosts_file()?;
    Ok(lines.iter().any(|line| line.contains(MARKER_START)))
}

pub fn enable() -> io::Result<()> {
    if is_enabled()? {
        return Ok(());
    }

    let hosts_path = get_hosts_path();
    let mut file = OpenOptions::new()
        .append(true)
        .open(hosts_path)?;

    writeln!(file, "\n{}", MARKER_START)?;
    for (ip, domain) in GITHUB_HOSTS {
        writeln!(file, "{} {}", ip, domain)?;
    }
    writeln!(file, "{}", MARKER_END)?;

    Ok(())
}

pub fn disable() -> io::Result<()> {
    if !is_enabled()? {
        return Ok(());
    }

    let lines = read_hosts_file()?;
    let mut new_lines = Vec::new();
    let mut skip = false;

    for line in lines {
        if line.contains(MARKER_START) {
            skip = true;
            continue;
        }
        if line.contains(MARKER_END) {
            skip = false;
            continue;
        }
        if !skip {
            new_lines.push(line);
        }
    }

    // Remove trailing empty lines
    while new_lines.last().map_or(false, |l| l.trim().is_empty()) {
        new_lines.pop();
    }

    let hosts_path = get_hosts_path();
    fs::write(hosts_path, new_lines.join("\n"))?;

    Ok(())
}

pub fn check_permission() -> Result<(), String> {
    let hosts_path = get_hosts_path();
    if !Path::new(hosts_path).exists() {
        return Err(format!("hosts file does not exist: {}", hosts_path));
    }

    match OpenOptions::new().append(true).open(hosts_path) {
        Ok(_) => Ok(()),
        Err(_) => {
            if cfg!(target_os = "windows") {
                Err("No permission to modify hosts file!\nPlease run as administrator".to_string())
            } else {
                Err("No permission to modify hosts file!\nPlease run with sudo".to_string())
            }
        }
    }
}
