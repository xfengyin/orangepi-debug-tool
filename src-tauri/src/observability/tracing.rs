use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct TraceId(String);

impl TraceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string().replace("-", ""))
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.len() == 32 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            Some(Self(hex.to_string()))
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for TraceId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TraceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct SpanId(String);

impl SpanId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string().replace("-", "")[..16].to_string())
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.len() == 16 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            Some(Self(hex.to_string()))
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SpanId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SpanId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanStatus {
    Ok,
    Error,
    Unset,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub parent_span_id: Option<SpanId>,
    pub name: String,
    pub service_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: SpanStatus,
    pub tags: HashMap<String, String>,
    pub logs: Vec<SpanLog>,
}

impl Span {
    pub fn new(name: impl Into<String>, service_name: impl Into<String>) -> Self {
        Self {
            trace_id: TraceId::new(),
            span_id: SpanId::new(),
            parent_span_id: None,
            name: name.into(),
            service_name: service_name.into(),
            start_time: Utc::now(),
            end_time: None,
            status: SpanStatus::Unset,
            tags: HashMap::new(),
            logs: Vec::new(),
        }
    }

    pub fn with_parent(mut self, parent: &Span) -> Self {
        self.trace_id = parent.trace_id.clone();
        self.parent_span_id = Some(parent.span_id.clone());
        self
    }

    pub fn with_trace_id(mut self, trace_id: &TraceId) -> Self {
        self.trace_id = trace_id.clone();
        self
    }

    pub fn tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.insert(key.into(), value.into());
        self
    }

    pub fn set_tag(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.tags.insert(key.into(), value.into());
    }

    pub fn log(&mut self, message: impl Into<String>) {
        self.logs.push(SpanLog {
            timestamp: Utc::now(),
            fields: HashMap::from([("message".to_string(), message.into())]),
        });
    }

    pub fn log_with_fields(&mut self, fields: HashMap<String, String>) {
        self.logs.push(SpanLog {
            timestamp: Utc::now(),
            fields,
        });
    }

    pub fn set_status(&mut self, status: SpanStatus) {
        self.status = status;
    }

    pub fn finish(&mut self) {
        self.end_time = Some(Utc::now());
    }

    pub fn duration(&self) -> Option<Duration> {
        self.end_time.map(|end| end.signed_duration_since(self.start_time).to_std().unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
pub struct SpanLog {
    pub timestamp: DateTime<Utc>,
    pub fields: HashMap<String, String>,
}

#[derive(Debug)]
pub struct AppTracer {
    spans: RwLock<Vec<Span>>,
    max_spans: usize,
    enabled: RwLock<bool>,
}

impl AppTracer {
    pub fn new() -> Self {
        Self {
            spans: RwLock::new(Vec::new()),
            max_spans: 10000,
            enabled: RwLock::new(true),
        }
    }

    pub fn with_max_spans(mut self, max: usize) -> Self {
        self.max_spans = max;
        self
    }

    pub fn start_span(&self, name: impl Into<String>) -> Span {
        Span::new(name, "orangepi-debug-tool")
    }

    pub fn start_span_with_parent(&self, name: impl Into<String>, parent: &Span) -> Span {
        Span::new(name, "orangepi-debug-tool").with_parent(parent)
    }

    pub fn record_span(&self, span: Span) {
        if !*self.enabled.read() {
            return;
        }

        let mut spans = self.spans.write();
        
        if spans.len() >= self.max_spans {
            let half_len = spans.len() / 2;
            spans.drain(0..half_len);
        }
        
        spans.push(span);
    }

    pub fn get_spans(&self) -> Vec<Span> {
        self.spans.read().clone()
    }

    pub fn get_spans_by_trace(&self, trace_id: &TraceId) -> Vec<Span> {
        self.spans.read()
            .iter()
            .filter(|s| &s.trace_id == trace_id)
            .cloned()
            .collect()
    }

    pub fn clear(&self) {
        self.spans.write().clear();
    }

    pub fn enable(&self) {
        *self.enabled.write() = true;
    }

    pub fn disable(&self) {
        *self.enabled.write() = false;
    }

    pub fn is_enabled(&self) -> bool {
        *self.enabled.read()
    }

    pub fn get_trace_tree(&self, trace_id: &TraceId) -> Option<TraceTree> {
        let spans = self.get_spans_by_trace(trace_id);
        
        if spans.is_empty() {
            return None;
        }

        let root = spans.iter()
            .find(|s| s.parent_span_id.is_none())?
            .clone();

        Some(TraceTree {
            root,
            children: spans.into_iter().filter(|s| s.parent_span_id.is_some()).collect(),
        })
    }
}

impl Default for AppTracer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct TraceTree {
    pub root: Span,
    pub children: Vec<Span>,
}

#[derive(Debug, Clone)]
pub struct TraceContext {
    pub trace_id: TraceId,
    pub span_id: Option<SpanId>,
    pub sampled: bool,
}

impl TraceContext {
    pub fn new() -> Self {
        Self {
            trace_id: TraceId::new(),
            span_id: None,
            sampled: true,
        }
    }

    pub fn from_header(header: &str) -> Option<Self> {
        Self::from_hex(header)
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let parts: Vec<&str> = hex.split(':').collect();
        
        if parts.len() >= 2 {
            let trace_id = TraceId::from_hex(parts[0])?;
            let span_id = parts.get(1).and_then(|s| SpanId::from_hex(s));
            let sampled = parts.get(2).map(|s| *s == "1").unwrap_or(true);
            
            Some(Self {
                trace_id,
                span_id,
                sampled,
            })
        } else {
            None
        }
    }

    pub fn to_header(&self) -> String {
        let span_str = self.span_id.as_ref().map(|s| s.as_str()).unwrap_or("");
        let sampled_str = if self.sampled { "1" } else { "0" };
        format!("{}:{}:{}", self.trace_id, span_str, sampled_str)
    }
}

impl Default for TraceContext {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! trace {
    ($tracer:expr, $name:expr, $($key:expr => $value:expr),*) => {{
        let mut span = $tracer.start_span($name);
        $(
            span.set_tag($key, $value);
        )*
        span
    }};
}

#[macro_export]
macro_rules! trace_fn {
    ($tracer:expr, $name:expr) => {{
        let mut span = $tracer.start_span($name);
        let start = std::time::Instant::now();
        let result = async {}.await;
        span.log(format!("Duration: {:?}", start.elapsed()));
        span.finish();
        $tracer.record_span(span);
        result
    }};
}
