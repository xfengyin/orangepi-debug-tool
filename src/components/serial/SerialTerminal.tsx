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
        fontFamily: '"JetBrains Mono", "SF Mono", "Menlo", monospace',
        fontSize: '0.8rem',
        lineHeight: 1.7,
        letterSpacing: '0.01em',
        backgroundColor: '#0a0a0a',
        color: '#e4e4e7',
        whiteSpace: 'pre-wrap',
        wordBreak: 'break-all',
        border: '1px solid #2a2a2a',
        borderRadius: 1,
        position: 'relative',
        '&::before': {
          content: '""',
          position: 'absolute',
          top: 0,
          left: 0,
          right: 0,
          height: 1,
          background: 'linear-gradient(90deg, #7c3aed, #3b82f6)',
          opacity: 0.5,
        },
      }}
    >
      {!status.connected && dataBuffer.length === 0 ? (
        <Box
          sx={{
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            justifyContent: 'center',
            height: '100%',
            gap: 1,
          }}
        >
          <Box
            sx={{
              width: 40,
              height: 40,
              borderRadius: 2,
              border: '1px solid #2a2a2a',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#52525b',
              fontSize: '1.2rem',
              fontFamily: '"JetBrains Mono", monospace',
              mb: 1,
            }}
          >
            {'>'}_
          </Box>
          <Box
            sx={{
              color: '#52525b',
              fontSize: '0.8rem',
              fontFamily: '"Inter", sans-serif',
            }}
          >
            连接串口以开始通信
          </Box>
          <Box
            sx={{
              color: '#3f3f46',
              fontSize: '0.7rem',
              fontFamily: '"JetBrains Mono", monospace',
            }}
          >
            waiting for connection...
          </Box>
        </Box>
      ) : (
        dataBuffer.map((packet, index) => (
          <Box
            key={index}
            sx={{
              color: packet.is_rx ? '#4ade80' : '#a78bfa',
              '&:hover': {
                backgroundColor: 'rgba(255, 255, 255, 0.02)',
              },
              px: 0.5,
              borderRadius: 0.5,
            }}
          >
            <Box
              component="span"
              sx={{
                color: packet.is_rx ? 'rgba(74, 222, 128, 0.5)' : 'rgba(167, 139, 250, 0.5)',
                mr: 1,
                fontSize: '0.7rem',
              }}
            >
              {packet.is_rx ? 'RX' : 'TX'}
            </Box>
            {formatData(packet.data)}
          </Box>
        ))
      )}
    </Paper>
  );
});

SerialTerminal.displayName = 'SerialTerminal';

export default SerialTerminal;
