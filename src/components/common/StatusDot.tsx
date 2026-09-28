import React, { memo } from 'react';
import { Box } from '@mui/material';
import { useUiTokens } from '../../theme/useUiTokens';

export type StatusDotState = 'connected' | 'disconnected' | 'pending';

interface StatusDotProps {
  state: StatusDotState;
  size?: number;
}

/** 状态指示点：连接=绿 / 等待=琥珀脉冲 / 断开=灰 */
const StatusDot: React.FC<StatusDotProps> = memo(({ state, size = 6 }) => {
  const t = useUiTokens();
  const color =
    state === 'connected' ? t.success.main : state === 'pending' ? t.warning.main : t.textMuted;
  return (
    <Box
      component="span"
      sx={{
        width: size,
        height: size,
        borderRadius: '50%',
        flexShrink: 0,
        backgroundColor: color,
        boxShadow: state === 'connected' ? `0 0 6px ${color}80` : 'none',
        ...(state === 'pending' && {
          '@keyframes statusDotPulse': {
            '0%, 100%': { opacity: 1 },
            '50%': { opacity: 0.35 },
          },
          animation: 'statusDotPulse 1.5s ease-in-out infinite',
        }),
      }}
    />
  );
});

StatusDot.displayName = 'StatusDot';

export default StatusDot;
