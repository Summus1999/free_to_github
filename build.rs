fn main() {
    // Embed manifest only when compiling GUI version on Windows
    #[cfg(target_os = "windows")]
    {
        let manifest = r#"
1 24 "free_to_github_gui.exe.manifest"
"#;
        
        // Create temporary .rc file
        if let Err(e) = std::fs::write("app.rc", manifest) {
            eprintln!("Warning: Failed to write app.rc: {}", e);
            return;
        }
        
        // Compile resource file using embed-resource
        embed_resource::compile("app.rc", embed_resource::NONE);
        
        // Remove temporary file
        let _ = std::fs::remove_file("app.rc");
    }
}
