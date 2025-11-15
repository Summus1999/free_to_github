#![windows_subsystem = "windows"]

use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use free_to_github::hosts;

#[cfg(debug_assertions)]
use free_to_github::{info, error};

// Cache duration for status checks (in seconds)
const STATUS_CACHE_DURATION: u64 = 2;

struct GitHubAcceleratorApp {
    status_message: Arc<Mutex<String>>,
    is_enabled: Arc<Mutex<bool>>,
    has_permission: Arc<Mutex<bool>>,
    error_message: Arc<Mutex<Option<String>>>,
    last_status_check: Arc<Mutex<Instant>>,
}

impl Default for GitHubAcceleratorApp {
    fn default() -> Self {
        let is_enabled = match hosts::is_enabled() {
            Ok(enabled) => enabled,
            Err(_) => false,
        };

        let has_permission = hosts::check_permission().is_ok();
        
        Self {
            status_message: Arc::new(Mutex::new("就绪".to_string())),
            is_enabled: Arc::new(Mutex::new(is_enabled)),
            has_permission: Arc::new(Mutex::new(has_permission)),
            error_message: Arc::new(Mutex::new(None)),
            last_status_check: Arc::new(Mutex::new(Instant::now())),
        }
    }
}

impl eframe::App for GitHubAcceleratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Optimized: Only check status periodically, not on every frame
        let should_check = {
            let last_check = self.last_status_check.lock().unwrap();
            last_check.elapsed() > Duration::from_secs(STATUS_CACHE_DURATION)
        };
        
        if should_check {
            // Update status cache
            if let Ok(enabled) = hosts::is_enabled() {
                *self.is_enabled.lock().unwrap() = enabled;
            }
            *self.last_status_check.lock().unwrap() = Instant::now();
        }
        
        // Set custom theme with sky blue background
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(25, 50, 100);  // Dark sky blue background
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(25, 50, 100);  // Remove group borders
        visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::TRANSPARENT;  // Transparent text stroke
        ctx.set_visuals(visuals);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Set background color to sky blue
            ui.style_mut().visuals.panel_fill = egui::Color32::from_rgb(25, 50, 100);
            
            ui.vertical_centered(|ui| {
                ui.add_space(35.0);
                
                // Title area - cleaner design
                ui.heading(egui::RichText::new("🚀 GitHub 加速").size(36.0).color(egui::Color32::from_rgb(100, 220, 255)));
                ui.add_space(30.0);
                
                // Permission check - warning box (no visible border)
                let has_permission = *self.has_permission.lock().unwrap();
                if !has_permission {
                    ui.horizontal(|ui| {
                        ui.add_space(100.0);
                        ui.label(egui::RichText::new("⚠️").size(24.0));
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("需要管理员权限").size(15.0).color(egui::Color32::from_rgb(255, 200, 100)));
                            ui.label(egui::RichText::new("请以管理员身份运行").size(11.0).color(egui::Color32::from_rgb(200, 200, 200)));
                        });
                        ui.add_space(100.0);
                    });
                    ui.add_space(25.0);
                }
                
                // Status display - card style (no border)
                let is_enabled = *self.is_enabled.lock().unwrap();
                let (status_text, status_icon, status_color) = if is_enabled {
                    ("已启用", "✅", egui::Color32::from_rgb(100, 255, 150))
                } else {
                    ("未启用", "⭕", egui::Color32::from_rgb(180, 180, 180))
                };
                
                ui.vertical_centered(|ui| {
                    ui.add_space(25.0);
                    ui.label(egui::RichText::new(status_icon).size(56.0));
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new(status_text).size(24.0).color(status_color).strong());
                    ui.add_space(25.0);
                });
                
                ui.add_space(35.0);
                
                // Control buttons - modern design
                ui.horizontal(|ui| {
                    ui.add_space(60.0);
                    
                    // Enable button
                    let enable_btn = egui::Button::new(
                        egui::RichText::new("🟢 启用").size(16.0).color(egui::Color32::WHITE)
                    )
                    .fill(egui::Color32::from_rgb(60, 180, 120))
                    .min_size(egui::vec2(140.0, 50.0));
                    
                    if ui.add_enabled(has_permission, enable_btn).clicked() {
                        self.enable_acceleration();
                    }
                    
                    ui.add_space(20.0);
                    
                    // Disable button
                    let disable_btn = egui::Button::new(
                        egui::RichText::new("🔴 禁用").size(16.0).color(egui::Color32::WHITE)
                    )
                    .fill(egui::Color32::from_rgb(220, 100, 100))
                    .min_size(egui::vec2(140.0, 50.0));
                    
                    if ui.add_enabled(has_permission, disable_btn).clicked() {
                        self.disable_acceleration();
                    }
                });
                
                ui.add_space(30.0);
                
                // Error message display
                if let Some(error) = self.error_message.lock().unwrap().as_ref() {
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new(error).size(12.0).color(egui::Color32::from_rgb(255, 120, 120)));
                    });
                    ui.add_space(15.0);
                }
                
                ui.add_space(20.0);
                
                // Divider line (subtle)
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("─────────────────").size(12.0).color(egui::Color32::from_rgb(50, 100, 150)));
                });
                ui.add_space(20.0);
                
                // Utility buttons area
                ui.horizontal(|ui| {
                    ui.add_space(30.0);
                    
                    // Refresh DNS button
                    let dns_btn = egui::Button::new(
                        egui::RichText::new("🔄 刷新DNS").size(13.0).color(egui::Color32::WHITE)
                    )
                    .fill(egui::Color32::from_rgb(70, 140, 200))
                    .min_size(egui::vec2(110.0, 38.0));
                    
                    if ui.add(dns_btn).clicked() && cfg!(target_os = "windows") {
                        self.flush_dns();
                    }
                    
                    ui.add_space(12.0);
                    
                    // Open Hosts folder button
                    let hosts_btn = egui::Button::new(
                        egui::RichText::new("📂 Hosts").size(13.0).color(egui::Color32::WHITE)
                    )
                    .fill(egui::Color32::from_rgb(120, 120, 150))
                    .min_size(egui::vec2(100.0, 38.0));
                    
                    if ui.add(hosts_btn).clicked() && cfg!(target_os = "windows") {
                        self.open_hosts_folder();
                    }
                    
                    ui.add_space(12.0);
                    
                    // Open GitHub button
                    let github_btn = egui::Button::new(
                        egui::RichText::new("🔗 GitHub").size(13.0).color(egui::Color32::WHITE)
                    )
                    .fill(if is_enabled { egui::Color32::from_rgb(80, 140, 220) } else { egui::Color32::from_rgb(80, 80, 100) })
                    .min_size(egui::vec2(110.0, 38.0));
                    
                    if ui.add_enabled(is_enabled, github_btn).clicked() {
                        self.open_github();
                    }
                });
                
                ui.add_space(25.0);
                
                // Footer tips
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("💡 启用后建议刷新 DNS").size(10.0).color(egui::Color32::from_rgb(150, 150, 150)));
                });
                
                ui.add_space(15.0);
            });
        });
    }
}

