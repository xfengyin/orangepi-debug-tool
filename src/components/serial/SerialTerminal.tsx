import React, { useRef, useEffect, memo } from 'react';
import { Box, Paper } from '@mui/material';
import { useSerialStore, useLogStore } from '../../stores';
import { monoFontFamily } from '../../theme';
import { useUiTokens } from '../../theme/useUiTokens';
import EmptyState from '../common/EmptyState';

/** 字节数组转可视文本：可打印字符原样输出，其余转 \\xNN */
const formatPacketData = (data: number[]): string =>
  data
    .map((b) => (b >= 32 && b < 127 ? String.fromCharCode(b) : `\\x${b.toString(16).padStart(2, '0')}`))
    .join('');

const SerialTerminal: React.FC = memo(() => {
  const t = useUiTokens();
  const { dataBuffer, status } = useSerialStore();
  const { isAutoScroll } = useLogStore();
  const terminalRef = useRef<HTMLDivElement>(null);

  // Auto scroll to bottom
  useEffect(() => {
    if (isAutoScroll && terminalRef.current) {
      terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
    }
  }, [dataBuffer, isAutoScroll]);

  return (
    <Paper
      ref={terminalRef}
      elevation={0}
      sx={{
        height: '100%',
        p: 2,
        overflow: 'auto',
        fontFamily: monoFontFamily,
        fontSize: '0.8rem',
        lineHeight: 1.7,
        letterSpacing: '0.01em',
        backgroundColor: t.inset,
        color: t.text,
        whiteSpace: 'pre-wrap',
        wordBreak: 'break-all',
        border: `1px solid ${t.border}`,
        borderRadius: 0,
      }}
    >
      {!status.connected && dataBuffer.length === 0 ? (
        <EmptyState title="连接串口以开始通信" hint="waiting for connection..." />
      ) : (
        dataBuffer.map((packet, index) => (
          <Box
            key={index}
            sx={{
              color: packet.is_rx ? t.chart.rx : t.chart.tx,
              '&:hover': {
                backgroundColor: t.mode === 'dark' ? 'rgba(255,255,255,0.02)' : 'rgba(0,0,0,0.02)',
              },
              px: 0.5,
              borderRadius: 0.5,
            }}
          >
            <Box
              component="span"
              sx={{
                opacity: 0.55,
                mr: 1,
                fontSize: '0.7rem',
              }}
            >
              {packet.is_rx ? 'RX' : 'TX'}
            </Box>
            {formatPacketData(packet.data)}
          </Box>
        ))
      )}
    </Paper>
  );
});

SerialTerminal.displayName = 'SerialTerminal';

export default SerialTerminal;
