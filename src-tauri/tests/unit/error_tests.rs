#[cfg(test)]
mod tests {
    use crate::error::{
        AppError, CircuitBreaker, CircuitBreakerState, CircuitBreakerConfig,
        ErrorCollector, ErrorStats, RetryHandler
    };
    use std::time::Duration;

    #[test]
    fn test_error_codes() {
        let serial_err = AppError::Serial("test".to_string());
        assert_eq!(serial_err.code(), "SERIAL_ERROR");
        assert_eq!(serial_err.numeric_code(), 1001);
        
        let gpio_err = AppError::Gpio("test".to_string());
        assert_eq!(gpio_err.code(), "GPIO_ERROR");
        assert_eq!(gpio_err.numeric_code(), 2001);
        
        let pwm_err = AppError::Pwm("test".to_string());
        assert_eq!(pwm_err.code(), "PWM_ERROR");
        assert_eq!(pwm_err.numeric_code(), 3001);
    }

    #[test]
    fn test_error_retryable() {
        let timeout_err = AppError::Timeout("test".to_string());
        assert!(timeout_err.is_retryable());
        
        let io_err = AppError::Io("test".to_string());
        assert!(io_err.is_retryable());
        
        let serial_err = AppError::Serial("test".to_string());
        assert!(!serial_err.is_retryable());
    }

    #[test]
    fn test_error_critical() {
        let device_err = AppError::Device("test".to_string());
        assert!(device_err.is_critical());
        
        let circuit_err = AppError::CircuitBreakerOpen("test".to_string());
        assert!(circuit_err.is_critical());
        
        let serial_err = AppError::Serial("test".to_string());
        assert!(!serial_err.is_critical());
    }

    #[test]
    fn test_error_user_message() {
        let serial_err = AppError::Serial("port not found".to_string());
        let msg = serial_err.to_user_message();
        assert!(msg.contains("串口通信错误"));
        assert!(msg.contains("port not found"));
    }

    #[test]
    fn test_error_recovery_suggestion() {
        let timeout_err = AppError::Timeout("connection".to_string());
        let suggestion = timeout_err.recovery_suggestion();
        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("超时"));
        
        let serial_err = AppError::Serial("test".to_string());
        let suggestion = serial_err.recovery_suggestion();
        assert!(suggestion.is_some());
    }

    #[test]
    fn test_circuit_breaker_initial_state() {
        let cb = CircuitBreaker::new("test");
        assert_eq!(cb.state(), CircuitBreakerState::Closed);
        assert!(!cb.is_open());
    }

    #[test]
    fn test_circuit_breaker_failure_threshold() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            half_open_max_calls: 2,
        };
        let mut cb = CircuitBreaker::new("test").with_config(config);
        
        cb.record_failure();
        assert_eq!(cb.state(), CircuitBreakerState::Closed);
        
        cb.record_failure();
        assert_eq!(cb.state(), CircuitBreakerState::Closed);
        
        cb.record_failure();
        assert_eq!(cb.state(), CircuitBreakerState::Open);
        assert!(cb.is_open());
    }

    #[test]
    fn test_circuit_breaker_half_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            half_open_max_calls: 2,
        };
        let mut cb = CircuitBreaker::new("test").with_config(config);
        
        cb.record_failure();
        assert!(cb.is_open());
        
        std::thread::sleep(Duration::from_millis(150));
        assert_eq!(cb.state(), CircuitBreakerState::HalfOpen);
        
        cb.record_success();
        assert_eq!(cb.state(), CircuitBreakerState::HalfOpen);
        
        cb.record_success();
        assert_eq!(cb.state(), CircuitBreakerState::Closed);
    }

    #[test]
    fn test_circuit_breaker_half_open_failure() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            half_open_max_calls: 2,
        };
        let mut cb = CircuitBreaker::new("test").with_config(config);
        
        cb.record_failure();
        std::thread::sleep(Duration::from_millis(150));
        
        assert_eq!(cb.state(), CircuitBreakerState::HalfOpen);
        cb.record_failure();
        assert_eq!(cb.state(), CircuitBreakerState::Open);
    }

    #[test]
    fn test_circuit_breaker_reset() {
        let mut cb = CircuitBreaker::new("test");
        
        for _ in 0..5 {
            cb.record_failure();
        }
        assert!(cb.is_open());
        
        cb.reset();
        assert_eq!(cb.state(), CircuitBreakerState::Closed);
        assert!(!cb.is_open());
    }

    #[test]
    fn test_error_collector_record() {
        let collector = ErrorCollector::new();
        
        collector.record("SERIAL_ERROR", "port not found");
        collector.record("SERIAL_ERROR", "connection refused");
        collector.record("GPIO_ERROR", "pin not found");
        
        let serial_stats = collector.get_stats("SERIAL_ERROR").unwrap();
        assert_eq!(serial_stats.count, 2);
        
        let gpio_stats = collector.get_stats("GPIO_ERROR").unwrap();
        assert_eq!(gpio_stats.count, 1);
    }

    #[test]
    fn test_error_collector_top_errors() {
        let collector = ErrorCollector::new();
        
        collector.record("ERROR_A", "test");
        collector.record("ERROR_A", "test");
        collector.record("ERROR_A", "test");
        collector.record("ERROR_B", "test");
        collector.record("ERROR_C", "test");
        
        let top = collector.get_top_errors(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, "ERROR_A");
        assert_eq!(top[0].1.count, 3);
        assert_eq!(top[1].0, "ERROR_B");
        assert_eq!(top[1].1.count, 1);
    }

    #[test]
    fn test_error_collector_clear() {
        let collector = ErrorCollector::new();
        
        collector.record("ERROR", "test");
        assert!(collector.get_stats("ERROR").is_some());
        
        collector.clear();
        assert!(collector.get_stats("ERROR").is_none());
    }

    #[tokio::test]
    async fn test_retry_handler_success_first_attempt() {
        let handler = RetryHandler::new(3);
        let counter = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        
        let result = handler.execute(async {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok::<_, ()>(42)
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_handler_retry_on_failure() {
        let handler = RetryHandler::new(3);
        let counter = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        
        let result = handler.execute(async {
            let count = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if count < 2 {
                Err("retry")
            } else {
                Ok(42)
            }
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_handler_max_attempts() {
        let handler = RetryHandler::new(3);
        let counter = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        
        let result = handler.execute(async {
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Err::<i32, _>("fail")
        }).await;
        
        assert!(result.is_err());
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 3);
    }
}
