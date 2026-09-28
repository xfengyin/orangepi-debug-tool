import React, { memo } from 'react';
import { Box } from '@mui/material';
import { monoFontFamily } from '../../theme';
import { useUiTokens } from '../../theme/useUiTokens';
import { useAppStore, useSerialStore } from '../../stores';
import StatusDot from '../common/StatusDot';

/** 底部状态栏：连接状态 / 端口信息 / 数据统计 / 设备与版本 */
const StatusBar: React.FC = memo(() => {
  const t = useUiTokens();
  const { systemInfo } = useAppStore();
  const { status, config } = useSerialStore();

  return (
    <Box
      component="footer"
      sx={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        px: 2,
        py: 0.5,
        minHeight: 26,
        backgroundColor: t.surface,
        borderTop: `1px solid ${t.border}`,
        fontSize: '0.7rem',
        fontFamily: monoFontFamily,
        color: t.textMuted,
        flexShrink: 0,
      }}
    >
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.75 }}>
          <StatusDot state={status.connected ? 'connected' : 'disconnected'} />
          <span>{status.connected ? 'Connected' : 'Disconnected'}</span>
        </Box>
        {status.connected && config.port_name && (
          <span>
            {config.port_name} @ {config.baud_rate} bps
          </span>
        )}
        {status.connected && (
          <span>
            RX: {status.rx_bytes} | TX: {status.tx_bytes}
          </span>
        )}
      </Box>
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
        {systemInfo?.hostname && <span>{systemInfo.hostname}</span>}
        <span>OrangePi Debug Tool v{systemInfo?.version || '2.0.0'}</span>
      </Box>
    </Box>
  );
});

StatusBar.displayName = 'StatusBar';

export default StatusBar;
