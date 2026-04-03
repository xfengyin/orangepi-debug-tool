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
      // Parse hex string
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
    { label: '接收', value: status.rx_bytes, color: 'success' },
    { label: '发送', value: status.tx_bytes, color: 'primary' },
  ], [status.rx_bytes, status.tx_bytes]);

  return (
    <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column', gap: 2 }}>
      {/* Connection Panel */}
      <Card>
        <CardContent>
          <Grid container spacing={2} alignItems="center">
            {/* Port Selection */}
            <Grid item xs={12} sm={6} md={3}>
              <FormControl fullWidth size="small">
                <InputLabel>串口</InputLabel>
                <Select
                  value={selectedPort || ''}
                  label="串口"
                  onChange={(e) => handlePortChange(e.target.value)}
                  disabled={status.connected}
                >
                  {ports.map((port) => (
                    <MenuItem key={port.port_name} value={port.port_name}>
                      {port.port_name}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            {/* Baud Rate */}
            <Grid item xs={6} sm={3} md={2}>
              <FormControl fullWidth size="small">
                <InputLabel>波特率</InputLabel>
                <Select
                  value={config.baud_rate}
                  label="波特率"
                  onChange={(e) => setConfig({ baud_rate: Number(e.target.value) })}
                  disabled={status.connected}
                >
                  {baudRates.map((rate) => (
                    <MenuItem key={rate} value={rate}>
                      {rate}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            {/* Data Bits */}
            <Grid item xs={6} sm={3} md={1}>
              <FormControl fullWidth size="small">
                <InputLabel>数据位</InputLabel>
                <Select
                  value={config.data_bits}
                  label="数据位"
                  onChange={(e) => setConfig({ data_bits: Number(e.target.value) })}
                  disabled={status.connected}
                >
                  {dataBits.map((bits) => (
                    <MenuItem key={bits} value={bits}>
                      {bits}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            {/* Parity */}
            <Grid item xs={6} sm={3} md={1}>
              <FormControl fullWidth size="small">
                <InputLabel>校验</InputLabel>
                <Select
                  value={config.parity}
                  label="校验"
                  onChange={(e) => setConfig({ parity: e.target.value as any })}
                  disabled={status.connected}
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
                <InputLabel>停止位</InputLabel>
                <Select
                  value={config.stop_bits}
                  label="停止位"
                  onChange={(e) => setConfig({ stop_bits: Number(e.target.value) })}
                  disabled={status.connected}
                >
                  {stopBits.map((bits) => (
                    <MenuItem key={bits} value={bits}>
                      {bits}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            {/* Action Buttons */}
            <Grid item xs={12} sm={6} md={4}>
              <Box sx={{ display: 'flex', gap: 1 }}>
                <Tooltip title="刷新串口列表">
                  <IconButton onClick={refreshPorts} size="small">
                    <RefreshIcon />
                  </IconButton>
                </Tooltip>
                <Tooltip title="自动检测">
                  <IconButton onClick={handleAutoDetect} size="small">
                    <AutoIcon />
                  </IconButton>
                </Tooltip>
                <Button
                  variant={status.connected ? 'outlined' : 'contained'}
                  color={status.connected ? 'error' : 'primary'}
                  onClick={handleToggleConnect}
                  disabled={isConnecting || !selectedPort}
                  startIcon={status.connected ? <UsbOffIcon /> : <UsbIcon />}
                  fullWidth
                >
                  {isConnecting ? '连接中...' : status.connected ? '断开' : '连接'}
                </Button>
              </Box>
            </Grid>
          </Grid>

          {/* Stats */}
          <Box sx={{ mt: 2, display: 'flex', gap: 2 }}>
            {statsDisplay.map((stat) => (
              <Chip
                key={stat.label}
                size="small"
                label={`${stat.label}: ${stat.value} bytes`}
                color={stat.color as any}
                variant="outlined"
              />
            ))}
            <Box sx={{ flexGrow: 1 }} />
            <Button size="small" onClick={() => setShowChart(!showChart)}>
              {showChart ? '隐藏图表' : '显示图表'}
            </Button>
          </Box>
        </CardContent>
      </Card>

      {/* Chart Panel */}
      {showChart && (
        <Card>
          <CardContent>
            <SerialChart />
          </CardContent>
        </Card>
      )}

      {/* Terminal & Command Panel */}
      <Grid container spacing={2} sx={{ flexGrow: 1, minHeight: 0 }}>
        <Grid item xs={12} md={8} sx={{ height: '100%' }}>
          <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
            <CardContent sx={{ flexGrow: 1, p: 0, overflow: 'hidden' }}>
              <SerialTerminal />
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={4} sx={{ height: '100%' }}>
          <CommandPanel />
        </Grid>
      </Grid>

      {/* Input Panel */}
      <Card>
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
                      sx={{ mr: 1 }}
                    >
                      <ToggleButton value="text">
                        <TextIcon fontSize="small" />
                      </ToggleButton>
                      <ToggleButton value="hex">
                        <HexIcon fontSize="small" />
                      </ToggleButton>
                    </ToggleButtonGroup>
                  ),
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
                <Tooltip title="清空">
                  <IconButton onClick={clearData} disabled={!status.connected}>
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