use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::error::{AppError, AppResult};

#[cfg(feature = "hardware-support")]
use gpio_cdev::{Chip, Line, Offset};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmConfig {
    pub chip: u32,
    pub channel: u32,
    pub pin: u32,
    pub frequency: f64,
    pub duty_cycle: f64,
    pub enabled: bool,
    pub polarity: bool,
}

impl PwmConfig {
    pub fn new(chip: u32, channel: u32, pin: u32) -> Self {
        Self {
            chip,
            channel,
            pin,
            frequency: 1000.0,
            duty_cycle: 50.0,
            enabled: false,
            polarity: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwmChannelInfo {
    pub chip: u32,
    pub channel: u32,
    pub pin: u32,
    pub frequency: f64,
    pub duty_cycle: f64,
    pub enabled: bool,
    pub polarity: bool,
}

impl From<&PwmConfig> for PwmChannelInfo {
    fn from(config: &PwmConfig) -> Self {
        Self {
            chip: config.chip,
            channel: config.channel,
            pin: config.pin,
            frequency: config.frequency,
            duty_cycle: config.duty_cycle,
            enabled: config.enabled,
            polarity: config.polarity,
        }
    }
}

pub struct PwmDevice {
    channels: HashMap<u32, PwmConfig>,
    #[cfg(feature = "hardware-support")]
    handles: HashMap<u32, Arc<PwmHandle>>,
}

#[cfg(feature = "hardware-support")]
struct PwmHandle {
    chip: String,
    offset: Offset,
}

impl PwmDevice {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
            #[cfg(feature = "hardware-support")]
            handles: HashMap::new(),
        }
    }
    
    pub fn list_channels(&self) -> Vec<PwmChannelInfo> {
        self.channels
            .values()
            .map(PwmChannelInfo::from)
            .collect()
    }
    
    pub fn configure(&mut self, config: PwmConfig) -> AppResult<()> {
        let channel_id = (config.chip << 16) | config.channel;
        
        #[cfg(feature = "hardware-support")]
        {
            let chip_name = format!("/dev/pwmchip{}", config.chip);
            if let Ok(chip) = Chip::new(&chip_name) {
                let line = chip.get_line(config.pin as Offset)?;
                self.handles.insert(channel_id, Arc::new(PwmHandle {
                    chip: chip_name,
                    offset: config.pin as Offset,
                }));
                debug!("PWM chip {} line {} exported successfully", config.chip, config.pin);
            }
        }
        
        self.channels.insert(channel_id, config);
        debug!("PWM channel {} configured successfully", channel_id);
        Ok(())
    }
    
    pub fn set_frequency(&mut self, chip: u32, channel: u32, frequency: f64) -> AppResult<()> {
        let channel_id = (chip << 16) | channel;
        let frequency_clone = frequency;
        
        let pwm = self
            .channels
            .get_mut(&channel_id)
            .ok_or_else(|| AppError::Pwm(format!("Channel {} not configured", channel)))?;
        
        #[cfg(feature = "hardware-support")]
        {
            let _ = self.set_period(chip, channel, frequency_clone);
        }
        
        pwm.frequency = frequency;
        Ok(())
    }
    
    pub fn set_duty_cycle(&mut self, chip: u32, channel: u32, duty_cycle: f64) -> AppResult<()> {
        if duty_cycle < 0.0 || duty_cycle > 100.0 {
            return Err(AppError::InvalidArgument(
                "Duty cycle must be between 0 and 100".to_string(),
            ));
        }
        
        let channel_id = (chip << 16) | channel;
        let frequency = {
            let pwm = self
                .channels
                .get(&channel_id)
                .ok_or_else(|| AppError::Pwm(format!("Channel {} not configured", channel)))?;
            pwm.frequency
        };
        
        let pwm = self
            .channels
            .get_mut(&channel_id)
            .ok_or_else(|| AppError::Pwm(format!("Channel {} not configured", channel)))?;
        
        #[cfg(feature = "hardware-support")]
        {
            let _ = self.set_duty_cycle_ns(chip, channel, frequency, duty_cycle);
        }
        
        pwm.duty_cycle = duty_cycle;
        Ok(())
    }
    
    pub fn set_enabled(&mut self, chip: u32, channel: u32, enabled: bool) -> AppResult<()> {
        let channel_id = (chip << 16) | channel;
        let pwm = self
            .channels
            .get_mut(&channel_id)
            .ok_or_else(|| AppError::Pwm(format!("Channel {} not configured", channel)))?;
        
        #[cfg(feature = "hardware-support")]
        {
            let _ = self.enable(chip, channel, enabled);
        }
        
        pwm.enabled = enabled;
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn set_period(&mut self, chip: u32, channel: u32, frequency: f64) -> AppResult<()> {
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn set_duty_cycle_ns(&mut self, chip: u32, channel: u32, frequency: f64, duty_cycle: f64) -> AppResult<()> {
        Ok(())
    }
    
    #[cfg(feature = "hardware-support")]
    fn enable(&mut self, chip: u32, channel: u32, enabled: bool) -> AppResult<()> {
        Ok(())
    }
}

impl Default for PwmDevice {
    fn default() -> Self {
        Self::new()
    }
}
