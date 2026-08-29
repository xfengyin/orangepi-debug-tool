import React, { useState } from 'react';
import {
  Box,
  Typography,
  Paper,
  Stack,
  Switch,
  FormControlLabel,
  Button,
  Divider,
  TextField,
} from '@mui/material';
import { useAppStore, useThemeStore } from '../../stores';

const SettingsPage: React.FC = () => {
  const { config, updateConfig, saveConfig, loadConfig, systemInfo, isOrangePi } = useAppStore();
  const { mode, toggleMode } = useThemeStore();
  const [saving, setSaving] = useState(false);

  const handleSave = async () => {
    setSaving(true);
    try {
      await saveConfig();
    } finally {
      setSaving(false);
    }
  };

  return (
    <Box>
      <Typography variant="h5" sx={{ mb: 0.5, color: '#ffffff' }}>
        设置
      </Typography>
      <Typography sx={{ color: '#71717a', mb: 3, fontSize: '0.9rem' }}>
        应用配置、主题与系统信息
      </Typography>

      <Paper sx={{ p: 2.5, backgroundColor: '#141414', border: '1px solid #2a2a2a', borderRadius: 2, mb: 2 }}>
        <Typography variant="subtitle2" sx={{ color: '#e4e4e7', mb: 1.5 }}>外观</Typography>
        <Divider sx={{ borderColor: '#2a2a2a', mb: 1.5 }} />
        <FormControlLabel
          control={<Switch checked={mode === 'dark'} onChange={toggleMode} color="secondary" />}
          label="深色模式"
          sx={{ color: '#a1a1aa' }}
        />
      </Paper>

      <Paper sx={{ p: 2.5, backgroundColor: '#141414', border: '1px solid #2a2a2a', borderRadius: 2, mb: 2 }}>
        <Typography variant="subtitle2" sx={{ color: '#e4e4e7', mb: 1.5 }}>应用配置</Typography>
        <Divider sx={{ borderColor: '#2a2a2a', mb: 2 }} />
        <Stack spacing={2}>
          <TextField
            label="自动保存间隔（秒）"
            type="number"
            value={config.auto_save_interval}
            onChange={(e) => updateConfig({ auto_save_interval: Number(e.target.value) })}
            size="small"
            sx={{ input: { color: '#e4e4e7' }, label: { color: '#71717a' }, '& .MuiOutlinedInput-root': { '& fieldset': { borderColor: '#2a2a2a' } } }}
          />
          <TextField
            label="串口缓冲区大小"
            type="number"
            value={config.serial_buffer_size}
            onChange={(e) => updateConfig({ serial_buffer_size: Number(e.target.value) })}
            size="small"
            sx={{ input: { color: '#e4e4e7' }, label: { color: '#71717a' }, '& .MuiOutlinedInput-root': { '& fieldset': { borderColor: '#2a2a2a' } } }}
          />
          <TextField
            label="最大日志条数"
            type="number"
            value={config.max_log_entries}
            onChange={(e) => updateConfig({ max_log_entries: Number(e.target.value) })}
            size="small"
            sx={{ input: { color: '#e4e4e7' }, label: { color: '#71717a' }, '& .MuiOutlinedInput-root': { '& fieldset': { borderColor: '#2a2a2a' } } }}
          />
          <FormControlLabel
            control={<Switch checked={config.hardware_acceleration} onChange={(e) => updateConfig({ hardware_acceleration: e.target.checked })} color="secondary" />}
            label="硬件加速"
            sx={{ color: '#a1a1aa' }}
          />
        </Stack>
        <Stack direction="row" spacing={1} sx={{ mt: 2 }}>
          <Button variant="contained" color="secondary" onClick={handleSave} disabled={saving}>
            保存配置
          </Button>
          <Button variant="outlined" color="inherit" onClick={() => loadConfig()}>
            重新加载
          </Button>
        </Stack>
      </Paper>

      <Paper sx={{ p: 2.5, backgroundColor: '#141414', border: '1px solid #2a2a2a', borderRadius: 2 }}>
        <Typography variant="subtitle2" sx={{ color: '#e4e4e7', mb: 1 }}>系统信息</Typography>
        <Divider sx={{ borderColor: '#2a2a2a', mb: 1.5 }} />
        <Typography sx={{ color: '#71717a', fontSize: '0.85rem' }}>
          平台：{systemInfo?.platform || '--'} · 架构：{systemInfo?.arch || '--'} · 版本：{systemInfo?.version || '--'} · OrangePi：{isOrangePi ? '是' : '否'}
        </Typography>
      </Paper>
    </Box>
  );
};

export default SettingsPage;
