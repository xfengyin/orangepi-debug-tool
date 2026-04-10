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
  DarkMode as DarkModeIcon,
  LightMode as LightModeIcon,
  Refresh as RefreshIcon,
  Circle as CircleIcon,
} from '@mui/icons-material';
import { useThemeStore, useAppStore, useSerialStore } from '../../stores';

const Header: React.FC = memo(() => {
  const { mode, toggleMode } = useThemeStore();
  const { toggleSidebar, systemInfo } = useAppStore();
  const { refreshPorts, status, config } = useSerialStore();

  return (
    <AppBar
      position="static"
      elevation={0}
      sx={{
        backgroundColor: '#0a0a0a',
        backgroundImage: 'none',
        borderBottom: '1px solid #2a2a2a',
      }}
    >
      <Toolbar variant="dense" sx={{ minHeight: '40px !important' }}>
        <IconButton
          edge="start"
          onClick={toggleSidebar}
          sx={{
            mr: 2,
            color: '#71717a',
            '&:hover': { color: '#e4e4e7', backgroundColor: 'rgba(255,255,255,0.05)' },
          }}
        >
          <MenuIcon fontSize="small" />
        </IconButton>

        <Box sx={{ flexGrow: 1, display: 'flex', alignItems: 'center', gap: 1.5 }}>
          {status.connected && config ? (
            <>
              <Box
                component="span"
                sx={{
                  width: 6,
                  height: 6,
                  borderRadius: '50%',
                  backgroundColor: '#4ade80',
                  boxShadow: '0 0 6px rgba(74, 222, 128, 0.5)',
                  flexShrink: 0,
                }}
              />
              <Typography
                sx={{
                  fontSize: '0.8rem',
                  fontFamily: '"JetBrains Mono", monospace',
                  color: '#a1a1aa',
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
                fontFamily: '"JetBrains Mono", monospace',
                color: '#52525b',
                letterSpacing: '0.02em',
              }}
            >
              No connection
            </Typography>
          )}
        </Box>

        <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
          {/* Refresh button */}
          <Tooltip title="刷新设备" arrow>
            <IconButton
              onClick={refreshPorts}
              size="small"
              sx={{
                color: '#71717a',
                '&:hover': { color: '#e4e4e7', backgroundColor: 'rgba(255,255,255,0.05)' },
              }}
            >
              <RefreshIcon fontSize="small" />
            </IconButton>
          </Tooltip>

          {/* Theme toggle */}
          <Tooltip title={mode === 'light' ? '深色模式' : '浅色模式'} arrow>
            <IconButton
              onClick={toggleMode}
              size="small"
              sx={{
                color: '#71717a',
                '&:hover': { color: '#e4e4e7', backgroundColor: 'rgba(255,255,255,0.05)' },
              }}
            >
              {mode === 'light' ? <DarkModeIcon fontSize="small" /> : <LightModeIcon fontSize="small" />}
            </IconButton>
          </Tooltip>

          {/* Version - Mono style */}
          <Typography
            sx={{
              ml: 1,
              fontSize: '0.65rem',
              fontFamily: '"JetBrains Mono", monospace',
              color: '#52525b',
              px: 1,
              py: 0.25,
              borderRadius: 1,
              border: '1px solid #2a2a2a',
              backgroundColor: '#141414',
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
