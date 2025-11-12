#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hosts;

use eframe::egui;
use std::sync::{Arc, Mutex};

struct GitHubAcceleratorApp {
    status_message: Arc<Mutex<String>>,
    is_enabled: Arc<Mutex<bool>>,
    has_permission: Arc<Mutex<bool>>,
    error_message: Arc<Mutex<Option<String>>>,
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
        }
    }
}

impl eframe::App for GitHubAcceleratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                
                // 标题
                ui.heading("🚀 GitHub 访问加速工具");
                ui.add_space(10.0);
                
                ui.label("基于本地 hosts 文件,无需第三方服务器");
                ui.add_space(20.0);
                
                ui.separator();
                ui.add_space(20.0);
                
                // 权限检查
                let has_permission = *self.has_permission.lock().unwrap();
                if !has_permission {
                    ui.colored_label(
                        egui::Color32::RED,
                        "⚠️ 没有管理员权限!"
                    );
                    ui.label("请以管理员身份运行此程序");
                    ui.add_space(10.0);
                }
                
                // 当前状态
                let is_enabled = *self.is_enabled.lock().unwrap();
                let status_text = if is_enabled { "✅ 已启用" } else { "⭕ 未启用" };
                let status_color = if is_enabled { 
                    egui::Color32::from_rgb(0, 200, 0) 
                } else { 
                    egui::Color32::GRAY 
                };
                
                ui.add_space(10.0);
                ui.label(egui::RichText::new("当前状态:").size(18.0));
                ui.label(egui::RichText::new(status_text).size(24.0).color(status_color));
                ui.add_space(20.0);
                
                // 控制按钮
                ui.horizontal(|ui| {
                    ui.add_space(50.0);
                    
                    if ui.add_sized([120.0, 50.0], 
                        egui::Button::new(egui::RichText::new("启用加速").size(16.0))
                    ).clicked() && has_permission {
                        self.enable_acceleration();
                    }
                    
                    ui.add_space(20.0);
                    
                    if ui.add_sized([120.0, 50.0], 
                        egui::Button::new(egui::RichText::new("禁用加速").size(16.0))
                    ).clicked() && has_permission {
                        self.disable_acceleration();
                    }
                });
                
                ui.add_space(20.0);
                
                // 状态消息
                let status_msg = self.status_message.lock().unwrap().clone();
                ui.label(egui::RichText::new(&status_msg).size(14.0).color(egui::Color32::GRAY));
                
                // 错误消息
                if let Some(error) = self.error_message.lock().unwrap().as_ref() {
                    ui.add_space(10.0);
                    ui.colored_label(egui::Color32::RED, error);
                }
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);
                
                // 帮助信息
                ui.label("💡 提示:");
                ui.label("启用/禁用后建议刷新 DNS 缓存");
                if cfg!(target_os = "windows") {
                    ui.label("命令: ipconfig /flushdns");
                }
                
                ui.add_space(10.0);
                ui.label(format!("hosts 文件位置: {}", hosts::get_hosts_path()));
                
                ui.add_space(20.0);
                
                // 刷新DNS按钮
                if ui.button("🔄 刷新 DNS 缓存").clicked() && cfg!(target_os = "windows") {
                    self.flush_dns();
                }
            });
        });
    }
}

impl GitHubAcceleratorApp {
    fn enable_acceleration(&mut self) {
        match hosts::enable() {
            Ok(_) => {
                *self.is_enabled.lock().unwrap() = true;
                *self.status_message.lock().unwrap() = "✓ 加速已启用!".to_string();
                *self.error_message.lock().unwrap() = None;
            }
            Err(e) => {
                *self.error_message.lock().unwrap() = Some(format!("启用失败: {}", e));
                *self.status_message.lock().unwrap() = "操作失败".to_string();
            }
        }
    }
    
    fn disable_acceleration(&mut self) {
        match hosts::disable() {
            Ok(_) => {
                *self.is_enabled.lock().unwrap() = false;
                *self.status_message.lock().unwrap() = "✓ 加速已禁用!".to_string();
                *self.error_message.lock().unwrap() = None;
            }
            Err(e) => {
                *self.error_message.lock().unwrap() = Some(format!("禁用失败: {}", e));
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
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 550.0])
            .with_resizable(false),
        ..Default::default()
    };
    
    eframe::run_native(
        "GitHub 访问加速工具",
        options,
        Box::new(|_cc| Box::new(GitHubAcceleratorApp::default())),
    )
}
