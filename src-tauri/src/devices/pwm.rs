//! PWM management module for OrangePi devices

use crate::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// PWM channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmConfig {
    /// Channel number
    pub channel: u32,
    /// PWM chip number
    pub chip: u32,
    /// Frequency in Hz
    pub frequency: f64,
    /// Duty cycle percentage (0.0 - 100.0)
    pub duty_cycle: f64,
    /// Polarity
    pub polarity: PwmPolarity,
    /// Enabled state
    pub enabled: bool,
}

/// PWM polarity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PwmPolarity {
    /// Normal polarity
    Normal,
    /// Inverted polarity
    Inverted,
}

impl Default for PwmPolarity {
    #[inline]
    fn default() -> Self {
        PwmPolarity::Normal
    }
}

/// PWM channel information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmChannelInfo {
    /// Channel number
    pub channel: u32,
    /// PWM chip
    pub chip: u32,
    /// Current frequency
    pub frequency: f64,
    /// Current duty cycle
    pub duty_cycle: f64,
    /// Current polarity
    pub polarity: PwmPolarity,
    /// Is enabled
    pub enabled: bool,
    /// Period in nanoseconds
    pub period_ns: u64,
    /// Duty cycle in nanoseconds
    pub duty_cycle_ns: u64,
}

/// PWM Waveform pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmWaveform {
    /// Waveform name
    pub name: String,
    /// Frequency steps
    pub frequency_steps: Vec<f64>,
    /// Duty cycle steps (0-100)
    pub duty_steps: Vec<f64>,
    /// Duration for each step in milliseconds
    pub step_duration_ms: u64,
    /// Number of cycles
    pub cycles: u32,
}

/// PWM manager
pub struct PwmManager {
    channels: HashMap<u32, PwmChannel>,
}

/// PWM channel handle
struct PwmChannel {
    config: PwmConfig,
}

