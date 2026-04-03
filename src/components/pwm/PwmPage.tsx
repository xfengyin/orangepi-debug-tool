import React, { useState, memo } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Grid,
  Slider,
  Button,
  Switch,
  TextField,
} from '@mui/material';
import { usePwmStore, useAppStore } from '../../stores';

const PwmPage: React.FC = memo(() => {
  const { channels, setFrequency, setDutyCycle, setEnabled, isLoading } = usePwmStore();
  const { addToast } = useAppStore();
  const [frequency, setFreqValue] = useState(1000);
  const [dutyCycle, setDutyValue] = useState(50);
  const [enabled, setEnabledState] = useState(false);

  const handleApply = async () => {
    // Configure PWM channel 0
    await setFrequency(0, 0, frequency);
    await setDutyCycle(0, 0, dutyCycle);
    await setEnabled(0, 0, enabled);
    addToast('PWM 配置已应用', 'success');
  };

  return (
    <Box>
      <Typography variant="h5" gutterBottom>
        PWM 输出
      </Typography>

      <Grid container spacing={3}>
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                PWM Channel 0
              </Typography>

              <Box sx={{ mt: 3 }}>
                <Typography gutterBottom>频率: {frequency} Hz</Typography>
                <Slider
                  value={frequency}
                  onChange={(_, v) => setFreqValue(v as number)}
                  min={1}
                  max={10000}
                  step={10}
                  marks={[
                    { value: 1, label: '1Hz' },
                    { value: 5000, label: '5kHz' },
                    { value: 10000, label: '10kHz' },
                  ]}
                />
              </Box>

              <Box sx={{ mt: 3 }}>
                <Typography gutterBottom>占空比: {dutyCycle}%</Typography>
                <Slider
                  value={dutyCycle}
                  onChange={(_, v) => setDutyValue(v as number)}
                  min={0}
                  max={100}
                  step={0.1}
                  marks={[
                    { value: 0, label: '0%' },
                    { value: 50, label: '50%' },
                    { value: 100, label: '100%' },
                  ]}
                />
              </Box>

              <Box sx={{ mt: 3, display: 'flex', alignItems: 'center', gap: 2 }}>
                <Typography>启用</Typography>
                <Switch
                  checked={enabled}
                  onChange={(e) => setEnabledState(e.target.checked)}
                />
              </Box>

              <Button
                variant="contained"
                fullWidth
                sx={{ mt: 3 }}
                onClick={handleApply}
                disabled={isLoading}
              >
                应用配置
              </Button>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                PWM 通道状态
              </Typography>
              <Box sx={{ mt: 2 }}>
                {channels.length === 0 ? (
                  <Typography color="text.secondary">未配置PWM通道</Typography>
                ) : (
                  channels.map((ch) => (
                    <Box key={ch.channel} sx={{ mb: 2, p: 2, bgcolor: 'background.default', borderRadius: 1 }}>
                      <Typography variant="subtitle2">
                        Channel {ch.channel} (Chip {ch.chip})
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        频率: {ch.frequency.toFixed(2)} Hz
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        占空比: {ch.duty_cycle.toFixed(2)}%
                      </Typography>
                      <Typography variant="body2" color={ch.enabled ? 'success.main' : 'text.secondary'}>
                        状态: {ch.enabled ? '启用' : '禁用'}
                      </Typography>
                    </Box>
                  ))
                )}
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  );
});

PwmPage.displayName = 'PwmPage';

export default PwmPage;