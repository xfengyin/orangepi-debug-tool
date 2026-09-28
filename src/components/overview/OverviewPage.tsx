import React from 'react';
import {
  Box,
  Typography,
  Card,
  Grid,
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
import { monoFontFamily } from '../../theme';
import { useUiTokens } from '../../theme/useUiTokens';

const OverviewPage: React.FC = () => {
  const t = useUiTokens();
  const { setCurrentView, systemInfo, isOrangePi } = useAppStore();
  const serial = useSerialStore();
  const gpio = useGpioStore();
  const pwm = usePwmStore();
  const log = useLogStore();

  const cards: { id: ViewType; title: string; desc: string; icon: React.ReactNode }[] = [
    { id: 'serial', title: '串口调试', desc: `${serial.ports.length} 个端口 · ${serial.status.connected ? '已连接' : '未连接'}`, icon: <UsbIcon /> },
    { id: 'gpio', title: 'GPIO 控制', desc: `${gpio.pins.length} 个引脚`, icon: <GpioIcon /> },
    { id: 'pwm', title: 'PWM 输出', desc: `${pwm.channels.length} 个通道`, icon: <PwmIcon /> },
    { id: 'log', title: '数据日志', desc: `${log.entries.length} 条日志`, icon: <LogIcon /> },
    { id: 'settings', title: '设置', desc: '主题 / 配置 / 系统信息', icon: <SettingsIcon /> },
  ];

  return (
    <Box>
      <Typography variant="h5" sx={{ mb: 0.5, color: t.text }}>
        OrangePi Debug Tool
      </Typography>
      <Typography sx={{ color: t.textMuted, mb: 3, fontSize: '0.9rem' }}>
        {isOrangePi ? '已检测到 OrangePi 设备' : '未检测到 OrangePi 设备'} · v{systemInfo?.version || '2.0.0'}
      </Typography>

      <Grid container spacing={2}>
        {cards.map((card) => (
          <Grid item xs={12} sm={6} md={4} key={card.id}>
            <Card
              sx={{
                transition: 'border-color 0.2s ease, box-shadow 0.2s ease',
                '&:hover': { borderColor: t.brand },
              }}
            >
              <CardActionArea onClick={() => setCurrentView(card.id)}>
                <CardContent sx={{ p: 2.5 }}>
                  <Box sx={{ color: t.textMuted, fontSize: 26, mb: 1 }}>{card.icon}</Box>
                  <Typography variant="subtitle1" sx={{ color: t.text }}>
                    {card.title}
                  </Typography>
                  <Typography variant="body2" sx={{ color: t.textMuted, mt: 0.5 }}>
                    {card.desc}
                  </Typography>
                </CardContent>
              </CardActionArea>
            </Card>
          </Grid>
        ))}
      </Grid>

      <Card sx={{ mt: 3, p: 2 }}>
        <Stack direction="row" alignItems="center" spacing={1} sx={{ mb: 1 }}>
          <CpuIcon sx={{ color: t.brand, fontSize: 18 }} />
          <Typography variant="subtitle2" sx={{ color: t.text }}>系统信息</Typography>
        </Stack>
        <Divider sx={{ mb: 1.5 }} />
        <Grid container spacing={2}>
          <Grid item xs={4}><Stat label="平台" value={systemInfo?.platform || '--'} /></Grid>
          <Grid item xs={4}><Stat label="架构" value={systemInfo?.arch || '--'} /></Grid>
          <Grid item xs={4}><Stat label="版本" value={systemInfo?.version || '--'} /></Grid>
        </Grid>
      </Card>
    </Box>
  );
};

function Stat({ label, value }: { label: string; value: string }) {
  const t = useUiTokens();
  return (
    <Box>
      <Typography sx={{ color: t.textMuted, fontSize: '0.75rem', fontFamily: monoFontFamily }}>{label}</Typography>
      <Typography sx={{ color: t.text, fontSize: '0.95rem', fontWeight: 600 }}>{value}</Typography>
    </Box>
  );
}

export default OverviewPage;
