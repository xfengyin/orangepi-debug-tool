import React, { useCallback, useMemo, memo } from 'react';
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
  Button,
  IconButton,
  Tooltip,
} from '@mui/material';
import {
  Usb as UsbIcon,
  UsbOff as UsbOffIcon,
  Refresh as RefreshIcon,
  AutoFixHigh as AutoIcon,
} from '@mui/icons-material';
import { useSerialStore, useLogStore, useAppStore } from '../../stores';
import { monoFontFamily } from '../../theme';
import { useUiTokens } from '../../theme/useUiTokens';
import { BAUD_RATES, DATA_BITS, PARITIES, STOP_BITS } from './constants';

interface LabeledSelectProps<T extends string | number> {
  label: string;
  value: T;
  options: readonly T[];
  onChange: (value: T) => void;
  disabled?: boolean;
  format?: (value: T) => string;
  mono?: boolean;
}

/** 串口参数下拉：标签/样式/等宽字体统一的通用选择器 */
function LabeledSelect<T extends string | number>({
  label,
  value,
  options,
  onChange,
  disabled,
  format,
  mono = true,
}: LabeledSelectProps<T>) {
  return (
    <FormControl fullWidth size="small">
      <InputLabel>{label}</InputLabel>
      <Select
        value={value}
        label={label}
        onChange={(e) => onChange(e.target.value as T)}
        disabled={disabled}
      >
        {options.map((opt) => (
          <MenuItem key={opt} value={opt}>
            <Typography sx={mono ? { fontFamily: monoFontFamily, fontSize: '0.8rem' } : undefined}>
              {format ? format(opt) : String(opt)}
            </Typography>
          </MenuItem>
        ))}
      </Select>
    </FormControl>
  );
}

const parityLabel = (p: string) => (p === 'none' ? '无' : p === 'even' ? '偶' : '奇');

interface ConnectionPanelProps {
  showChart: boolean;
  onToggleChart: () => void;
}

/** 串口连接面板：端口参数选择、连接/断开、统计与图表开关 */
const ConnectionPanel: React.FC<ConnectionPanelProps> = memo(({ showChart, onToggleChart }) => {
  const t = useUiTokens();
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
  } = useSerialStore();
  const { addToast } = useAppStore();
  const { addLog } = useLogStore();

  const handlePortChange = useCallback(
    (port: string) => {
      setSelectedPort(port);
      setConfig({ port_name: port });
    },
    [setSelectedPort, setConfig]
  );

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

  const handleAutoDetect = useCallback(async () => {
    addToast('正在自动检测...', 'info');
    const port = await autoDetect();
    if (port) {
      addToast(`检测到设备: ${port}`, 'success');
    } else {
      addToast('未检测到设备', 'warning');
    }
  }, [autoDetect, addToast]);

  const statsDisplay = useMemo(
    () => [
      { label: 'RX', value: status.rx_bytes, color: t.success.main },
      { label: 'TX', value: status.tx_bytes, color: t.brand },
    ],
    [status.rx_bytes, status.tx_bytes, t]
  );

  return (
    <Card>
      <CardContent sx={{ pb: '12px !important' }}>
        <Grid container spacing={2} alignItems="center">
          <Grid item xs={12} sm={6} md={3}>
            <LabeledSelect
              label="串口"
              value={selectedPort || ''}
              options={ports.map((p) => p.port_name)}
              onChange={handlePortChange}
              disabled={status.connected}
            />
          </Grid>
          <Grid item xs={6} sm={3} md={2}>
            <LabeledSelect
              label="波特率"
              value={config.baud_rate}
              options={BAUD_RATES}
              onChange={(v) => setConfig({ baud_rate: Number(v) })}
              disabled={status.connected}
            />
          </Grid>
          <Grid item xs={6} sm={3} md={1}>
            <LabeledSelect
              label="数据位"
              value={config.data_bits}
              options={DATA_BITS}
              onChange={(v) => setConfig({ data_bits: Number(v) })}
              disabled={status.connected}
            />
          </Grid>
          <Grid item xs={6} sm={3} md={1}>
            <LabeledSelect
              label="校验"
              value={config.parity}
              options={PARITIES}
              onChange={(v) => setConfig({ parity: v as 'none' | 'even' | 'odd' })}
              disabled={status.connected}
              format={parityLabel}
              mono={false}
            />
          </Grid>
          <Grid item xs={6} sm={3} md={1}>
            <LabeledSelect
              label="停止位"
              value={config.stop_bits}
              options={STOP_BITS}
              onChange={(v) => setConfig({ stop_bits: Number(v) })}
              disabled={status.connected}
            />
          </Grid>

          {/* Action Buttons */}
          <Grid item xs={12} sm={6} md={4}>
            <Box sx={{ display: 'flex', gap: 1 }}>
              <Tooltip title="刷新串口列表" arrow>
                <IconButton
                  onClick={refreshPorts}
                  size="small"
                  sx={{ border: `1px solid ${t.border}` }}
                >
                  <RefreshIcon fontSize="small" />
                </IconButton>
              </Tooltip>
              <Tooltip title="自动检测" arrow>
                <IconButton
                  onClick={handleAutoDetect}
                  size="small"
                  sx={{ border: `1px solid ${t.border}` }}
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
              >
                {isConnecting ? '连接中...' : status.connected ? '断开' : '连接'}
              </Button>
            </Box>
          </Grid>
        </Grid>

        {/* Stats */}
        <Box sx={{ mt: 2, display: 'flex', gap: 1, alignItems: 'center' }}>
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
                backgroundColor: t.inset,
                border: `1px solid ${t.border}`,
              }}
            >
              <Typography
                sx={{
                  fontSize: '0.65rem',
                  fontFamily: monoFontFamily,
                  color: stat.color,
                  fontWeight: 600,
                  letterSpacing: '0.05em',
                }}
              >
                {stat.label}
              </Typography>
              <Typography sx={{ fontSize: '0.75rem', fontFamily: monoFontFamily, color: t.text }}>
                {stat.value} B
              </Typography>
            </Box>
          ))}
          <Box sx={{ flexGrow: 1 }} />
          <Button
            size="small"
            onClick={onToggleChart}
            sx={{ fontFamily: monoFontFamily, fontSize: '0.75rem' }}
          >
            {showChart ? '隐藏图表' : '显示图表'}
          </Button>
        </Box>
      </CardContent>
    </Card>
  );
});

ConnectionPanel.displayName = 'ConnectionPanel';

export default ConnectionPanel;
