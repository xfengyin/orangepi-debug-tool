import React, { useEffect, memo } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Grid,
  Switch,
  Chip,
} from '@mui/material';
import { useGpioStore, useAppStore } from '../../stores';
import { monoFontFamily } from '../../theme';
import { useUiTokens } from '../../theme/useUiTokens';
import PageHeader from '../common/PageHeader';

const pinDefinitions = [
  { pin: 3, name: 'PA12/SCL', modes: ['I2C', 'GPIO'] },
  { pin: 5, name: 'PA11/SDA', modes: ['I2C', 'GPIO'] },
  { pin: 7, name: 'PA6', modes: ['GPIO'] },
  { pin: 11, name: 'PA1', modes: ['GPIO'] },
  { pin: 12, name: 'PA7', modes: ['GPIO'] },
  { pin: 13, name: 'PA0', modes: ['GPIO'] },
  { pin: 15, name: 'PA3', modes: ['GPIO'] },
  { pin: 16, name: 'PA15', modes: ['GPIO'] },
];

const GpioPage: React.FC = memo(() => {
  const t = useUiTokens();
  const { isLoading, refreshPins, togglePin } = useGpioStore();
  const { addToast } = useAppStore();

  useEffect(() => {
    refreshPins();
    // 挂载时拉取一次引脚状态；refreshPins 为 store 动作，故意不加入依赖数组
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleToggle = async (pin: number) => {
    const newValue = await togglePin(pin);
    if (newValue !== null) {
      addToast(`GPIO ${pin} 设置为 ${newValue}`, 'info');
    }
  };

  return (
    <Box>
      <PageHeader title="GPIO 控制" />

      <Grid container spacing={2}>
        {pinDefinitions.map((pinDef) => (
          <Grid item xs={12} sm={6} md={4} key={pinDef.pin}>
            <Card
              sx={{
                transition: 'border-color 0.2s ease, box-shadow 0.2s ease',
                '&:hover': { borderColor: t.borderStrong },
              }}
            >
              <CardContent sx={{ py: 2, '&:last-child': { pb: 2 } }}>
                <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                  <Box>
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                      <Typography
                        variant="h6"
                        sx={{ fontSize: '0.95rem', fontWeight: 600, color: t.text }}
                      >
                        GPIO {pinDef.pin}
                      </Typography>
                      <Box
                        component="span"
                        sx={{
                          px: 0.75,
                          py: 0.25,
                          borderRadius: 0.75,
                          backgroundColor: t.inset,
                          border: `1px solid ${t.border}`,
                          fontFamily: monoFontFamily,
                          fontSize: '0.65rem',
                          color: t.textMuted,
                        }}
                      >
                        {pinDef.name}
                      </Box>
                    </Box>
                    <Box sx={{ mt: 1, display: 'flex', gap: 0.5 }}>
                      {pinDef.modes.map((mode) => (
                        <Chip
                          key={mode}
                          label={mode}
                          size="small"
                          sx={{
                            height: 20,
                            fontSize: '0.65rem',
                            fontFamily: monoFontFamily,
                            backgroundColor: t.brandSoft,
                            color: t.brand,
                            border: `1px solid ${t.brandSoft}`,
                          }}
                        />
                      ))}
                    </Box>
                  </Box>
                  <Switch onChange={() => handleToggle(pinDef.pin)} disabled={isLoading} />
                </Box>
              </CardContent>
            </Card>
          </Grid>
        ))}
      </Grid>
    </Box>
  );
});

GpioPage.displayName = 'GpioPage';

export default GpioPage;
