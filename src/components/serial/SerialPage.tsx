import React, { useState, useCallback, memo, useMemo } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Grid,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  TextField,
  Button,
  ButtonGroup,
  IconButton,
  Divider,
  Chip,
  Tooltip,
  ToggleButton,
  ToggleButtonGroup,
  Paper,
} from '@mui/material';
import {
  Usb as UsbIcon,
  UsbOff as UsbOffIcon,
  Send as SendIcon,
  Clear as ClearIcon,
  Save as SaveIcon,
  PlayArrow as PlayIcon,
  Hexagon as HexIcon,
  TextFields as TextIcon,
  Refresh as RefreshIcon,
  AutoFixHigh as AutoIcon,
} from '@mui/icons-material';
import { useSerialStore, useLogStore, useAppStore } from '../../stores';
import SerialTerminal from './SerialTerminal';
import SerialChart from './SerialChart';
import CommandPanel from './CommandPanel';

const baudRates = [300, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600];
const dataBits = [5, 6, 7, 8];
const parities = ['none', 'even', 'odd'];
const stopBits = [1, 2];

const SerialPage: React.FC = memo(() => {
  const {
    ports,
    selectedPort,
    config,
    status,
    isConnecting,
    setSelectedPort,
    setConfig,
    refreshPorts,
    autoDetect,
    connect,
    disconnect,
    clearData,
  } = useSerialStore();
  const { addLog, addToast } = useAppStore();
  const [inputMode, setInputMode] = useState<'text' | 'hex'>('text');
  const [inputText, setInputText] = useState('');
  const [showChart, setShowChart] = useState(false);

  // Handle port selection
  const handlePortChange = useCallback((port: string) => {
    setSelectedPort(port);
    setConfig({ port_name: port });
  }, [setSelectedPort, setConfig]);

  // Handle connect/disconnect
  const handleToggleConnect = useCallback(async () => {
    if (status.connected) {
      const success = await disconnect();
      if (success) {
        addToast('串口已断开', 'info');
        addLog('info', 'Serial', 'Disconnected from serial port');
      }
    } else {
      const success = await connect();
      if (success) {
        addToast(`已连接到 ${config.port_name}`, 'success');
        addLog('info', 'Serial', `Connected to ${config.port_name} @ ${config.baud_rate}`);
      } else {
        addToast('连接失败', 'error');
      }
    }
  }, [status.connected, connect, disconnect, addToast, addLog, config]);

  // Handle send
  const handleSend = useCallback(async () => {
    if (!inputText.trim() || !status.connected) return;

    let data: string | number[];
    if (inputMode === 'hex') {
      const hexStr = inputText.replace(/\s/g, '');
      data = hexStr.match(/.{1,2}/g)?.map((b) => parseInt(b, 16)) || [];
    } else {
      data = inputText;
    }

    const success = await useSerialStore.getState().write(data);
    if (success) {
      setInputText('');
      addLog('debug', 'Serial TX', inputText);
    }
  }, [inputText, inputMode, status.connected, addLog]);

  // Handle auto detect
  const handleAutoDetect = useCallback(async () => {
    addToast('正在自动检测...', 'info');
    const port = await autoDetect();
    if (port) {
      addToast(`检测到设备: ${port}`, 'success');
    } else {
      addToast('未检测到设备', 'warning');
    }
  }, [autoDetect, addToast]);

  // Stats display
  const statsDisplay = useMemo(() => [
    { label: 'RX', value: status.rx_bytes, color: '#4ade80' },
    { label: 'TX', value: status.tx_bytes, color: '#a78bfa' },
  ], [status.rx_bytes, status.tx_bytes]);

  return (
    <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column', gap: 2 }}>
      {/* Connection Panel - Cursor Dark Card */}
      <Card sx={{ backgroundColor: '#1a1a1a', border: '1px solid #2a2a2a' }}>
        <CardContent sx={{ pb: '12px !important' }}>
          <Grid container spacing={2} alignItems="center">
            {/* Port Selection */}
            <Grid item xs={12} sm={6} md={3}>
              <FormControl fullWidth size="small">
                <InputLabel sx={{ color: '#71717a', '&.Mui-focused': { color: '#a78bfa' } }}>串口</InputLabel>
                <Select
                  value={selectedPort || ''}
                  label="串口"
                  onChange={(e) => handlePortChange(e.target.value)}
                  disabled={status.connected}
                  sx={{
                    backgroundColor: '#0a0a0a',
                    '& .MuiSelect-icon': { color: '#71717a' },
                  }}
                >
                  {ports.map((port) => (
                    <MenuItem key={port.port_name} value={port.port_name}>
                      <Typography sx={{ fontFamily: '"JetBrains Mono", monospace', fontSize: '0.8rem' }}>
                        {port.port_name}
                      </Typography>
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            {/* Baud Rate */}
            <Grid item xs={6} sm={3} md={2}>
              <FormControl fullWidth size="small">
                <InputLabel sx={{ color: '#71717a', '&.Mui-focused': { color: '#a78bfa' } }}>波特率</InputLabel>
                <Select
                  value={config.baud_rate}
                  label="波特率"
                  onChange={(e) => setConfig({ baud_rate: Number(e.target.value) })}
                  disabled={status.connected}
                  sx={{
                    backgroundColor: '#0a0a0a',
                    '& .MuiSelect-icon': { color: '#71717a' },
                  }}
                >
                  {baudRates.map((rate) => (
                    <MenuItem key={rate} value={rate}>
                      <Typography sx={{ fontFamily: '"JetBrains Mono", monospace', fontSize: '0.8rem' }}>
                        {rate}
                      </Typography>
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            {/* Data Bits */}
            <Grid item xs={6} sm={3} md={1}>
              <FormControl fullWidth size="small">
                <InputLabel sx={{ color: '#71717a', '&.Mui-focused': { color: '#a78bfa' } }}>数据位</InputLabel>
                <Select
                  value={config.data_bits}
                  label="数据位"
                  onChange={(e) => setConfig({ data_bits: Number(e.target.value) })}
                  disabled={status.connected}
                  sx={{
                    backgroundColor: '#0a0a0a',
                    '& .MuiSelect-icon': { color: '#71717a' },
                  }}
                >
                  {dataBits.map((bits) => (
                    <MenuItem key={bits} value={bits}>
                      <Typography sx={{ fontFamily: '"JetBrains Mono", monospace', fontSize: '0.8rem' }}>
                        {bits}
                      </Typography>
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            {/* Parity */}
            <Grid item xs={6} sm={3} md={1}>
              <FormControl fullWidth size="small">
                <InputLabel sx={{ color: '#71717a', '&.Mui-focused': { color: '#a78bfa' } }}>校验</InputLabel>
                <Select
                  value={config.parity}
                  label="校验"
                  onChange={(e) => setConfig({ parity: e.target.value as any })}
                  disabled={status.connected}
                  sx={{
                    backgroundColor: '#0a0a0a',
                    '& .MuiSelect-icon': { color: '#71717a' },
                  }}
                >
                  {parities.map((p) => (
                    <MenuItem key={p} value={p}>
                      {p === 'none' ? '无' : p === 'even' ? '偶' : '奇'}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            {/* Stop Bits */}
            <Grid item xs={6} sm={3} md={1}>
              <FormControl fullWidth size="small">
                <InputLabel sx={{ color: '#71717a', '&.Mui-focused': { color: '#a78bfa' } }}>停止位</InputLabel>
                <Select
                  value={config.stop_bits}
                  label="停止位"
                  onChange={(e) => setConfig({ stop_bits: Number(e.target.value) })}
                  disabled={status.connected}
                  sx={{
                    backgroundColor: '#0a0a0a',
                    '& .MuiSelect-icon': { color: '#71717a' },
                  }}
                >
                  {stopBits.map((bits) => (
                    <MenuItem key={bits} value={bits}>
                      <Typography sx={{ fontFamily: '"JetBrains Mono", monospace', fontSize: '0.8rem' }}>
                        {bits}
                      </Typography>
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            {/* Action Buttons */}
            <Grid item xs={12} sm={6} md={4}>
              <Box sx={{ display: 'flex', gap: 1 }}>
                <Tooltip title="刷新串口列表" arrow>
                  <IconButton
                    onClick={refreshPorts}
                    size="small"
                    sx={{
                      color: '#71717a',
                      border: '1px solid #2a2a2a',
                      borderRadius: 1.5,
                      '&:hover': { color: '#e4e4e7', borderColor: '#3a3a3a', backgroundColor: 'rgba(255,255,255,0.04)' },
                    }}
                  >
                    <RefreshIcon fontSize="small" />
                  </IconButton>
                </Tooltip>
                <Tooltip title="自动检测" arrow>
                  <IconButton
                    onClick={handleAutoDetect}
                    size="small"
                    sx={{
                      color: '#71717a',
                      border: '1px solid #2a2a2a',
                      borderRadius: 1.5,
                      '&:hover': { color: '#a78bfa', borderColor: '#7c3aed', backgroundColor: 'rgba(124,58,237,0.05)' },
                    }}
                  >
                    <AutoIcon fontSize="small" />
                  </IconButton>
                </Tooltip>
                <Button
                  variant={status.connected ? 'outlined' : 'contained'}
                  color={status.connected ? 'error' : 'primary'}
                  onClick={handleToggleConnect}
                  disabled={isConnecting || !selectedPort}
                  startIcon={status.connected ? <UsbOffIcon /> : <UsbIcon />}
                  fullWidth
                  sx={{
                    ...(status.connected
                      ? {
                          borderColor: 'rgba(239, 68, 68, 0.4)',
                          color: '#f87171',
                          '&:hover': {
                            borderColor: '#ef4444',
                            backgroundColor: 'rgba(239, 68, 68, 0.05)',
                          },
                        }
                      : {}),
                  }}
                >
                  {isConnecting ? '连接中...' : status.connected ? '断开' : '连接'}
                </Button>
              </Box>
            </Grid>
          </Grid>

          {/* Stats - Cursor-style mono labels */}
          <Box sx={{ mt: 2, display: 'flex', gap: 2, alignItems: 'center' }}>
            {statsDisplay.map((stat) => (
              <Box
                key={stat.label}
                sx={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: 0.75,
                  px: 1.5,
                  py: 0.5,
                  borderRadius: 1,
                  backgroundColor: '#0a0a0a',
                  border: '1px solid #2a2a2a',
                }}
              >
                <Typography
                  sx={{
                    fontSize: '0.65rem',
                    fontFamily: '"JetBrains Mono", monospace',
                    color: stat.color,
                    fontWeight: 600,
                    letterSpacing: '0.05em',
                  }}
                >
                  {stat.label}
                </Typography>
                <Typography
                  sx={{
                    fontSize: '0.75rem',
                    fontFamily: '"JetBrains Mono", monospace',
                    color: '#e4e4e7',
                  }}
                >
                  {stat.value} B
                </Typography>
              </Box>
            ))}
            <Box sx={{ flexGrow: 1 }} />
            <Button
              size="small"
              onClick={() => setShowChart(!showChart)}
              sx={{
                color: '#71717a',
                fontSize: '0.75rem',
                fontFamily: '"JetBrains Mono", monospace',
                '&:hover': { color: '#a78bfa', backgroundColor: 'rgba(124, 58, 237, 0.05)' },
              }}
            >
              {showChart ? '隐藏图表' : '显示图表'}
            </Button>
          </Box>
        </CardContent>
      </Card>

      {/* Chart Panel */}
      {showChart && (
        <Card sx={{ backgroundColor: '#1a1a1a', border: '1px solid #2a2a2a' }}>
          <CardContent>
            <SerialChart />
          </CardContent>
        </Card>
      )}

      {/* Terminal & Command Panel */}
      <Grid container spacing={2} sx={{ flexGrow: 1, minHeight: 0 }}>
        <Grid item xs={12} md={8} sx={{ height: '100%' }}>
          <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column', backgroundColor: '#1a1a1a', border: '1px solid #2a2a2a' }}>
            <CardContent sx={{ flexGrow: 1, p: 0, overflow: 'hidden' }}>
              <SerialTerminal />
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={4} sx={{ height: '100%' }}>
          <CommandPanel />
        </Grid>
      </Grid>

      {/* Input Panel - Cursor-style dark input */}
      <Card sx={{ backgroundColor: '#1a1a1a', border: '1px solid #2a2a2a' }}>
        <CardContent>
          <Grid container spacing={2} alignItems="center">
            <Grid item xs={12} md={9}>
              <TextField
                fullWidth
                size="small"
                placeholder={inputMode === 'hex' ? '输入十六进制 (如: 01 02 03)' : '输入要发送的数据...'}
                value={inputText}
                onChange={(e) => setInputText(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSend()}
                disabled={!status.connected}
                InputProps={{
                  startAdornment: (
                    <ToggleButtonGroup
                      size="small"
                      value={inputMode}
                      exclusive
                      onChange={(_, v) => v && setInputMode(v)}
                      sx={{
                        mr: 1,
                        '& .MuiToggleButton-root': {
                          border: '1px solid #2a2a2a',
                          color: '#52525b',
                          py: 0.25,
                          px: 1,
                          fontSize: '0.7rem',
                          '&.Mui-selected': {
                            backgroundColor: 'rgba(124, 58, 237, 0.12)',
                            color: '#a78bfa',
                            borderColor: '#7c3aed',
                          },
                        },
                      }}
                    >
                      <ToggleButton value="text">
                        <TextIcon sx={{ fontSize: '0.9rem' }} />
                      </ToggleButton>
                      <ToggleButton value="hex">
                        <HexIcon sx={{ fontSize: '0.9rem' }} />
                      </ToggleButton>
                    </ToggleButtonGroup>
                  ),
                  sx: {
                    fontFamily: inputMode === 'hex' ? '"JetBrains Mono", monospace' : '"Inter", sans-serif',
                    fontSize: '0.85rem',
                  },
                }}
              />
            </Grid>
            <Grid item xs={12} md={3}>
              <Box sx={{ display: 'flex', gap: 1 }}>
                <Button
                  variant="contained"
                  startIcon={<SendIcon />}
                  onClick={handleSend}
                  disabled={!status.connected || !inputText.trim()}
                  fullWidth
                >
                  发送
                </Button>
                <Tooltip title="清空" arrow>
                  <IconButton
                    onClick={clearData}
                    disabled={!status.connected}
                    sx={{
                      color: '#52525b',
                      '&:hover': { color: '#f87171', backgroundColor: 'rgba(239, 68, 68, 0.05)' },
                    }}
                  >
                    <ClearIcon />
                  </IconButton>
                </Tooltip>
              </Box>
            </Grid>
          </Grid>
        </CardContent>
      </Card>
    </Box>
  );
});

SerialPage.displayName = 'SerialPage';

export default SerialPage;
