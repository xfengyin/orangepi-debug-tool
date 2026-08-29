import React from 'react';
import {
  Box,
  Typography,
  Paper,
  Grid,
  Card,
  CardActionArea,
  CardContent,
  Stack,
  Divider,
} from '@mui/material';
import {
  Usb as UsbIcon,
  SettingsInputComponent as GpioIcon,
  Waves as PwmIcon,
  Article as LogIcon,
  Settings as SettingsIcon,
  Memory as CpuIcon,
} from '@mui/icons-material';
import { useAppStore, useSerialStore, useGpioStore, usePwmStore, useLogStore } from '../../stores';
import type { ViewType } from '../../types';

const OverviewPage: React.FC = () => {
  const { setCurrentView, systemInfo, isOrangePi } = useAppStore();
  const serial = useSerialStore();
  const gpio = useGpioStore();
  const pwm = usePwmStore();
  const log = useLogStore();

  const cards: { id: ViewType; title: string; desc: string; icon: React.ReactNode; accent: string }[] = [
    { id: 'serial', title: '串口调试', desc: `${serial.ports.length} 个端口 · ${serial.status.connected ? '已连接' : '未连接'}`, icon: <UsbIcon />, accent: '#3b82f6' },
    { id: 'gpio', title: 'GPIO 控制', desc: `${gpio.pins.length} 个引脚`, icon: <GpioIcon />, accent: '#7c3aed' },
    { id: 'pwm', title: 'PWM 输出', desc: `${pwm.channels.length} 个通道`, icon: <PwmIcon />, accent: '#f59e0b' },
    { id: 'log', title: '数据日志', desc: `${log.entries.length} 条日志`, icon: <LogIcon />, accent: '#4ade80' },
    { id: 'settings', title: '设置', desc: '主题 / 配置 / 系统信息', icon: <SettingsIcon />, accent: '#a1a1aa' },
  ];

  return (
    <Box>
      <Typography variant="h5" sx={{ mb: 0.5, color: '#ffffff' }}>
        OrangePi Debug Tool
      </Typography>
      <Typography sx={{ color: '#71717a', mb: 3, fontSize: '0.9rem' }}>
        {isOrangePi ? '已检测到 OrangePi 设备' : '未检测到 OrangePi 设备'} · v{systemInfo?.version || '2.0.0'}
      </Typography>

      <Grid container spacing={2}>
        {cards.map((card) => (
          <Grid item xs={12} sm={6} md={4} key={card.id}>
            <Card
              sx={{
                backgroundColor: '#141414',
                border: '1px solid #2a2a2a',
                borderRadius: 2,
                '&:hover': {
                  borderColor: card.accent,
                  boxShadow: `0 0 16px ${card.accent}33`,
                },
              }}
            >
              <CardActionArea onClick={() => setCurrentView(card.id)}>
                <CardContent sx={{ p: 2.5 }}>
                  <Box sx={{ color: card.accent, fontSize: 28, mb: 1 }}>{card.icon}</Box>
                  <Typography variant="subtitle1" sx={{ color: '#ffffff', fontWeight: 600 }}>
                    {card.title}
                  </Typography>
                  <Typography variant="body2" sx={{ color: '#71717a', mt: 0.5, fontSize: '0.8rem' }}>
                    {card.desc}
                  </Typography>
                </CardContent>
              </CardActionArea>
            </Card>
          </Grid>
        ))}
      </Grid>

      <Paper
        sx={{
          mt: 3,
          p: 2,
          backgroundColor: '#101010',
          border: '1px solid #2a2a2a',
          borderRadius: 2,
        }}
      >
        <Stack direction="row" alignItems="center" spacing={1} sx={{ mb: 1 }}>
          <CpuIcon sx={{ color: '#a78bfa', fontSize: 18 }} />
          <Typography variant="subtitle2" sx={{ color: '#e4e4e7' }}>系统信息</Typography>
        </Stack>
        <Divider sx={{ borderColor: '#2a2a2a', mb: 1.5 }} />
        <Grid container spacing={2}>
          <Grid item xs={4}><Stat label="平台" value={systemInfo?.platform || '--'} /></Grid>
          <Grid item xs={4}><Stat label="架构" value={systemInfo?.arch || '--'} /></Grid>
          <Grid item xs={4}><Stat label="版本" value={systemInfo?.version || '--'} /></Grid>
        </Grid>
      </Paper>
    </Box>
  );
};

function Stat({ label, value }: { label: string; value: string }) {
  return (
    <Box>
      <Typography sx={{ color: '#71717a', fontSize: '0.75rem', fontFamily: 'monospace' }}>{label}</Typography>
      <Typography sx={{ color: '#e4e4e7', fontSize: '0.95rem', fontWeight: 600 }}>{value}</Typography>
    </Box>
  );
}

export default OverviewPage;
