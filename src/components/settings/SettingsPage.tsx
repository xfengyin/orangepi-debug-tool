import React, { useState } from 'react';
import {
  Box,
  Typography,
  Card,
  Stack,
  Switch,
  FormControlLabel,
  Button,
  Divider,
  TextField,
} from '@mui/material';
import { useAppStore, useThemeStore } from '../../stores';
import { useUiTokens } from '../../theme/useUiTokens';
import PageHeader from '../common/PageHeader';

const SettingsPage: React.FC = () => {
  const t = useUiTokens();
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
      <PageHeader title="设置" description="应用配置、主题与系统信息" />

      <Card sx={{ p: 2.5, mb: 2 }}>
        <Typography variant="subtitle2" sx={{ color: t.text, mb: 1.5 }}>外观</Typography>
        <Divider sx={{ mb: 1.5 }} />
        <FormControlLabel
          control={<Switch checked={mode === 'dark'} onChange={toggleMode} />}
          label="深色模式"
          sx={{ color: t.textSecondary }}
        />
      </Card>

      <Card sx={{ p: 2.5, mb: 2 }}>
        <Typography variant="subtitle2" sx={{ color: t.text, mb: 1.5 }}>应用配置</Typography>
        <Divider sx={{ mb: 2 }} />
        <Stack spacing={2}>
          <TextField
            label="自动保存间隔（秒）"
            type="number"
            value={config.auto_save_interval}
            onChange={(e) => updateConfig({ auto_save_interval: Number(e.target.value) })}
            size="small"
          />
          <TextField
            label="串口缓冲区大小"
            type="number"
            value={config.serial_buffer_size}
            onChange={(e) => updateConfig({ serial_buffer_size: Number(e.target.value) })}
            size="small"
          />
          <TextField
            label="最大日志条数"
            type="number"
            value={config.max_log_entries}
            onChange={(e) => updateConfig({ max_log_entries: Number(e.target.value) })}
            size="small"
          />
          <FormControlLabel
            control={
              <Switch
                checked={config.hardware_acceleration}
                onChange={(e) => updateConfig({ hardware_acceleration: e.target.checked })}
              />
            }
            label="硬件加速"
            sx={{ color: t.textSecondary }}
          />
        </Stack>
        <Stack direction="row" spacing={1} sx={{ mt: 2 }}>
          <Button variant="contained" onClick={handleSave} disabled={saving}>
            保存配置
          </Button>
          <Button variant="outlined" onClick={() => loadConfig()}>
            重新加载
          </Button>
        </Stack>
      </Card>

      <Card sx={{ p: 2.5 }}>
        <Typography variant="subtitle2" sx={{ color: t.text, mb: 1 }}>系统信息</Typography>
        <Divider sx={{ mb: 1.5 }} />
        <Typography sx={{ color: t.textMuted, fontSize: '0.85rem' }}>
          平台：{systemInfo?.platform || '--'} · 架构：{systemInfo?.arch || '--'} · 版本：{systemInfo?.version || '--'} · OrangePi：{isOrangePi ? '是' : '否'}
        </Typography>
      </Card>
    </Box>
  );
};

export default SettingsPage;
