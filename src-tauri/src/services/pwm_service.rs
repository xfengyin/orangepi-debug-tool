use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use parking_lot::Mutex;
use tracing::{debug, info};

use crate::adapters::{DeviceAdapterRegistry, PwmAdapter, PwmChannelInfo};
use crate::config::PwmDeviceConfig;
use crate::error::{AppError, AppResult};
use crate::observability::MetricsCollector;

#[derive(Debug, Clone)]
pub struct PwmChannel {
    pub channel: u32,
    pub enabled: bool,
    pub frequency: f64,
    pub duty_cycle: f64,
    pub polarity: String,
}

pub struct PwmService {
    registry: Arc<Mutex<DeviceAdapterRegistry>>,
    config: PwmDeviceConfig,
    channels: Arc<RwLock<HashMap<u32, PwmChannel>>>,
    metrics: Arc<MetricsCollector>,
}

impl PwmService {
    pub fn new(registry: Arc<Mutex<DeviceAdapterRegistry>>, config: &PwmDeviceConfig) -> Self {
        let metrics = Arc::new(MetricsCollector::new());
        metrics.register_counter("pwm_channel_enables", "Total PWM channel enables");
        metrics.register_counter("pwm_channel_disables", "Total PWM channel disables");
        metrics.register_counter("pwm_frequency_changes", "Total PWM frequency changes");
        metrics.register_counter("pwm_duty_cycle_changes", "Total PWM duty cycle changes");

        Self {
            registry,
            config: config.clone(),
            channels: Arc::new(RwLock::new(HashMap::new())),
            metrics,
        }
    }

    pub async fn initialize(&self) -> AppResult<()> {
        info!("Initializing PwmService with config: {:?}", self.config);
        
        if let Some(adapter) = self.registry.lock().get_default_pwm() {
            let channels = adapter.list_channels().await
                .map_err(|e| AppError::Pwm(format!("Failed to list channels: {}", e)))?;
            info!("Found {} PWM channels", channels.len());
        }
        
        Ok(())
    }

    pub async fn shutdown(&self) -> AppResult<()> {
        info!("Shutting down PwmService");
        
        let channels = self.channels.read().await;
        for (channel, state) in channels.iter() {
            if state.enabled {
                info!("Disabling PWM channel: {}", channel);
            }
        }
        
        Ok(())
    }

    pub async fn list_channels(&self) -> AppResult<Vec<PwmChannelInfo>> {
        let adapter = self.registry.lock().get_default_pwm()
            .ok_or_else(|| AppError::NotFound("No PWM adapter available".to_string()))?;
        
        adapter.list_channels().await
            .map_err(|e| AppError::Pwm(format!("Failed to list channels: {}", e)))
    }

    pub async fn enable_channel(&self, channel: u32) -> AppResult<()> {
        let adapter = self.registry.lock().get_default_pwm()
            .ok_or_else(|| AppError::NotFound("No PWM adapter available".to_string()))?;

        adapter.enable_channel(0, channel, true).await
            .map_err(|e| AppError::Pwm(format!("Failed to enable channel {}: {}", channel, e)))?;

        let mut channels = self.channels.write().await;
        channels.insert(channel, PwmChannel {
            channel,
            enabled: true,
            frequency: self.config.default_frequency_hz as f64,
            duty_cycle: self.config.default_duty_cycle,
            polarity: "normal".to_string(),
        });

        self.metrics.increment("pwm_channel_enables");
        info!("Enabled PWM channel {}", channel);
        Ok(())
    }

    pub async fn disable_channel(&self, channel: u32) -> AppResult<()> {
        let adapter = self.registry.lock().get_default_pwm()
            .ok_or_else(|| AppError::NotFound("No PWM adapter available".to_string()))?;

        adapter.enable_channel(0, channel, false).await
            .map_err(|e| AppError::Pwm(format!("Failed to disable channel {}: {}", channel, e)))?;

        if let Some(channel_state) = self.channels.write().await.get_mut(&channel) {
            channel_state.enabled = false;
        }

        self.metrics.increment("pwm_channel_disables");
        info!("Disabled PWM channel {}", channel);
        Ok(())
    }

