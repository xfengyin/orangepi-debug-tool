import React, { useEffect, memo } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Grid,
  Button,
  Switch,
  Chip,
} from '@mui/material';
import { useGpioStore, useAppStore } from '../../stores';

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
  const { pins, isLoading, refreshPins, readPin, writePin, togglePin } = useGpioStore();
  const { addToast } = useAppStore();

  useEffect(() => {
    refreshPins();
  }, []);

  const handleToggle = async (pin: number) => {
    const newValue = await togglePin(pin);
    if (newValue !== null) {
      addToast(`GPIO ${pin} 设置为 ${newValue}`, 'info');
    }
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
        GPIO 控制
      </Typography>

      <Grid container spacing={2}>
        {pinDefinitions.map((pinDef) => (
          <Grid item xs={12} sm={6} md={4} key={pinDef.pin}>
            <Card
              sx={{
                backgroundColor: '#1a1a1a',
                border: '1px solid #2a2a2a',
                borderRadius: 2,
                transition: 'border-color 0.2s ease, box-shadow 0.2s ease',
                '&:hover': {
                  borderColor: '#3a3a3a',
                  boxShadow: '0 0 12px rgba(124, 58, 237, 0.06)',
                },
              }}
            >
              <CardContent sx={{ py: 2, '&:last-child': { pb: 2 } }}>
                <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                  <Box>
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                      <Typography
                        variant="h6"
                        sx={{
                          fontSize: '0.95rem',
                          fontWeight: 600,
                          color: '#e4e4e7',
                        }}
                      >
                        GPIO {pinDef.pin}
                      </Typography>
                      <Box
                        component="span"
                        sx={{
                          px: 0.75,
                          py: 0.25,
                          borderRadius: 0.75,
                          backgroundColor: '#0a0a0a',
                          border: '1px solid #2a2a2a',
                          fontFamily: '"JetBrains Mono", monospace',
                          fontSize: '0.65rem',
                          color: '#71717a',
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
                            fontFamily: '"JetBrains Mono", monospace',
                            backgroundColor: 'rgba(124, 58, 237, 0.1)',
                            color: '#a78bfa',
                            border: '1px solid rgba(124, 58, 237, 0.2)',
                          }}
                        />
                      ))}
                    </Box>
                  </Box>
                  <Switch
                    onChange={() => handleToggle(pinDef.pin)}
                    disabled={isLoading}
                    sx={{
                      '& .MuiSwitch-switchBase.Mui-checked': {
                        color: '#7c3aed',
                      },
                      '& .MuiSwitch-switchBase.Mui-checked + .MuiSwitch-track': {
                        backgroundColor: '#7c3aed',
                      },
                      '& .MuiSwitch-track': {
                        backgroundColor: '#2a2a2a',
                      },
                    }}
                  />
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
