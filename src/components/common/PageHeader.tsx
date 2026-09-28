import React, { memo } from 'react';
import { Box, Typography } from '@mui/material';
import { useUiTokens } from '../../theme/useUiTokens';

interface PageHeaderProps {
  title: string;
  description?: string;
}

/** 页面标题区：标题 + 可选描述，间距与字重统一 */
const PageHeader: React.FC<PageHeaderProps> = memo(({ title, description }) => {
  const t = useUiTokens();
  return (
    <Box sx={{ mb: 3 }}>
      <Typography variant="h5" sx={{ color: t.text }}>
        {title}
      </Typography>
      {description && (
        <Typography sx={{ color: t.textMuted, mt: 0.5, fontSize: '0.875rem' }}>{description}</Typography>
      )}
    </Box>
  );
});

PageHeader.displayName = 'PageHeader';

export default PageHeader;
