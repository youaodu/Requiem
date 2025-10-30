fn main() {
    // Only embed icon resource on Windows
    #[cfg(target_os = "windows")]
    {
        // Check if winres is available (it's a build dependency)
        if let Err(e) = embed_windows_icon() {
            println!("cargo:warning=Failed to embed Windows icon: {}", e);
        }
    }
}

#[cfg(target_os = "windows")]
fn embed_windows_icon() -> Result<(), Box<dyn std::error::Error>> {
    // Try to use winres if available
    // This will embed the icon into the executable
    let mut res = winres::WindowsResource::new();

    // Set icon if it exists
    if std::path::Path::new("assets/icon.ico").exists() {
        res.set_icon("assets/icon.ico");
    } else if std::path::Path::new("assets/icon.png").exists() {
        // If only PNG exists, use it (winres can handle PNG on some systems)
        // But ideally we should have a .ico file
        println!("cargo:warning=icon.ico not found, using icon.png (consider converting to .ico)");
        res.set_icon("assets/icon.png");
    } else {
        println!("cargo:warning=No icon file found in assets/ directory");
    }

    // Set application metadata
    res.set("ProductName", "Requiem");
    res.set("FileDescription", "A lightweight HTTP client");
    res.set("CompanyName", "youaodu");
    res.set("LegalCopyright", "Copyright © 2024 youao.du@gmail.com");

    res.compile()?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn embed_windows_icon() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