impl PwmManager {
    /// Create a new PWM manager
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }
    
    /// List available PWM channels
    pub fn list_channels(&self) -> AppResult<Vec<PwmChannelInfo>> {
        let mut channels = Vec::new();
        
        // OrangePi typically has PWM channels on chip 0
        for chip in 0..2 {
            for channel in 0..4 {
                if let Ok(info) = self.get_channel_info(chip, channel) {
                    channels.push(info);
                }
            }
        }
        
        Ok(channels)
    }
    
    /// Configure a PWM channel
    pub fn configure_channel(&mut self, config: PwmConfig) -> AppResult<()> {
        info!(
            "Configuring PWM channel {} on chip {}: {}Hz, {}% duty",
            config.channel, config.chip, config.frequency, config.duty_cycle
        );
        
        #[cfg(feature = "hardware-support")]
        {
            self.export_channel(config.chip, config.channel)?;
            self.set_period(config.chip, config.channel, config.frequency)?;
            self.set_duty_cycle(config.chip, config.channel, config.frequency, config.duty_cycle)?;
            self.set_polarity(config.chip, config.channel, config.polarity)?;
            self.enable(config.chip, config.channel, config.enabled)?;
        }
        
        let channel = PwmChannel {
            config: config.clone(),
        };
        
        let channel_id = (config.chip << 16) | config.channel;
        self.channels.insert(channel_id, channel);
        
        debug!("PWM channel {} configured successfully", config.channel);
        Ok(())
    }
    
    /// Update PWM frequency
    pub fn set_frequency(&mut self, chip: u32, channel: u32, frequency: f64) -> AppResult<()> {
        let channel_id = (chip << 16) | channel;
        let pwm = self
            .channels
            .get_mut(&channel_id)
            .ok_or_else(|| AppError::Pwm(format!("Channel {} not configured", channel)))?;
        
        #[cfg(feature = "hardware-support")]
        {
            self.set_period(chip, channel, frequency)?;
        }
        
        pwm.config.frequency = frequency;
        Ok(())
    }
    
    /// Update PWM duty cycle
    pub fn set_duty_cycle(&mut self, chip: u32, channel: u32, duty_cycle: f64) -> AppResult<()> {
        if duty_cycle < 0.0 || duty_cycle > 100.0 {
            return Err(AppError::InvalidArgument(
                "Duty cycle must be between 0 and 100".to_string(),
            ));
        }
        
        let channel_id = (chip << 16) | channel;
        let pwm = self
            .channels
            .get_mut(&channel_id)
            .ok_or_else(|| AppError::Pwm(format!("Channel {} not configured", channel)))?;
        
        #[cfg(feature = "hardware-support")]
        {
            self.set_duty_cycle_ns(chip, channel, pwm.config.frequency, duty_cycle)?;
        }
        
        pwm.config.duty_cycle = duty_cycle;
        Ok(())
    }
    
    /// Enable/disable PWM channel
    pub fn set_enabled(&mut self, chip: u32, channel: u32, enabled: bool) -> AppResult<()> {
        let channel_id = (chip << 16) | channel;
        let pwm = self
            .channels
            .get_mut(&channel_id)
            .ok_or_else(|| AppError::Pwm(format!("Channel {} not configured", channel)))?;
        
        #[cfg(feature = "hardware-support")]
        {
            self.enable(chip, channel, enabled)?;
        }
        
        pwm.config.enabled = enabled;
        Ok(())
    }
    
    /// Get channel information
    pub fn get_channel_info(&self, chip: u32, channel: u32) -> AppResult<PwmChannelInfo> {
        let channel_id = (chip << 16) | channel;
        
        if let Some(pwm) = self.channels.get(&channel_id) {
            let period_ns = (1.0 / pwm.config.frequency * 1e9) as u64;
            let duty_ns = (period_ns as f64 * pwm.config.duty_cycle / 100.0) as u64;
            
            Ok(PwmChannelInfo {
                channel: pwm.config.channel,
                chip: pwm.config.chip,
                frequency: pwm.config.frequency,
                duty_cycle: pwm.config.duty_cycle,
                polarity: pwm.config.polarity,
                enabled: pwm.config.enabled,
                period_ns,
                duty_cycle_ns: duty_ns,
            })
        } else {
            // Try to read from sysfs
            self.read_sysfs_info(chip, channel)
        }
    }
    
    /// Unconfigure a channel
    pub fn unconfigure_channel(&mut self, chip: u32, channel: u32) -> AppResult<()> {
        let channel_id = (chip << 16) | channel;
        
        if self.channels.remove(&channel_id).is_some() {
            #[cfg(feature = "hardware-support")]
            {
                self.unexport_channel(chip, channel)?;
            }
            info!("PWM channel {} unconfigured", channel);
        }
        Ok(())
    }
    
    /// Play a waveform pattern
    pub async fn play_waveform(
        &mut self,
        chip: u32,
        channel: u32,
        waveform: PwmWaveform,
    ) -> AppResult<()> {
        info!("Playing waveform '{}' on channel {}", waveform.name, channel);
        
        for _ in 0..waveform.cycles {
            for (i, freq) in waveform.frequency_steps.iter().enumerate() {
                let duty = waveform.duty_steps.get(i).copied().unwrap_or(50.0);
                
                self.set_frequency(chip, channel, *freq)?;
                self.set_duty_cycle(chip, channel, duty)?;
                self.set_enabled(chip, channel, true)?;
                
                tokio::time::sleep(tokio::time::Duration::from_millis(waveform.step_duration_ms))
                    .await;
            }
        }
        
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn export_channel(&self, chip: u32, channel: u32) -> AppResult<()> {
        let export_path = format!("/sys/class/pwm/pwmchip{}/export", chip);
        let channel_path = format!("/sys/class/pwm/pwmchip{}/pwm{}", chip, channel);
        
        if !std::path::Path::new(&channel_path).exists() {
            std::fs::write(&export_path, channel.to_string())
                .map_err(|e| AppError::Pwm(format!("Failed to export PWM channel: {}", e)))?;
        }
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn unexport_channel(&self, chip: u32, channel: u32) -> AppResult<()> {
        let unexport_path = format!("/sys/class/pwm/pwmchip{}/unexport", chip);
        std::fs::write(&unexport_path, channel.to_string())
            .map_err(|e| AppError::Pwm(format!("Failed to unexport PWM channel: {}", e)))?;
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn set_period(&self, chip: u32, channel: u32, frequency: f64) -> AppResult<()> {
        let period_path = format!("/sys/class/pwm/pwmchip{}/pwm{}/period", chip, channel);
        let period_ns = (1.0 / frequency * 1e9) as u64;
        std::fs::write(&period_path, period_ns.to_string())
            .map_err(|e| AppError::Pwm(format!("Failed to set period: {}", e)))?;
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn set_duty_cycle_ns(
        &self,
        chip: u32,
        channel: u32,
        frequency: f64,
        duty_cycle: f64,
    ) -> AppResult<()> {
        let duty_path = format!("/sys/class/pwm/pwmchip{}/pwm{}/duty_cycle", chip, channel);
        let period_ns = (1.0 / frequency * 1e9) as u64;
        let duty_ns = (period_ns as f64 * duty_cycle / 100.0) as u64;
        std::fs::write(&duty_path, duty_ns.to_string())
            .map_err(|e| AppError::Pwm(format!("Failed to set duty cycle: {}", e)))?;
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn set_duty_cycle(
        &self,
        chip: u32,
        channel: u32,
        frequency: f64,
        duty_cycle: f64,
    ) -> AppResult<()> {
        self.set_duty_cycle_ns(chip, channel, frequency, duty_cycle)
    }
    
    #[cfg(feature = "hardware-support")]
    fn set_polarity(&self, chip: u32, channel: u32, polarity: PwmPolarity) -> AppResult<()> {
        let polarity_path = format!("/sys/class/pwm/pwmchip{}/pwm{}/polarity", chip, channel);
        let polarity_str = match polarity {
            PwmPolarity::Normal => "normal",
            PwmPolarity::Inverted => "inversed",
        };
        std::fs::write(&polarity_path, polarity_str)
            .map_err(|e| AppError::Pwm(format!("Failed to set polarity: {}", e)))?;
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn enable(&self, chip: u32, channel: u32, enabled: bool) -> AppResult<()> {
        let enable_path = format!("/sys/class/pwm/pwmchip{}/pwm{}/enable", chip, channel);
        std::fs::write(&enable_path, if enabled { "1" } else { "0" })
            .map_err(|e| AppError::Pwm(format!("Failed to enable PWM: {}", e)))?;
        Ok(())
    }
    
    fn read_sysfs_info(&self, chip: u32, channel: u32) -> AppResult<PwmChannelInfo> {
        // Fallback to reading from sysfs if channel is not in our cache
        Err(AppError::Pwm(format!(
            "Channel {} on chip {} not available",
            channel, chip
        )))
    }
}

impl Default for PwmManager {
    fn default() -> Self {
        Self::new()
    }
}