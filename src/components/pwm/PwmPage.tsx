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
    await setFrequency(0, 0, frequency);
    await setDutyCycle(0, 0, dutyCycle);
    await setEnabled(0, 0, enabled);
    addToast('PWM 配置已应用', 'success');
  };

  return (
    <Box>
      <Typography
        variant="h5"
        sx={{
          fontWeight: 600,
          color: '#ffffff',
          mb: 3,
          letterSpacing: '-0.01em',
        }}
      >
        PWM 输出
      </Typography>

      <Grid container spacing={3}>
        <Grid item xs={12} md={6}>
          <Card sx={{ backgroundColor: '#1a1a1a', border: '1px solid #2a2a2a' }}>
            <CardContent>
              <Typography
                variant="h6"
                sx={{
                  color: '#e4e4e7',
                  fontWeight: 600,
                  fontSize: '0.95rem',
                  mb: 1,
                }}
              >
                PWM Channel 0
              </Typography>
              <Typography
                sx={{
                  fontSize: '0.75rem',
                  fontFamily: '"JetBrains Mono", monospace',
                  color: '#52525b',
                  mb: 2,
                }}
              >
                chip 0 / channel 0
              </Typography>

              <Box sx={{ mt: 3 }}>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                  <Typography sx={{ color: '#a1a1aa', fontSize: '0.85rem' }}>频率</Typography>
                  <Typography
                    sx={{
                      color: '#a78bfa',
                      fontFamily: '"JetBrains Mono", monospace',
                      fontSize: '0.85rem',
                    }}
                  >
                    {frequency} Hz
                  </Typography>
                </Box>
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
                  sx={{
                    color: '#7c3aed',
                    '& .MuiSlider-markLabel': {
                      color: '#52525b',
                      fontSize: '0.65rem',
                      fontFamily: '"JetBrains Mono", monospace',
                    },
                    '& .MuiSlider-track': {
                      background: 'linear-gradient(90deg, #7c3aed, #3b82f6)',
                    },
                  }}
                />
              </Box>

              <Box sx={{ mt: 4 }}>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                  <Typography sx={{ color: '#a1a1aa', fontSize: '0.85rem' }}>占空比</Typography>
                  <Typography
                    sx={{
                      color: '#a78bfa',
                      fontFamily: '"JetBrains Mono", monospace',
                      fontSize: '0.85rem',
                    }}
                  >
                    {dutyCycle}%
                  </Typography>
                </Box>
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
                  sx={{
                    color: '#7c3aed',
                    '& .MuiSlider-markLabel': {
                      color: '#52525b',
                      fontSize: '0.65rem',
                      fontFamily: '"JetBrains Mono", monospace',
                    },
                    '& .MuiSlider-track': {
                      background: 'linear-gradient(90deg, #7c3aed, #3b82f6)',
                    },
                  }}
                />
              </Box>

              <Box sx={{ mt: 3, display: 'flex', alignItems: 'center', gap: 2 }}>
                <Typography sx={{ color: '#a1a1aa', fontSize: '0.85rem' }}>启用</Typography>
                <Switch
                  checked={enabled}
                  onChange={(e) => setEnabledState(e.target.checked)}
                  sx={{
                    '& .MuiSwitch-switchBase.Mui-checked': { color: '#7c3aed' },
                    '& .MuiSwitch-switchBase.Mui-checked + .MuiSwitch-track': { backgroundColor: '#7c3aed' },
                    '& .MuiSwitch-track': { backgroundColor: '#2a2a2a' },
                  }}
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
          <Card sx={{ backgroundColor: '#1a1a1a', border: '1px solid #2a2a2a' }}>
            <CardContent>
              <Typography
                variant="h6"
                sx={{
                  color: '#e4e4e7',
                  fontWeight: 600,
                  fontSize: '0.95rem',
                  mb: 2,
                }}
              >
                PWM 通道状态
              </Typography>
              <Box sx={{ mt: 2 }}>
                {channels.length === 0 ? (
                  <Box
                    sx={{
                      py: 4,
                      display: 'flex',
                      flexDirection: 'column',
                      alignItems: 'center',
                      gap: 1,
                    }}
                  >
                    <Typography
                      sx={{
                        color: '#52525b',
                        fontSize: '0.8rem',
                        fontFamily: '"JetBrains Mono", monospace',
                      }}
                    >
                      No PWM channels configured
                    </Typography>
                  </Box>
                ) : (
                  channels.map((ch) => (
                    <Box
                      key={ch.channel}
                      sx={{
                        mb: 2,
                        p: 2,
                        bgcolor: '#0a0a0a',
                        borderRadius: 1,
                        border: '1px solid #2a2a2a',
                      }}
                    >
                      <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 1 }}>
                        <Typography
                          variant="subtitle2"
                          sx={{
                            color: '#e4e4e7',
                            fontWeight: 600,
                            fontFamily: '"JetBrains Mono", monospace',
                          }}
                        >
                          CH{ch.channel} / Chip {ch.chip}
                        </Typography>
                        <Box
                          component="span"
                          sx={{
                            px: 1,
                            py: 0.25,
                            borderRadius: 0.75,
                            fontSize: '0.6rem',
                            fontFamily: '"JetBrains Mono", monospace',
                            fontWeight: 600,
                            letterSpacing: '0.05em',
                            backgroundColor: ch.enabled ? 'rgba(74, 222, 128, 0.1)' : 'rgba(113, 113, 122, 0.1)',
                            color: ch.enabled ? '#4ade80' : '#71717a',
                            border: '1px solid',
                            borderColor: ch.enabled ? 'rgba(74, 222, 128, 0.2)' : 'rgba(113, 113, 122, 0.15)',
                          }}
                        >
                          {ch.enabled ? 'ON' : 'OFF'}
                        </Box>
                      </Box>
                      <Box sx={{ display: 'flex', gap: 3 }}>
                        <Box>
                          <Typography sx={{ color: '#52525b', fontSize: '0.7rem', fontFamily: '"JetBrains Mono", monospace' }}>
                            FREQ
                          </Typography>
                          <Typography sx={{ color: '#a78bfa', fontSize: '0.85rem', fontFamily: '"JetBrains Mono", monospace' }}>
                            {ch.frequency.toFixed(2)} Hz
                          </Typography>
                        </Box>
                        <Box>
                          <Typography sx={{ color: '#52525b', fontSize: '0.7rem', fontFamily: '"JetBrains Mono", monospace' }}>
                            DUTY
                          </Typography>
                          <Typography sx={{ color: '#60a5fa', fontSize: '0.85rem', fontFamily: '"JetBrains Mono", monospace' }}>
                            {ch.duty_cycle.toFixed(2)}%
                          </Typography>
                        </Box>
                      </Box>
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