impl GitHubAcceleratorApp {
    fn enable_acceleration(&mut self) {
        // Immediate status update for better UX
        #[cfg(debug_assertions)]
        info!("User triggered enable acceleration");
        
        match hosts::enable() {
            Ok(_) => {
                *self.is_enabled.lock().unwrap() = true;
                *self.status_message.lock().unwrap() = "✓ 加速已启用!".to_string();
                *self.error_message.lock().unwrap() = None;
                *self.last_status_check.lock().unwrap() = Instant::now();
                #[cfg(debug_assertions)]
                info!("GitHub acceleration enabled successfully");
            }
            Err(e) => {
                let error_msg = format!("启用失败: {}", e);
                #[cfg(debug_assertions)]
                error!("Failed to enable acceleration: {}", e);
                *self.error_message.lock().unwrap() = Some(error_msg);
                *self.status_message.lock().unwrap() = "操作失败".to_string();
            }
        }
    }
    
    fn disable_acceleration(&mut self) {
        // Immediate status update for better UX
        #[cfg(debug_assertions)]
        info!("User triggered disable acceleration");
        
        match hosts::disable() {
            Ok(_) => {
                *self.is_enabled.lock().unwrap() = false;
                *self.status_message.lock().unwrap() = "✓ 加速已禁用!".to_string();
                *self.error_message.lock().unwrap() = None;
                *self.last_status_check.lock().unwrap() = Instant::now();
                #[cfg(debug_assertions)]
                info!("GitHub acceleration disabled successfully");
            }
            Err(e) => {
                let error_msg = format!("禁用失败: {}", e);
                #[cfg(debug_assertions)]
                error!("Failed to disable acceleration: {}", e);
                *self.error_message.lock().unwrap() = Some(error_msg);
                *self.status_message.lock().unwrap() = "操作失败".to_string();
            }
        }
    }
    
    fn flush_dns(&mut self) {
        if cfg!(target_os = "windows") {
            match std::process::Command::new("ipconfig")
                .arg("/flushdns")
                .output() {
                Ok(_) => {
                    *self.status_message.lock().unwrap() = "✓ DNS 缓存已刷新!".to_string();
                }
                Err(e) => {
                    *self.error_message.lock().unwrap() = Some(format!("刷新 DNS 失败: {}", e));
                }
            }
        }
    }
    
    fn open_hosts_folder(&mut self) {
        if cfg!(target_os = "windows") {
            let _ = std::process::Command::new("explorer")
                .arg("C:\\Windows\\System32\\drivers\\etc")
                .spawn();
            *self.status_message.lock().unwrap() = "已打开 hosts 文件目录".to_string();
        }
    }
    
    fn open_github(&mut self) {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("cmd")
                .args(&["/C", "start", "https://github.com"])
                .spawn();
        }
        
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("xdg-open")
                .arg("https://github.com")
                .spawn();
        }
        
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open")
                .arg("https://github.com")
                .spawn();
        }
        
        *self.status_message.lock().unwrap() = "正在打开 GitHub...".to_string();
    }
}

fn main() -> Result<(), eframe::Error> {
    // Initialize file logger for debugging GitHub connection issues (debug builds only)
    #[cfg(debug_assertions)]
    {
        let _ = free_to_github::logger::FileLogger::init();
        info!("Application started");
    }
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([520.0, 600.0])
            .with_resizable(false)
            .with_decorations(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "GitHub 访问加速工具",
        options,
        Box::new(|cc| {
            // Setup Chinese font support
            setup_custom_fonts(&cc.egui_ctx);
            Box::new(GitHubAcceleratorApp::default())
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // Add Chinese font support (using system built-in Microsoft YaHei)
    #[cfg(target_os = "windows")]
    {
        if let Ok(font_data) = std::fs::read("C:\\Windows\\Fonts\\msyh.ttc") {
            fonts.font_data.insert(
                "msyh".to_owned(),
                egui::FontData::from_owned(font_data),
            );
            
            // Set Microsoft YaHei as default font
            fonts.families.get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "msyh".to_owned());
            
            fonts.families.get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .push("msyh".to_owned());
        }
    }
    
    ctx.set_fonts(fonts);
}
