//! 主题管理模块

use serde::{Deserialize, Serialize};

/// 主题类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ThemeType {
    Light,
    Dark,
    System,
}

/// 颜色方案
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorScheme {
    pub name: String,
    pub primary: String,
    pub secondary: String,
    pub background: String,
    pub surface: String,
    pub text: String,
    pub text_secondary: String,
    pub border: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            primary: "#6366f1".to_string(),
            secondary: "#8b5cf6".to_string(),
            background: "#f8fafc".to_string(),
            surface: "#ffffff".to_string(),
            text: "#1e293b".to_string(),
            text_secondary: "#64748b".to_string(),
            border: "#e2e8f0".to_string(),
            success: "#10b981".to_string(),
            warning: "#f59e0b".to_string(),
            danger: "#ef4444".to_string(),
        }
    }
}

/// 深色主题
pub fn dark_theme() -> ColorScheme {
    ColorScheme {
        name: "Dark".to_string(),
        primary: "#818cf8".to_string(),
        secondary: "#a78bfa".to_string(),
        background: "#0f172a".to_string(),
        surface: "#1e293b".to_string(),
        text: "#f1f5f9".to_string(),
        text_secondary: "#94a3b8".to_string(),
        border: "#334155".to_string(),
        success: "#34d399".to_string(),
        warning: "#fbbf24".to_string(),
        danger: "#f87171".to_string(),
    }
}

/// 主题管理器
pub struct ThemeManager {
    current_theme: ThemeType,
    color_scheme: ColorScheme,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            current_theme: ThemeType::System,
            color_scheme: ColorScheme::default(),
        }
    }
    
    /// 设置主题
    pub fn set_theme(&mut self, theme: ThemeType) {
        self.current_theme = theme;
        self.update_color_scheme();
    }
    
    /// 获取当前主题
    pub fn get_theme(&self) -> &ThemeType {
        &self.current_theme
    }
    
    /// 获取当前颜色方案
    pub fn get_color_scheme(&self) -> &ColorScheme {
        &self.color_scheme
    }
    
    /// 更新颜色方案
    fn update_color_scheme(&mut self) {
        self.color_scheme = match self.current_theme {
            ThemeType::Light => ColorScheme::default(),
            ThemeType::Dark => dark_theme(),
            ThemeType::System => {
                // TODO: 检测系统主题
                ColorScheme::default()
            }
        };
    }
    
    /// 获取所有可用主题
    pub fn get_available_themes() -> Vec<ThemeType> {
        vec![ThemeType::Light, ThemeType::Dark, ThemeType::System]
    }
    
    /// 导出为 CSS 变量
    pub fn export_css(&self) -> String {
        format!(
            r#":root {{
  --color-primary: {};
  --color-secondary: {};
  --color-background: {};
  --color-surface: {};
  --color-text: {};
  --color-text-secondary: {};
  --color-border: {};
  --color-success: {};
  --color-warning: {};
  --color-danger: {};
}}"#,
            self.color_scheme.primary,
            self.color_scheme.secondary,
            self.color_scheme.background,
            self.color_scheme.surface,
            self.color_scheme.text,
            self.color_scheme.text_secondary,
            self.color_scheme.border,
            self.color_scheme.success,
            self.color_scheme.warning,
            self.color_scheme.danger,
        )
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_switch() {
        let mut manager = ThemeManager::new();
        manager.set_theme(ThemeType::Dark);
        assert_eq!(*manager.get_theme(), ThemeType::Dark);
    }
    
    #[test]
    fn test_css_export() {
        let manager = ThemeManager::new();
        let css = manager.export_css();
        assert!(css.contains("--color-primary:"));
    }
}
