mod hosts;

use hosts::{enable, disable, is_enabled, check_permission};

fn enable_cmd() -> std::io::Result<()> {
    enable()?;
    println!("✓ GitHub 加速已启用!");
    println!("提示: 可能需要刷新DNS缓存:");
    if cfg!(target_os = "windows") {
        println!("  运行命令: ipconfig /flushdns");
    } else {
        println!("  运行命令: sudo systemd-resolve --flush-caches (Linux)");
        println!("           或 sudo dscacheutil -flushcache (macOS)");
    }
    Ok(())
}

fn disable_cmd() -> std::io::Result<()> {
    disable()?;
    println!("✓ GitHub 加速已禁用!");
    Ok(())
}

fn status() -> std::io::Result<()> {
    if is_enabled()? {
        println!("状态: 已启用 ✓");
    } else {
        println!("状态: 未启用");
    }
    Ok(())
}

fn check_permission_exit() {
    if let Err(msg) = check_permission() {
        eprintln!("错误: {}", msg);
        std::process::exit(1);
    }
}

fn print_help() {
    println!("Free to GitHub - 本地 GitHub 访问加速工具");
    println!();
    println!("用法:");
    println!("  free_to_github [命令]");
    println!();
    println!("命令:");
    println!("  enable   启用 GitHub 加速");
    println!("  disable  禁用 GitHub 加速");
    println!("  status   查看当前状态");
    println!("  help     显示帮助信息");
    println!();
    println!("注意: 需要管理员/root 权限运行");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "enable" => {
            check_permission_exit();
            if let Err(e) = enable_cmd() {
                eprintln!("启用失败: {}", e);
                std::process::exit(1);
            }
        }
        "disable" => {
            check_permission_exit();
            if let Err(e) = disable_cmd() {
                eprintln!("禁用失败: {}", e);
                std::process::exit(1);
            }
        }
        "status" => {
            if let Err(e) = status() {
                eprintln!("查询状态失败: {}", e);
                std::process::exit(1);
            }
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            eprintln!("未知命令: {}", command);
            println!();
            print_help();
            std::process::exit(1);
        }
    }
}
