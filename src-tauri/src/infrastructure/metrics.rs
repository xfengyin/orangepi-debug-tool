use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Counter {
    value: Arc<AtomicU64>,
    name: String,
    labels: HashMap<String, String>,
}

impl Counter {
    pub fn new(name: &str, labels: HashMap<String, String>) -> Self {
        Self {
            value: Arc::new(AtomicU64::new(0)),
            name: name.to_string(),
            labels,
        }
    }
    
    pub fn inc(&self, delta: u64) {
        self.value.fetch_add(delta, Ordering::Relaxed);
    }
    
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
    
    pub fn reset(&self) {
        self.value.store(0, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone)]
pub struct Gauge {
    value: Arc<AtomicU64>,
    name: String,
    labels: HashMap<String, String>,
}

impl Gauge {
    pub fn new(name: &str, labels: HashMap<String, String>) -> Self {
        Self {
            value: Arc::new(AtomicU64::new(0)),
            name: name.to_string(),
            labels,
        }
    }
    
    pub fn set(&self, value: u64) {
        self.value.store(value, Ordering::Relaxed);
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

#[derive(Debug, Clone)]
pub struct Histogram {
    values: Arc<RwLock<Vec<u64>>>,
    name: String,
    labels: HashMap<String, String>,
}

impl Histogram {
    pub fn new(name: &str, labels: HashMap<String, String>) -> Self {
        Self {
            values: Arc::new(RwLock::new(Vec::new())),
            name: name.to_string(),
            labels,
        }
    }
    
    pub fn observe(&self, value: u64) {
        self.values.write().push(value);
    }
    
    pub fn get_stats(&self) -> HistogramStats {
        let values = self.values.read();
        if values.is_empty() {
            return HistogramStats {
                count: 0,
                sum: 0,
                min: 0,
                max: 0,
                mean: 0.0,
                p50: 0,
                p95: 0,
                p99: 0,
            };
        }
        
        let mut sorted = values.clone();
        sorted.sort();
        
        let count = sorted.len() as u64;
        let sum: u64 = sorted.iter().sum();
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        let mean = sum as f64 / count as f64;
        
        fn percentile(sorted: &[u64], p: f64) -> u64 {
            let idx = ((sorted.len() as f64 - 1.0) * p).round() as usize;
            sorted[idx.min(sorted.len() - 1)]
        }
        
        HistogramStats {
            count,
            sum,
            min,
            max,
            mean,
            p50: percentile(&sorted, 0.50),
            p95: percentile(&sorted, 0.95),
            p99: percentile(&sorted, 0.99),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistogramStats {
    pub count: u64,
    pub sum: u64,
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub p50: u64,
    pub p95: u64,
    pub p99: u64,
}

pub struct MetricsCollector {
    counters: RwLock<HashMap<String, Arc<Counter>>>,
    gauges: RwLock<HashMap<String, Arc<Gauge>>>,
    histograms: RwLock<HashMap<String, Arc<Histogram>>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
            histograms: RwLock::new(HashMap::new()),
        }
    }
    
    pub fn register_counter(&self, name: &str, labels: HashMap<String, String>) -> Arc<Counter> {
        let counter = Arc::new(Counter::new(name, labels.clone()));
        self.counters.write().insert(name.to_string(), counter.clone());
        counter
    }
    
    pub fn register_gauge(&self, name: &str, labels: HashMap<String, String>) -> Arc<Gauge> {
        let gauge = Arc::new(Gauge::new(name, labels.clone()));
        self.gauges.write().insert(name.to_string(), gauge.clone());
        gauge
    }
    
    pub fn register_histogram(&self, name: &str, labels: HashMap<String, String>) -> Arc<Histogram> {
        let histogram = Arc::new(Histogram::new(name, labels.clone()));
        self.histograms.write().insert(name.to_string(), histogram.clone());
        histogram
    }
    
    pub fn get_all_metrics(&self) -> MetricsSnapshot {
        let mut snapshot = MetricsSnapshot::default();
        
        for (name, counter) in self.counters.read().iter() {
            snapshot.counters.push(MetricEntry {
                name: name.clone(),
                value: counter.get(),
                labels: counter.labels.clone(),
            });
        }
        
        for (name, gauge) in self.gauges.read().iter() {
            snapshot.gauges.push(MetricEntry {
                name: name.clone(),
                value: gauge.get(),
                labels: gauge.labels.clone(),
            });
        }
        
        for (name, histogram) in self.histograms.read().iter() {
            snapshot.histograms.push(HistogramEntry {
                name: name.clone(),
                stats: histogram.get_stats(),
                labels: histogram.labels.clone(),
            });
        }
        
        snapshot
    }
    
    pub fn export_prometheus(&self) -> String {
        let mut output = String::new();
        let snapshot = self.get_all_metrics();
        
        for counter in &snapshot.counters {
            let labels = format_labels(&counter.labels);
            output.push_str(&format!("# TYPE {} counter\n", counter.name));
            output.push_str(&format!("{}{{{}}} {}\n", counter.name, labels, counter.value));
        }
        
        for gauge in &snapshot.gauges {
            let labels = format_labels(&gauge.labels);
            output.push_str(&format!("# TYPE {} gauge\n", gauge.name));
            output.push_str(&format!("{}{{{}}} {}\n", gauge.name, labels, gauge.value));
        }
        
        output
    }
}

fn format_labels(labels: &HashMap<String, String>) -> String {
    labels
        .iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, v))
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MetricsSnapshot {
    pub counters: Vec<MetricEntry>,
    pub gauges: Vec<MetricEntry>,
    pub histograms: Vec<HistogramEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetricEntry {
    pub name: String,
    pub value: u64,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistogramEntry {
    pub name: String,
    pub stats: HistogramStats,
    pub labels: HashMap<String, String>,
}
