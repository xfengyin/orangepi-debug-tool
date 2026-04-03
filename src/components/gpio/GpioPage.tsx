import React, { useEffect, memo } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Grid,
  Button,
  Switch,
  FormControlLabel,
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
      <Typography variant="h5" gutterBottom>
        GPIO 控制
      </Typography>

      <Grid container spacing={2}>
        {pinDefinitions.map((pinDef) => (
          <Grid item xs={12} sm={6} md={4} key={pinDef.pin}>
            <Card>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                  <Box>
                    <Typography variant="h6">GPIO {pinDef.pin}</Typography>
                    <Typography variant="caption" color="text.secondary">
                      {pinDef.name}
                    </Typography>
                    <Box sx={{ mt: 1 }}>
                      {pinDef.modes.map((mode) => (
                        <Chip key={mode} label={mode} size="small" sx={{ mr: 0.5 }} />
                      ))}
                    </Box>
                  </Box>
                  <Switch
                    onChange={() => handleToggle(pinDef.pin)}
                    disabled={isLoading}
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