import React, { memo } from 'react';
import {
  AppBar,
  Toolbar,
  Typography,
  IconButton,
  Box,
  Tooltip,
} from '@mui/material';
import {
  Menu as MenuIcon,
  Search as SearchIcon,
  DarkMode as DarkModeIcon,
  LightMode as LightModeIcon,
  Refresh as RefreshIcon,
} from '@mui/icons-material';
import { useThemeStore, useAppStore, useSerialStore } from '../../stores';
import { monoFontFamily } from '../../theme';
import { useUiTokens } from '../../theme/useUiTokens';
import StatusDot from '../common/StatusDot';

interface HeaderProps {
  onOpenCommand: () => void;
}

const Header: React.FC<HeaderProps> = memo(({ onOpenCommand }) => {
  const t = useUiTokens();
  const { mode, toggleMode } = useThemeStore();
  const { toggleSidebar, systemInfo } = useAppStore();
  const { refreshPorts, status, config } = useSerialStore();

  return (
    <AppBar position="static" elevation={0}>
      <Toolbar variant="dense" sx={{ minHeight: '44px !important' }}>
        <IconButton edge="start" onClick={toggleSidebar} sx={{ mr: 2 }}>
          <MenuIcon fontSize="small" />
        </IconButton>

        <Box sx={{ flexGrow: 1, display: 'flex', alignItems: 'center', gap: 1.5 }}>
          {status.connected && config ? (
            <>
              <StatusDot state="connected" />
              <Typography
                sx={{
                  fontSize: '0.8rem',
                  fontFamily: monoFontFamily,
                  color: t.textSecondary,
                  letterSpacing: '0.02em',
                }}
              >
                {config.port_name} @ {config.baud_rate} bps
              </Typography>
            </>
          ) : (
            <Typography
              sx={{
                fontSize: '0.8rem',
                fontFamily: monoFontFamily,
                color: t.textMuted,
                letterSpacing: '0.02em',
              }}
            >
              No connection
            </Typography>
          )}
        </Box>

        <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
          {/* Command palette */}
          <IconButton onClick={onOpenCommand} size="small">
            <SearchIcon fontSize="small" />
          </IconButton>

          {/* Refresh button */}
          <Tooltip title="刷新设备" arrow>
            <IconButton onClick={refreshPorts} size="small">
              <RefreshIcon fontSize="small" />
            </IconButton>
          </Tooltip>

          {/* Theme toggle */}
          <Tooltip title={mode === 'light' ? '深色模式' : '浅色模式'} arrow>
            <IconButton onClick={toggleMode} size="small">
              {mode === 'light' ? <DarkModeIcon fontSize="small" /> : <LightModeIcon fontSize="small" />}
            </IconButton>
          </Tooltip>

          {/* Version */}
          <Typography
            sx={{
              ml: 1,
              fontSize: '0.65rem',
              fontFamily: monoFontFamily,
              color: t.textMuted,
              px: 1,
              py: 0.25,
              borderRadius: 1,
              border: `1px solid ${t.border}`,
              backgroundColor: t.surfaceAlt,
            }}
          >
            v{systemInfo?.version || '2.0.0'}
          </Typography>
        </Box>
      </Toolbar>
    </AppBar>
  );
});

Header.displayName = 'Header';

export default Header;
