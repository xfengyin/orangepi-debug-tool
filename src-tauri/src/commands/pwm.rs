//! PWM command handlers

use crate::commands::{into_response, ApiResponse};
use crate::devices::pwm::{PwmChannelInfo, PwmConfig, PwmWaveform};
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

/// List available PWM channels
#[tauri::command]
pub async fn list_pwm_channels(state: State<'_, AppState>) -> ApiResponse<Vec<PwmChannelInfo>> {
    let pwm = state.pwm.read();
    into_response(pwm.list_channels())
}

/// Configure a PWM channel
#[tauri::command]
pub async fn configure_pwm(
    config: PwmConfig,
    state: State<'_, AppState>,
) -> ApiResponse<()> {
    into_response(async {
        let mut pwm = state.pwm.write();
        pwm.configure_channel(config)
    }())
}

/// Set PWM frequency
#[tauri::command]
pub async fn set_pwm_frequency(
    chip: u32,
    channel: u32,
    frequency: f64,
    state: State<'_, AppState>,
) -> ApiResponse<()> {
    into_response(async {
        let mut pwm = state.pwm.write();
        pwm.set_frequency(chip, channel, frequency)
    }())
}

/// Set PWM duty cycle
#[tauri::command]
pub async fn set_pwm_duty_cycle(
    chip: u32,
    channel: u32,
    duty_cycle: f64,
    state: State<'_, AppState>,
) -> ApiResponse<()> {
    into_response(async {
        let mut pwm = state.pwm.write();
        pwm.set_duty_cycle(chip, channel, duty_cycle)
    }())
}

/// Enable/disable PWM channel
#[tauri::command]
pub async fn set_pwm_enabled(
    chip: u32,
    channel: u32,
    enabled: bool,
    state: State<'_, AppState>,
) -> ApiResponse<()> {
    into_response(async {
        let mut pwm = state.pwm.write();
        pwm.set_enabled(chip, channel, enabled)
    }())
}

/// Get PWM channel info
#[tauri::command]
pub async fn get_pwm_info(
    chip: u32,
    channel: u32,
    state: State<'_, AppState>,
) -> ApiResponse<PwmChannelInfo> {
    into_response(async {
        let pwm = state.pwm.read();
        pwm.get_channel_info(chip, channel)
    }())
}

/// Play a waveform pattern
#[tauri::command]
pub async fn play_pwm_waveform(
    chip: u32,
    channel: u32,
    waveform: PwmWaveform,
    state: State<'_, AppState>,
) -> ApiResponse<()> {
    into_response(async {
        let mut pwm = state.pwm.write();
        pwm.play_waveform(chip, channel, waveform).await
    }.await)
}

/// Unconfigure a PWM channel
#[tauri::command]
pub async fn unconfigure_pwm(
    chip: u32,
    channel: u32,
    state: State<'_, AppState>,
) -> ApiResponse<()> {
    into_response(async {
        let mut pwm = state.pwm.write();
        pwm.unconfigure_channel(chip, channel)
    }())
}