//! 终端仿真模块

use serde::{Deserialize, Serialize};

/// 终端类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TerminalType {
    VT100,
    VT220,
    Xterm,
    Linux,
}

/// ANSI 颜色
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    RGB(u8, u8, u8),
}

/// 终端状态
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TerminalState {
    pub cursor_row: u16,
    pub cursor_col: u16,
    pub foreground: AnsiColor,
    pub background: AnsiColor,
    pub bold: bool,
    pub underline: bool,
    pub reverse: bool,
}

impl Default for TerminalState {
    fn default() -> Self {
        Self {
            cursor_row: 0,
            cursor_col: 0,
            foreground: AnsiColor::White,
            background: AnsiColor::Black,
            bold: false,
            underline: false,
            reverse: false,
        }
    }
}

/// 解析 ANSI 转义序列
pub fn parse_ansi_sequence(data: &str) -> Vec<TerminalCommand> {
    let mut commands = Vec::new();
    let mut chars = data.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if chars.next() == Some('[') {
                // CSI 序列
                let mut params = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphabetic() {
                        let cmd = chars.next().unwrap();
                        commands.push(parse_csi(&params, cmd));
                        break;
                    } else {
                        params.push(chars.next().unwrap());
                    }
                }
            }
        } else {
            commands.push(TerminalCommand::Print(ch));
        }
    }
    
    commands
}

/// CSI 命令
#[derive(Debug, Clone)]
pub enum TerminalCommand {
    Print(char),
    CursorMove { row: i32, col: i32 },
    CursorUp(u16),
    CursorDown(u16),
    CursorForward(u16),
    CursorBackward(u16),
    ClearScreen,
    ClearLine,
    SetColor { fg: Option<AnsiColor>, bg: Option<AnsiColor> },
    SetBold(bool),
    SetUnderline(bool),
    SetReverse(bool),
    Reset,
}

fn parse_csi(params: &str, cmd: char) -> TerminalCommand {
    match cmd {
        'A' => {
            let n = params.parse().unwrap_or(1);
            TerminalCommand::CursorUp(n)
        },
        'B' => {
            let n = params.parse().unwrap_or(1);
            TerminalCommand::CursorDown(n)
        },
        'C' => {
            let n = params.parse().unwrap_or(1);
            TerminalCommand::CursorForward(n)
        },
        'D' => {
            let n = params.parse().unwrap_or(1);
            TerminalCommand::CursorBackward(n)
        },
        'H' | 'f' => {
            let parts: Vec<&str> = params.split(';').collect();
            let row = parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(1);
            let col = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
            TerminalCommand::CursorMove { row, col }
        },
        'J' => TerminalCommand::ClearScreen,
        'K' => TerminalCommand::ClearLine,
        'm' => parse_sgr(params),
        _ => TerminalCommand::Print(cmd),
    }
}

fn parse_sgr(params: &str) -> TerminalCommand {
    if params.is_empty() {
        return TerminalCommand::Reset;
    }
    
    let codes: Vec<u16> = params.split(';')
        .filter_map(|s| s.parse().ok())
        .collect();
    
    for &code in &codes {
        match code {
            0 => return TerminalCommand::Reset,
            1 => return TerminalCommand::SetBold(true),
            4 => return TerminalCommand::SetUnderline(true),
            7 => return TerminalCommand::SetReverse(true),
            22 => return TerminalCommand::SetBold(false),
            24 => return TerminalCommand::SetUnderline(false),
            27 => return TerminalCommand::SetReverse(false),
            30..=37 => {
                let color = match code {
                    30 => AnsiColor::Black,
                    31 => AnsiColor::Red,
                    32 => AnsiColor::Green,
                    33 => AnsiColor::Yellow,
                    34 => AnsiColor::Blue,
                    35 => AnsiColor::Magenta,
                    36 => AnsiColor::Cyan,
                    _ => AnsiColor::White,
                };
                return TerminalCommand::SetColor { fg: Some(color), bg: None };
            },
            _ => {}
        }
    }
    
    TerminalCommand::Reset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_up() {
        let commands = parse_ansi_sequence("\x1b[5A");
        assert!(matches!(commands[0], TerminalCommand::CursorUp(5)));
    }

    #[test]
    fn test_set_color() {
        let commands = parse_ansi_sequence("\x1b[31m");
        assert!(matches!(commands[0], TerminalCommand::SetColor { .. }));
    }
}
