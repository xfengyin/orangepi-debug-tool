use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};
use chrono::Local;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::sync::Mutex;

pub struct LogService {
    log_dir: PathBuf,
    current_file: RwLock<Option<Arc<Mutex<BufWriter<File>>>>>,
    rotation_size: usize,
    max_files: usize,
    current_size: RwLock<usize>,
}

impl LogService {
    pub fn new(log_dir: PathBuf) -> Self {
        Self {
            log_dir,
            current_file: RwLock::new(None),
            rotation_size: 10 * 1024 * 1024,
            max_files: 10,
            current_size: RwLock::new(0),
        }
    }

    pub async fn write(&self, data: &str) -> std::io::Result<()> {
        let mut file_guard = self.current_file.write().await;

        if file_guard.is_none() {
            let file = self.open_new_file().await?;
            *file_guard = Some(file);
        }

        if let Some(ref mut file) = *file_guard {
            let mut file = file.lock().unwrap();
            file.write_all(data.as_bytes()).await?;
            file.flush().await?;

            *self.current_size.write().await += data.len();
        }

        Ok(())
    }

    pub async fn write_line(&self, data: &str) -> std::io::Result<()> {
        self.write(&format!("{}\n", data)).await
    }

    pub async fn write_with_timestamp(&self, data: &str) -> std::io::Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let line = format!("[{}] {}", timestamp, data);
        self.write_line(&line).await
    }

    async fn open_new_file(&self) -> std::io::Result<Arc<Mutex<BufWriter<File>>>> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("debug_{}.log", timestamp);
        let path = self.log_dir.join(&filename);

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .await?;

        *self.current_size.write().await = 0;

        Ok(Arc::new(Mutex::new(BufWriter::new(file))))
    }

    pub async fn rotate_if_needed(&self) -> std::io::Result<()> {
        let size = *self.current_size.read().await;
        if size >= self.rotation_size {
            self.rotate().await?;
        }
        Ok(())
    }

    async fn rotate(&self) -> std::io::Result<()> {
        *self.current_file.write().await = None;
        let new_file = self.open_new_file().await?;
        *self.current_file.write().await = Some(new_file);
        Ok(())
    }

    pub fn set_rotation_size(&mut self, size: usize) {
        self.rotation_size = size;
    }

    pub fn set_max_files(&mut self, max: usize) {
        self.max_files = max;
    }
}
