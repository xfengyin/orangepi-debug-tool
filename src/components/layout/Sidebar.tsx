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
  Terminal as TerminalIcon,
  Usb as UsbIcon,
  SettingsInputComponent as GpioIcon,
  Waves as PwmIcon,
  Article as LogIcon,
  Settings as SettingsIcon,
} from '@mui/icons-material';
import { useAppStore, useSerialStore } from '../../stores';
import type { ViewType } from '../../types';
import { monoFontFamily } from '../../theme';
import { useUiTokens } from '../../theme/useUiTokens';
import StatusDot from '../common/StatusDot';
import { DRAWER_WIDTH } from './constants';

interface NavItem {
  id: ViewType;
  label: string;
  icon: React.ReactNode;
  badge?: string | number;
}

const navItems: NavItem[] = [
  { id: 'overview', label: '概览', icon: <TerminalIcon /> },
  { id: 'serial', label: '串口调试', icon: <UsbIcon /> },
  { id: 'gpio', label: 'GPIO控制', icon: <GpioIcon /> },
  { id: 'pwm', label: 'PWM输出', icon: <PwmIcon /> },
  { id: 'log', label: '数据日志', icon: <LogIcon /> },
  { id: 'settings', label: '设置', icon: <SettingsIcon /> },
];

const Sidebar: React.FC = memo(() => {
  const t = useUiTokens();
  const { currentView, setCurrentView, isSidebarOpen, isOrangePi } = useAppStore();
  const { status } = useSerialStore();

  const handleNavClick = (view: ViewType) => {
    setCurrentView(view);
  };

  return (
    <Drawer
      variant="persistent"
      anchor="left"
      open={isSidebarOpen}
      sx={{
        width: DRAWER_WIDTH,
        flexShrink: 0,
        '& .MuiDrawer-paper': {
          width: DRAWER_WIDTH,
          boxSizing: 'border-box',
        },
      }}
    >
      {/* Logo */}
      <Box
        sx={{
          p: 2,
          display: 'flex',
          alignItems: 'center',
          gap: 1.5,
          borderBottom: `1px solid ${t.border}`,
        }}
      >
        <Box
          sx={{
            width: 34,
            height: 34,
            borderRadius: 1.5,
            backgroundColor: t.brand,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            color: '#ffffff',
            fontWeight: 700,
            fontSize: '0.85rem',
            letterSpacing: '-0.5px',
          }}
        >
          OP
        </Box>
        <Box>
          <Typography
            variant="subtitle1"
            fontWeight={600}
            noWrap
            sx={{ fontSize: '0.9rem', color: t.text, letterSpacing: '-0.01em' }}
          >
            OrangePi
          </Typography>
          <Typography
            variant="caption"
            noWrap
            sx={{ color: t.textMuted, fontSize: '0.7rem', fontFamily: monoFontFamily }}
          >
            Debug Tool v2.0
          </Typography>
        </Box>
      </Box>

      {/* Device status */}
      <Box sx={{ p: 2, pb: 1 }}>
        <Box
          sx={{
            display: 'flex',
            alignItems: 'center',
            gap: 1,
            px: 1.5,
            py: 0.75,
            borderRadius: 1.5,
            backgroundColor: status.connected ? t.success.soft : t.surfaceAlt,
            border: `1px solid ${status.connected ? t.success.soft : t.border}`,
          }}
        >
          <StatusDot state={status.connected ? 'connected' : 'disconnected'} />
          <Typography
            sx={{
              fontSize: '0.75rem',
              color: status.connected ? t.success.main : t.textMuted,
              fontWeight: 500,
              fontFamily: monoFontFamily,
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
                fontFamily: monoFontFamily,
                backgroundColor: t.brandSoft,
                color: t.brand,
                border: `1px solid ${t.brandSoft}`,
              }}
            />
          )}
        </Box>
      </Box>

      <Divider />

      {/* Navigation */}
      <List sx={{ flexGrow: 1, pt: 1, px: 0.5 }}>
        {navItems.map((item) => {
          const isActive = currentView === item.id;
          return (
            <ListItem key={item.id} disablePadding>
              <ListItemButton selected={isActive} onClick={() => handleNavClick(item.id)}>
                <ListItemIcon sx={{ '& .MuiSvgIcon-root': { fontSize: '1.1rem' } }}>
                  {item.icon}
                </ListItemIcon>
                <ListItemText
                  primary={item.label}
                  primaryTypographyProps={{
                    fontWeight: isActive ? 600 : 400,
                    fontSize: '0.85rem',
                    color: 'inherit',
                  }}
                />
                {isActive && (
                  <Box
                    sx={{
                      width: 3,
                      height: 14,
                      borderRadius: 1.5,
                      backgroundColor: t.brand,
                      mr: -0.5,
                    }}
                  />
                )}
              </ListItemButton>
            </ListItem>
          );
        })}
      </List>

      <Divider />

      {/* Footer */}
      <Box sx={{ p: 1.5, px: 0.5 }}>
        <ListItemButton onClick={() => handleNavClick('settings')}>
          <ListItemIcon sx={{ '& .MuiSvgIcon-root': { fontSize: '1.1rem' } }}>
            <SettingsIcon />
          </ListItemIcon>
          <ListItemText
            primary="设置"
            primaryTypographyProps={{ fontSize: '0.85rem', fontWeight: 400, color: 'inherit' }}
          />
        </ListItemButton>
      </Box>
    </Drawer>
  );
});

Sidebar.displayName = 'Sidebar';

export default Sidebar;
