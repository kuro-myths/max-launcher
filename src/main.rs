//! Max Launcher — pusat resmi untuk semua project Max.
//!
//! Saat ini masih tahap awal (early development).

mod apps;

fn main() {
    println!("{}", "=".repeat(50));
    println!("⚡ Max Launcher v{}", env!("CARGO_PKG_VERSION"));
    println!("{}", "=".repeat(50));
    println!();
    println!("Project Max Suite:");
    for app in apps::max_apps() {
        println!("  {} {} — {}", app.emoji, app.name, app.url);
    }
    println!();
    println!("Status: early development. Stay tuned!");
}
