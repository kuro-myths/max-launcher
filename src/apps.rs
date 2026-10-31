//! Daftar resmi semua project dalam ekosistem Max.

/// Satu project dalam Max Suite.
pub struct MaxApp {
    pub emoji: &'static str,
    pub name: &'static str,
    pub url: &'static str,
}

/// Semua project Max yang resmi, satu-satunya sumber kebenaran
/// untuk daftar aplikasi yang dikelola Max Launcher.
pub fn max_apps() -> Vec<MaxApp> {
    vec![
        MaxApp {
            emoji: "📡",
            name: "Max Router",
            url: "https://github.com/kuro-myths/max-router",
        },
        MaxApp {
            emoji: "🌌",
            name: "Max Multiver",
            url: "https://github.com/kuro-myths/max-multiver",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_suite_tidak_kosong() {
        assert!(!max_apps().is_empty());
    }

    #[test]
    fn max_router_selalu_ada() {
        assert!(max_apps().iter().any(|a| a.name == "Max Router"));
    }
}
