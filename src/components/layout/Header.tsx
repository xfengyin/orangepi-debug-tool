import React, { memo } from 'react';
import {
  AppBar,
  Toolbar,
  Typography,
  IconButton,
  Box,
  Button,
  Tooltip,
} from '@mui/material';
import {
  Menu as MenuIcon,
  DarkMode as DarkModeIcon,
  LightMode as LightModeIcon,
  Refresh as RefreshIcon,
  BugReport as DebugIcon,
} from '@mui/icons-material';
import { useThemeStore, useAppStore, useSerialStore } from '../../stores';

const Header: React.FC = memo(() => {
  const { mode, toggleMode } = useThemeStore();
  const { toggleSidebar, systemInfo } = useAppStore();
  const { refreshPorts, status } = useSerialStore();

  return (
    <AppBar
      position="static"
      elevation={0}
      sx={{
        backgroundColor: (theme) => theme.palette.background.paper,
        borderBottom: (theme) => `1px solid ${theme.palette.divider}`,
      }}
    >
      <Toolbar variant="dense">
        <IconButton
          edge="start"
          color="inherit"
          onClick={toggleSidebar}
          sx={{ mr: 2 }}
        >
          <MenuIcon />
        </IconButton>

        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
          {status.connected && status.config && (
            <Box component="span" sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              <DebugIcon fontSize="small" color="primary" />
              {status.config.port_name} @ {status.config.baud_rate} bps
            </Box>
          )}
        </Typography>

        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          {/* Refresh button */}
          <Tooltip title="刷新设备">
            <IconButton onClick={refreshPorts} size="small">
              <RefreshIcon />
            </IconButton>
          </Tooltip>

          {/* Theme toggle */}
          <Tooltip title={mode === 'light' ? '切换深色模式' : '切换浅色模式'}>
            <IconButton onClick={toggleMode} size="small">
              {mode === 'light' ? <DarkModeIcon /> : <LightModeIcon />}
            </IconButton>
          </Tooltip>

          {/* Version info */}
          <Typography variant="caption" color="text.secondary" sx={{ ml: 1 }}>
            v{systemInfo?.version || '2.0.0'}
          </Typography>
        </Box>
      </Toolbar>
    </AppBar>
  );
});

Header.displayName = 'Header';

export default Header;