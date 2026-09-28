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
} from '@mui/material';
import { usePwmStore, useAppStore } from '../../stores';
import { monoFontFamily } from '../../theme';
import { useUiTokens } from '../../theme/useUiTokens';
import PageHeader from '../common/PageHeader';
import EmptyState from '../common/EmptyState';

const PwmPage: React.FC = memo(() => {
  const t = useUiTokens();
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
      <PageHeader title="PWM 输出" />

      <Grid container spacing={3}>
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h6" sx={{ color: t.text, mb: 1 }}>
                PWM Channel 0
              </Typography>
              <Typography
                sx={{ fontSize: '0.75rem', fontFamily: monoFontFamily, color: t.textMuted, mb: 2 }}
              >
                chip 0 / channel 0
              </Typography>

              <Box sx={{ mt: 3 }}>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                  <Typography sx={{ color: t.textSecondary, fontSize: '0.85rem' }}>频率</Typography>
                  <Typography sx={{ color: t.brand, fontFamily: monoFontFamily, fontSize: '0.85rem' }}>
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
                />
              </Box>

              <Box sx={{ mt: 4 }}>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                  <Typography sx={{ color: t.textSecondary, fontSize: '0.85rem' }}>占空比</Typography>
                  <Typography sx={{ color: t.brand, fontFamily: monoFontFamily, fontSize: '0.85rem' }}>
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
                />
              </Box>

              <Box sx={{ mt: 3, display: 'flex', alignItems: 'center', gap: 2 }}>
                <Typography sx={{ color: t.textSecondary, fontSize: '0.85rem' }}>启用</Typography>
                <Switch checked={enabled} onChange={(e) => setEnabledState(e.target.checked)} />
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
              <Typography variant="h6" sx={{ color: t.text, mb: 2 }}>
                PWM 通道状态
              </Typography>
              <Box sx={{ mt: 2 }}>
                {channels.length === 0 ? (
                  <EmptyState title="No PWM channels configured" height={160} icon={null} />
                ) : (
                  channels.map((ch) => (
                    <Box
                      key={ch.channel}
                      sx={{
                        mb: 2,
                        p: 2,
                        bgcolor: t.inset,
                        borderRadius: 1,
                        border: `1px solid ${t.border}`,
                      }}
                    >
                      <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 1 }}>
                        <Typography
                          variant="subtitle2"
                          sx={{ color: t.text, fontFamily: monoFontFamily }}
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
                            fontFamily: monoFontFamily,
                            fontWeight: 600,
                            letterSpacing: '0.05em',
                            backgroundColor: ch.enabled ? t.success.soft : t.surfaceAlt,
                            color: ch.enabled ? t.success.main : t.textMuted,
                            border: `1px solid ${ch.enabled ? t.success.soft : t.border}`,
                          }}
                        >
                          {ch.enabled ? 'ON' : 'OFF'}
                        </Box>
                      </Box>
                      <Box sx={{ display: 'flex', gap: 3 }}>
                        <Box>
                          <Typography sx={{ color: t.textMuted, fontSize: '0.7rem', fontFamily: monoFontFamily }}>
                            FREQ
                          </Typography>
                          <Typography sx={{ color: t.brand, fontSize: '0.85rem', fontFamily: monoFontFamily }}>
                            {ch.frequency.toFixed(2)} Hz
                          </Typography>
                        </Box>
                        <Box>
                          <Typography sx={{ color: t.textMuted, fontSize: '0.7rem', fontFamily: monoFontFamily }}>
                            DUTY
                          </Typography>
                          <Typography sx={{ color: t.info.main, fontSize: '0.85rem', fontFamily: monoFontFamily }}>
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
