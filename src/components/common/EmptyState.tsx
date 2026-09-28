import React, { memo } from 'react';
import { Box, Typography } from '@mui/material';
import { monoFontFamily } from '../../theme';
import { useUiTokens } from '../../theme/useUiTokens';

interface EmptyStateProps {
  /** 主提示（必填） */
  title: string;
  /** 次级说明（等宽字体，可选） */
  hint?: string;
  /** 自定义图标节点；不传则渲染默认 ">_" 终端块，传 null 则不显示图标 */
  icon?: React.ReactNode;
  height?: number | string;
}

/** 统一空状态呈现 */
const EmptyState: React.FC<EmptyStateProps> = memo(({ title, hint, icon, height = '100%' }) => {
  const t = useUiTokens();
  return (
    <Box
      sx={{
        height,
        minHeight: 120,
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        gap: 1,
      }}
    >
      {icon === null ? null : icon !== undefined ? (
        icon
      ) : (
        <Box
          sx={{
            width: 40,
            height: 40,
            borderRadius: 2,
            border: `1px solid ${t.border}`,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            color: t.textMuted,
            fontSize: '1.1rem',
            fontFamily: monoFontFamily,
            mb: 0.5,
          }}
        >
          {'>'}_
        </Box>
      )}
      <Typography sx={{ color: t.textMuted, fontSize: '0.8125rem' }}>{title}</Typography>
      {hint && (
        <Typography sx={{ color: t.textMuted, fontSize: '0.7rem', fontFamily: monoFontFamily, opacity: 0.7 }}>
          {hint}
        </Typography>
      )}
    </Box>
  );
});

EmptyState.displayName = 'EmptyState';

export default EmptyState;
