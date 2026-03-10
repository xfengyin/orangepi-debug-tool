//! 图形显示模块

use serde::{Deserialize, Serialize};

/// 数据点
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataPoint {
    pub timestamp: f64, // 时间戳 (秒)
    pub value: f64,     // 数值
    pub label: Option<String>,
}

/// 波形类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WaveformType {
    Line,
    Bar,
    Scatter,
    Step,
}

/// 图表配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChartConfig {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub waveform_type: WaveformType,
    pub show_grid: bool,
    pub show_legend: bool,
    pub max_points: usize,
    pub auto_scale: bool,
    pub y_min: Option<f64>,
    pub y_max: Option<f64>,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            title: "实时波形".to_string(),
            x_label: "时间 (s)".to_string(),
            y_label: "数值".to_string(),
            waveform_type: WaveformType::Line,
            show_grid: true,
            show_legend: true,
            max_points: 1000,
            auto_scale: true,
            y_min: None,
            y_max: None,
        }
    }
}

/// 波形数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WaveformData {
    pub name: String,
    pub color: String,
    pub points: Vec<DataPoint>,
}

/// 图形渲染器
pub struct GraphRenderer {
    pub configs: Vec<ChartConfig>,
    pub data: Vec<WaveformData>,
}

impl GraphRenderer {
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
            data: Vec::new(),
        }
    }
    
    /// 添加波形
    pub fn add_waveform(&mut self, name: String, color: String) {
        self.data.push(WaveformData {
            name,
            color,
            points: Vec::new(),
        });
    }
    
    /// 添加数据点
    pub fn add_data_point(&mut self, waveform_index: usize, timestamp: f64, value: f64) {
        if waveform_index >= self.data.len() {
            return;
        }
        
        let waveform = &mut self.data[waveform_index];
        waveform.points.push(DataPoint {
            timestamp,
            value,
            label: None,
        });
        
        // 限制数据点数量
        let max_points = self.configs.get(waveform_index)
            .map(|c| c.max_points)
            .unwrap_or(1000);
        
        if waveform.points.len() > max_points {
            waveform.points.remove(0);
        }
    }
    
    /// 清空数据
    pub fn clear(&mut self) {
        for waveform in &mut self.data {
            waveform.points.clear();
        }
    }
    
    /// 导出为 SVG
    pub fn export_svg(&self, width: u32, height: u32) -> String {
        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">
  <rect width="100%" height="100%" fill="white"/>"#,
            width, height
        );
        
        // 绘制网格
        if self.configs.first().map(|c| c.show_grid).unwrap_or(true) {
            svg.push_str(&self.draw_grid(width, height));
        }
        
        // 绘制波形
        for (i, waveform) in self.data.iter().enumerate() {
            svg.push_str(&self.draw_waveform(i, waveform, width, height));
        }
        
        svg.push_str("\n</svg>");
        svg
    }
    
    /// 绘制网格
    fn draw_grid(&self, width: u32, height: u32) -> String {
        let mut grid = String::new();
        
        // 横线
        for i in 0..=5 {
            let y = (i * height / 5) as f32;
            grid.push_str(&format!(
                r#"<line x1="0" y1="{}" x2="{}" y2="{}" stroke="#e5e7eb" stroke-width="1"/>"#,
                y, width, y
            ));
        }
        
        // 竖线
        for i in 0..=10 {
            let x = (i * width / 10) as f32;
            grid.push_str(&format!(
                r#"<line x1="{}" y1="0" x2="{}" y2="{}" stroke="#e5e7eb" stroke-width="1"/>"#,
                x, x, height
            ));
        }
        
        grid
    }
    
    /// 绘制波形
    fn draw_waveform(&self, index: usize, waveform: &WaveformData, width: u32, height: u32) -> String {
        if waveform.points.is_empty() {
            return String::new();
        }
        
        let mut path = format!(
            r#"<path d="M" fill="none" stroke="{}" stroke-width="2">"#,
            waveform.color
        );
        
        // 计算缩放
        let min_value = waveform.points.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
        let max_value = waveform.points.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max);
        let value_range = max_value - min_value;
        
        let time_range = waveform.points.last().unwrap().timestamp - waveform.points.first().unwrap().timestamp;
        
        for (i, point) in waveform.points.iter().enumerate() {
            let x = if time_range > 0.0 {
                ((point.timestamp - waveform.points.first().unwrap().timestamp) / time_range * (width - 40) as f64) as u32 + 20
            } else {
                (i * (width as usize / waveform.points.len())) as u32
            };
            
            let y = if value_range > 0.0 {
                ((1.0 - (point.value - min_value) / value_range) * (height - 40) as f64) as u32 + 20
            } else {
                height / 2
            };
            
            if i == 0 {
                path.push_str(&format!(" {} {}", x, y));
            } else {
                path.push_str(&format!(" L {} {}", x, y));
            }
        }
        
        path.push_str(r#""/></path>"#);
        path
    }
}

impl Default for GraphRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_data_point() {
        let mut renderer = GraphRenderer::new();
        renderer.add_waveform("Waveform 1".to_string(), "#6366f1".to_string());
        renderer.add_data_point(0, 0.0, 10.0);
        renderer.add_data_point(0, 1.0, 20.0);
        
        assert_eq!(renderer.data[0].points.len(), 2);
    }
}
