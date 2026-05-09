use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct MetricsCollector {
    counters: RwLock<HashMap<String, Counter>>,
    gauges: RwLock<HashMap<String, Gauge>>,
    histograms: RwLock<HashMap<String, Histogram>>,
    registry: RwLock<Vec<MetricFamily>>,
}

#[derive(Debug, Clone)]
pub struct Counter {
    value: AtomicU64,
    labels: HashMap<String, String>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
            labels: HashMap::new(),
        }
    }

    pub fn with_labels(mut self, labels: HashMap<String, String>) -> Self {
        self.labels = labels;
        self
    }

    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add(&self, v: u64) {
        self.value.fetch_add(v, Ordering::Relaxed);
    }

    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Gauge {
    value: AtomicU64,
    labels: HashMap<String, String>,
}

impl Gauge {
    pub fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
            labels: HashMap::new(),
        }
    }

    pub fn with_labels(mut self, labels: HashMap<String, String>) -> Self {
        self.labels = labels;
        self
    }

    pub fn set(&self, v: u64) {
        self.value.store(v, Ordering::Relaxed);
    }

    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    pub fn dec(&self) {
        self.value.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
}

impl Default for Gauge {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Histogram {
    buckets: Vec<AtomicU64>,
    sum: AtomicU64,
    count: AtomicU64,
    labels: HashMap<String, String>,
}

impl Histogram {
    pub fn new(bounds: &[f64]) -> Self {
        Self {
            buckets: bounds.iter().map(|_| AtomicU64::new(0)).collect(),
            sum: AtomicU64::new(0),
            count: AtomicU64::new(0),
            labels: HashMap::new(),
        }
    }

    pub fn with_labels(mut self, labels: HashMap<String, String>) -> Self {
        self.labels = labels;
        self
    }

    pub fn observe(&self, value: f64) {
        self.sum.fetch_add(value as u64, Ordering::Relaxed);
        self.count.fetch_add(1, Ordering::Relaxed);
        
        let bounds = get_default_bounds();
        for (i, bound) in bounds.iter().enumerate() {
            if value <= *bound {
                self.buckets[i].fetch_add(1, Ordering::Relaxed);
                break;
            }
        }
    }

    pub fn get_count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    pub fn get_sum(&self) -> u64 {
        self.sum.load(Ordering::Relaxed)
    }

