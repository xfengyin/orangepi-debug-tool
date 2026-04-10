import React, { memo } from 'react';
import {
  Box,
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Divider,
  Typography,
  Chip,
} from '@mui/material';
import {
  Usb as UsbIcon,
  SettingsInputComponent as GpioIcon,
  Waves as PwmIcon,
  Article as LogIcon,
  Settings as SettingsIcon,
} from '@mui/icons-material';
import { useAppStore, useSerialStore } from '../../stores';
import type { ViewType } from '../../types';

interface NavItem {
  id: ViewType;
  label: string;
  icon: React.ReactNode;
  badge?: string | number;
}

const navItems: NavItem[] = [
  { id: 'serial', label: '串口调试', icon: <UsbIcon /> },
  { id: 'gpio', label: 'GPIO控制', icon: <GpioIcon /> },
  { id: 'pwm', label: 'PWM输出', icon: <PwmIcon /> },
  { id: 'log', label: '数据日志', icon: <LogIcon /> },
];

const Sidebar: React.FC = memo(() => {
  const { currentView, setCurrentView, isSidebarOpen, isOrangePi } = useAppStore();
  const { status } = useSerialStore();

  const drawerWidth = 240;

  const handleNavClick = (view: ViewType) => {
    setCurrentView(view);
  };

  return (
    <Drawer
      variant="persistent"
      anchor="left"
      open={isSidebarOpen}
      sx={{
        width: drawerWidth,
        flexShrink: 0,
        '& .MuiDrawer-paper': {
          width: drawerWidth,
          boxSizing: 'border-box',
          borderRight: '1px solid #2a2a2a',
          backgroundColor: '#0a0a0a',
          backgroundImage: 'none',
        },
      }}
    >
      {/* Logo - Cursor-style gradient accent */}
      <Box
        sx={{
          p: 2,
          display: 'flex',
          alignItems: 'center',
          gap: 1.5,
          borderBottom: '1px solid #2a2a2a',
        }}
      >
        <Box
          sx={{
            width: 36,
            height: 36,
            borderRadius: 2,
            background: 'linear-gradient(135deg, #7c3aed 0%, #3b82f6 100%)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            color: 'white',
            fontWeight: 700,
            fontSize: '0.85rem',
            fontFamily: '"Inter", sans-serif',
            letterSpacing: '-0.5px',
            boxShadow: '0 0 12px rgba(124, 58, 237, 0.3)',
          }}
        >
          OP
        </Box>
        <Box>
          <Typography
            variant="subtitle1"
            fontWeight={600}
            noWrap
            sx={{
              fontSize: '0.9rem',
              color: '#ffffff',
              letterSpacing: '-0.01em',
            }}
          >
            OrangePi
          </Typography>
          <Typography
            variant="caption"
            noWrap
            sx={{
              color: '#71717a',
              fontSize: '0.7rem',
              fontFamily: '"JetBrains Mono", monospace',
            }}
          >
            Debug Tool v2.0
          </Typography>
        </Box>
      </Box>

      {/* Device status - Cursor-style indicator */}
      <Box sx={{ p: 2, pb: 1 }}>
        <Box
          sx={{
            display: 'flex',
            alignItems: 'center',
            gap: 1,
            px: 1.5,
            py: 0.75,
            borderRadius: 1.5,
            backgroundColor: status.connected ? 'rgba(74, 222, 128, 0.06)' : 'rgba(113, 113, 122, 0.06)',
            border: '1px solid',
            borderColor: status.connected ? 'rgba(74, 222, 128, 0.15)' : 'rgba(113, 113, 122, 0.1)',
          }}
        >
          <Box
            component="span"
            sx={{
              width: 6,
              height: 6,
              borderRadius: '50%',
              backgroundColor: status.connected ? '#4ade80' : '#71717a',
              boxShadow: status.connected ? '0 0 6px rgba(74, 222, 128, 0.5)' : 'none',
              transition: 'all 0.3s ease',
            }}
          />
          <Typography
            sx={{
              fontSize: '0.75rem',
              color: status.connected ? '#4ade80' : '#71717a',
              fontWeight: 500,
              fontFamily: '"JetBrains Mono", monospace',
            }}
          >
            {status.connected ? 'CONNECTED' : 'DISCONNECTED'}
          </Typography>
          {isOrangePi && (
            <Chip
              size="small"
              label="OPi"
              sx={{
                ml: 'auto',
                height: 18,
                fontSize: '0.6rem',
                fontFamily: '"JetBrains Mono", monospace',
                backgroundColor: 'rgba(124, 58, 237, 0.15)',
                color: '#a78bfa',
                border: '1px solid rgba(124, 58, 237, 0.25)',
              }}
            />
          )}
        </Box>
      </Box>

      <Divider sx={{ borderColor: '#2a2a2a' }} />

      {/* Navigation - File-tree style */}
      <List sx={{ flexGrow: 1, pt: 1, px: 0.5 }}>
        {navItems.map((item) => {
          const isActive = currentView === item.id;
          return (
            <ListItem key={item.id} disablePadding>
              <ListItemButton
                selected={isActive}
                onClick={() => handleNavClick(item.id)}
                sx={{
                  mx: 1,
                  borderRadius: 1.5,
                  py: 1,
                  px: 1.5,
                  color: isActive ? '#a78bfa' : '#71717a',
                  '&:hover': {
                    backgroundColor: 'rgba(255, 255, 255, 0.04)',
                    color: '#e4e4e7',
                  },
                  '&.Mui-selected': {
                    backgroundColor: 'rgba(124, 58, 237, 0.1)',
                    color: '#a78bfa',
                    '&:hover': {
                      backgroundColor: 'rgba(124, 58, 237, 0.15)',
                      color: '#c4b5fd',
                    },
                  },
                }}
              >
                <ListItemIcon
                  sx={{
                    color: 'inherit',
                    minWidth: 32,
                    '& .MuiSvgIcon-root': {
                      fontSize: '1.1rem',
                    },
                  }}
                >
                  {item.icon}
                </ListItemIcon>
                <ListItemText
                  primary={item.label}
                  primaryTypographyProps={{
                    fontWeight: isActive ? 600 : 400,
                    fontSize: '0.85rem',
                    color: 'inherit',
                    letterSpacing: '0.01em',
                  }}
                />
                {isActive && (
                  <Box
                    sx={{
                      width: 3,
                      height: 14,
                      borderRadius: 1.5,
                      background: 'linear-gradient(180deg, #7c3aed, #3b82f6)',
                      mr: -0.5,
                    }}
                  />
                )}
              </ListItemButton>
            </ListItem>
          );
        })}
      </List>

      <Divider sx={{ borderColor: '#2a2a2a' }} />

      {/* Footer */}
      <Box sx={{ p: 1.5, px: 0.5 }}>
        <ListItemButton
          onClick={() => handleNavClick('settings')}
          sx={{
            borderRadius: 1.5,
            py: 1,
            px: 1.5,
            color: '#71717a',
            '&:hover': {
              backgroundColor: 'rgba(255, 255, 255, 0.04)',
              color: '#e4e4e7',
            },
          }}
        >
          <ListItemIcon sx={{ minWidth: 32, color: 'inherit', '& .MuiSvgIcon-root': { fontSize: '1.1rem' } }}>
            <SettingsIcon />
          </ListItemIcon>
          <ListItemText
            primary="设置"
            primaryTypographyProps={{
              fontSize: '0.85rem',
              fontWeight: 400,
              color: 'inherit',
            }}
          />
        </ListItemButton>
      </Box>
    </Drawer>
  );
});

Sidebar.displayName = 'Sidebar';

export default Sidebar;