    pub async fn set_frequency(&self, channel: u32, frequency_hz: u32) -> AppResult<()> {
        if frequency_hz == 0 {
            return Err(AppError::InvalidArgument("Frequency must be greater than 0".to_string()));
        }

        let adapter = self.registry.lock().get_default_pwm()
            .ok_or_else(|| AppError::NotFound("No PWM adapter available".to_string()))?;

        adapter.set_frequency(0, channel, frequency_hz as f64).await
            .map_err(|e| AppError::Pwm(format!("Failed to set frequency for channel {}: {}", channel, e)))?;

        if let Some(channel_state) = self.channels.write().await.get_mut(&channel) {
            channel_state.frequency = frequency_hz as f64;
        }

        self.metrics.increment("pwm_frequency_changes");
        debug!("Set PWM channel {} frequency to {} Hz", channel, frequency_hz);
        Ok(())
    }

    pub async fn set_duty_cycle(&self, channel: u32, duty_percent: f64) -> AppResult<()> {
        if !(0.0..=100.0).contains(&duty_percent) {
            return Err(AppError::InvalidArgument(
                "Duty cycle must be between 0 and 100".to_string()
            ));
        }

        let adapter = self.registry.lock().get_default_pwm()
            .ok_or_else(|| AppError::NotFound("No PWM adapter available".to_string()))?;

        adapter.set_duty_cycle(0, channel, duty_percent).await
            .map_err(|e| AppError::Pwm(format!("Failed to set duty cycle for channel {}: {}", channel, e)))?;

        if let Some(channel_state) = self.channels.write().await.get_mut(&channel) {
            channel_state.duty_cycle = duty_percent;
        }

        self.metrics.increment("pwm_duty_cycle_changes");
        debug!("Set PWM channel {} duty cycle to {}%", channel, duty_percent);
        Ok(())
    }

    pub async fn get_channel_state(&self, channel: u32) -> Option<PwmChannel> {
        self.channels.read().await.get(&channel).cloned()
    }

    pub async fn get_all_channels(&self) -> Vec<PwmChannel> {
        self.channels.read().await.values().cloned().collect()
    }

    pub async fn configure_servo(&self, channel: u32, angle: f64) -> AppResult<()> {
        let min_duty = 5.0;
        let max_duty = 10.0;
        let angle_range = 180.0;
        
        let duty_cycle = min_duty + (angle.clamp(0.0, angle_range) / angle_range) * (max_duty - min_duty);
        
        self.set_duty_cycle(channel, duty_cycle).await?;
        debug!("Set PWM channel {} servo angle to {} degrees (duty: {}%)", channel, angle, duty_cycle);
        Ok(())
    }

    pub async fn configure_motor(&self, channel: u32, speed_percent: f64) -> AppResult<()> {
        let speed = speed_percent.clamp(-100.0, 100.0);
        
        let duty_cycle = speed.abs();
        
        self.set_duty_cycle(channel, duty_cycle).await?;
        debug!("Set PWM channel {} motor speed to {}%", channel, speed);
        Ok(())
    }

    pub async fn fade_in(&self, channel: u32, target_duty: f64, duration_ms: u64) -> AppResult<()> {
        let steps = 100;
        let step_duration = duration_ms / steps;
        let current_duty = self.get_channel_state(channel).await
            .map(|c| c.duty_cycle)
            .unwrap_or(0.0);
        let step_size = (target_duty - current_duty) / steps as f64;

        for i in 0..=steps {
            let duty = current_duty + step_size * i as f64;
            self.set_duty_cycle(channel, duty).await?;
            tokio::time::sleep(tokio::time::Duration::from_millis(step_duration)).await;
        }

        info!("Faded in PWM channel {} to {}% over {} ms", channel, target_duty, duration_ms);
        Ok(())
    }

    pub async fn fade_out(&self, channel: u32, duration_ms: u64) -> AppResult<()> {
        let steps = 100;
        let step_duration = duration_ms / steps;
        let current_duty = self.get_channel_state(channel).await
            .map(|c| c.duty_cycle)
            .unwrap_or(0.0);
        let step_size = current_duty / steps as f64;

        for i in 0..=steps {
            let duty = current_duty - step_size * i as f64;
            self.set_duty_cycle(channel, duty.max(0.0)).await?;
            tokio::time::sleep(tokio::time::Duration::from_millis(step_duration)).await;
        }

        info!("Faded out PWM channel {} over {} ms", channel, duration_ms);
        Ok(())
    }

    pub fn get_metrics(&self) -> PwmMetrics {
        let channels = self.channels.blocking_read();
        let enabled_channels = channels.values().filter(|c| c.enabled).count() as u64;
        
        PwmMetrics {
            total_channels: channels.len() as u64,
            enabled_channels,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PwmMetrics {
    pub total_channels: u64,
    pub enabled_channels: u64,
}
