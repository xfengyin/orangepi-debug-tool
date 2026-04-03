import React, { useRef, useEffect, memo } from 'react';
import { Box, Paper } from '@mui/material';
import { useSerialStore, useAppStore } from '../../stores';

const SerialTerminal: React.FC = memo(() => {
  const { dataBuffer, status } = useSerialStore();
  const { isAutoScroll } = useAppStore();
  const terminalRef = useRef<HTMLDivElement>(null);

  // Auto scroll to bottom
  useEffect(() => {
    if (isAutoScroll && terminalRef.current) {
      terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
    }
  }, [dataBuffer, isAutoScroll]);

  // Format data for display
  const formatData = (data: number[]): string => {
    return data
      .map((b) => {
        if (b >= 32 && b < 127) {
          return String.fromCharCode(b);
        }
        return `\\x${b.toString(16).padStart(2, '0')}`;
      })
      .join('');
  };

  return (
    <Paper
      ref={terminalRef}
      elevation={0}
      sx={{
        height: '100%',
        p: 2,
        overflow: 'auto',
        fontFamily: '"JetBrains Mono", monospace',
        fontSize: '0.875rem',
        lineHeight: 1.6,
        backgroundColor: (theme) =>
          theme.palette.mode === 'dark' ? '#1e1e1e' : '#f8f9fa',
        color: (theme) =>
          theme.palette.mode === 'dark' ? '#d4d4d4' : '#1e293b',
        whiteSpace: 'pre-wrap',
        wordBreak: 'break-all',
      }}
    >
      {!status.connected && dataBuffer.length === 0 ? (
        <Box
          sx={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            height: '100%',
            color: 'text.secondary',
          }}
        >
          连接串口以开始通信
        </Box>
      ) : (
        dataBuffer.map((packet, index) => (
          <Box
            key={index}
            sx={{
              color: packet.is_rx
                ? (theme) => theme.palette.success.main
                : (theme) => theme.palette.primary.main,
            }}
          >
            {packet.is_rx ? '<< ' : '>> '}
            {formatData(packet.data)}
          </Box>
        ))
      )}
    </Paper>
  );
});

SerialTerminal.displayName = 'SerialTerminal';

export default SerialTerminal;