    pub fn get_buckets(&self) -> Vec<u64> {
        self.buckets.iter().map(|b| b.load(Ordering::Relaxed)).collect()
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new(&get_default_bounds())
    }
}

fn get_default_bounds() -> Vec<f64> {
    vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
}

#[derive(Debug)]
pub struct MetricFamily {
    pub name: String,
    pub help: String,
    pub metric_type: String,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
            histograms: RwLock::new(HashMap::new()),
            registry: RwLock::new(Vec::new()),
        }
    }

    pub fn register_counter(&self, name: &str, help: &str) {
        let family = MetricFamily {
            name: name.to_string(),
            help: help.to_string(),
            metric_type: "counter".to_string(),
        };
        
        let mut counters = self.counters.write();
        if !counters.contains_key(name) {
            counters.insert(name.to_string(), Counter::new());
            self.registry.write().push(family);
        }
    }

    pub fn register_gauge(&self, name: &str, help: &str) {
        let family = MetricFamily {
            name: name.to_string(),
            help: help.to_string(),
            metric_type: "gauge".to_string(),
        };
        
        let mut gauges = self.gauges.write();
        if !gauges.contains_key(name) {
            gauges.insert(name.to_string(), Gauge::new());
            self.registry.write().push(family);
        }
    }

    pub fn register_histogram(&self, name: &str, help: &str) {
        let family = MetricFamily {
            name: name.to_string(),
            help: help.to_string(),
            metric_type: "histogram".to_string(),
        };
        
        let mut histograms = self.histograms.write();
        if !histograms.contains_key(name) {
            histograms.insert(name.to_string(), Histogram::new(&get_default_bounds()));
            self.registry.write().push(family);
        }
    }

    pub fn increment(&self, name: &str) {
        if let Some(counter) = self.counters.read().get(name) {
            counter.inc();
        }
    }

    pub fn add_counter(&self, name: &str, value: u64) {
        if let Some(counter) = self.counters.read().get(name) {
            counter.add(value);
        }
    }

    pub fn set_gauge(&self, name: &str, value: u64) {
        if let Some(gauge) = self.gauges.read().get(name) {
            gauge.set(value);
        }
    }

    pub fn observe_histogram(&self, name: &str, value: f64) {
        if let Some(histogram) = self.histograms.read().get(name) {
            histogram.observe(value);
        }
    }

    pub fn gather(&self) -> String {
        let mut output = String::new();
        let registry = self.registry.read();
        
        for family in registry.iter() {
            output.push_str(&format!("# HELP {} {}\n", family.name, family.help));
            output.push_str(&format!("# TYPE {} {}\n", family.name, family.metric_type));
            
            match family.metric_type.as_str() {
                "counter" => {
                    let counters = self.counters.read();
                    if let Some(counter) = counters.get(&family.name) {
                        output.push_str(&format!("{} {}\n", family.name, counter.get()));
                    }
                }
                "gauge" => {
                    let gauges = self.gauges.read();
                    if let Some(gauge) = gauges.get(&family.name) {
                        output.push_str(&format!("{} {}\n", family.name, gauge.get()));
                    }
                }
                "histogram" => {
                    let histograms = self.histograms.read();
                    if let Some(histogram) = histograms.get(&family.name) {
                        for (i, count) in histogram.get_buckets().iter().enumerate() {
                            if i < get_default_bounds().len() {
                                output.push_str(&format!("{}_bucket{{le=\"{}\"}} {}\n", 
                                    family.name, get_default_bounds()[i], count));
                            }
                        }
                        output.push_str(&format!("{}_bucket{{le=\"+Inf\"}} {}\n", 
                            family.name, histogram.get_count()));
                        output.push_str(&format!("{}_sum {}\n", family.name, histogram.get_sum()));
                        output.push_str(&format!("{}_count {}\n", family.name, histogram.get_count()));
                    }
                }
                _ => {}
            }
        }
        
        output
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum SerialMetric {
    BytesReceived,
    BytesTransmitted,
    PacketsReceived,
    PacketsTransmitted,
    ConnectionErrors,
    ReadErrors,
    WriteErrors,
    ConnectionDuration,
    ThroughputBps,
}

impl SerialMetric {
    pub fn name(&self) -> &'static str {
        match self {
            SerialMetric::BytesReceived => "serial_bytes_received_total",
            SerialMetric::BytesTransmitted => "serial_bytes_transmitted_total",
            SerialMetric::PacketsReceived => "serial_packets_received_total",
            SerialMetric::PacketsTransmitted => "serial_packets_transmitted_total",
            SerialMetric::ConnectionErrors => "serial_connection_errors_total",
            SerialMetric::ReadErrors => "serial_read_errors_total",
            SerialMetric::WriteErrors => "serial_write_errors_total",
            SerialMetric::ConnectionDuration => "serial_connection_duration_seconds",
            SerialMetric::ThroughputBps => "serial_throughput_bps",
        }
    }
}

#[derive(Debug, Clone)]
pub enum GpioMetric {
    PinReads,
    PinWrites,
    InterruptCount,
    ReadLatencyMs,
    WriteLatencyMs,
}

impl GpioMetric {
    pub fn name(&self) -> &'static str {
        match self {
            GpioMetric::PinReads => "gpio_pin_reads_total",
            GpioMetric::PinWrites => "gpio_pin_writes_total",
            GpioMetric::InterruptCount => "gpio_interrupt_count_total",
            GpioMetric::ReadLatencyMs => "gpio_read_latency_ms",
            GpioMetric::WriteLatencyMs => "gpio_write_latency_ms",
        }
    }
}

#[derive(Debug, Clone)]
pub enum SystemMetric {
    ActiveConnections,
    TaskQueueDepth,
    MemoryUsageBytes,
    CpuUsagePercent,
    UptimeSeconds,
}

impl SystemMetric {
    pub fn name(&self) -> &'static str {
        match self {
            SystemMetric::ActiveConnections => "system_active_connections",
            SystemMetric::TaskQueueDepth => "system_task_queue_depth",
            SystemMetric::MemoryUsageBytes => "system_memory_usage_bytes",
            SystemMetric::CpuUsagePercent => "system_cpu_usage_percent",
            SystemMetric::UptimeSeconds => "system_uptime_seconds",
        }
    }
}
