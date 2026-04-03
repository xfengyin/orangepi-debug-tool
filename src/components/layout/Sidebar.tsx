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
          borderRight: (theme) => `1px solid ${theme.palette.divider}`,
        },
      }}
    >
      {/* Logo */}
      <Box
        sx={{
          p: 2,
          display: 'flex',
          alignItems: 'center',
          gap: 1,
          borderBottom: (theme) => `1px solid ${theme.palette.divider}`,
        }}
      >
        <Box
          sx={{
            width: 40,
            height: 40,
            borderRadius: 2,
            background: 'linear-gradient(135deg, #165DFF 0%, #4d85ff 100%)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            color: 'white',
            fontWeight: 'bold',
            fontSize: '1.2rem',
          }}
        >
          OP
        </Box>
        <Box>
          <Typography variant="subtitle1" fontWeight="bold" noWrap>
            OrangePi
          </Typography>
          <Typography variant="caption" color="text.secondary" noWrap>
            Debug Tool v2.0
          </Typography>
        </Box>
      </Box>

      {/* Device status */}
      <Box sx={{ p: 2, pb: 1 }}>
        <Chip
          size="small"
          label={status.connected ? '已连接' : '未连接'}
          color={status.connected ? 'success' : 'default'}
          sx={{
            '& .MuiChip-label': {
              display: 'flex',
              alignItems: 'center',
              gap: 0.5,
            },
          }}
          icon={
            <Box
              component="span"
              sx={{
                width: 8,
                height: 8,
                borderRadius: '50%',
                backgroundColor: status.connected ? '#4ade80' : '#94a3b8',
                boxShadow: status.connected ? '0 0 8px #4ade80' : 'none',
              }}
            />
          }
        />
        {isOrangePi && (
          <Chip
            size="small"
            label="OrangePi"
            color="primary"
            sx={{ ml: 1 }}
          />
        )}
      </Box>

      <Divider />

      {/* Navigation */}
      <List sx={{ flexGrow: 1, pt: 1 }}>
        {navItems.map((item) => (
          <ListItem key={item.id} disablePadding>
            <ListItemButton
              selected={currentView === item.id}
              onClick={() => handleNavClick(item.id)}
              sx={{
                mx: 1,
                borderRadius: 2,
                '&.Mui-selected': {
                  backgroundColor: (theme) =>
                    theme.palette.mode === 'dark'
                      ? 'rgba(77, 133, 255, 0.16)'
                      : 'rgba(22, 93, 255, 0.12)',
                },
              }}
            >
              <ListItemIcon
                sx={{
                  color:
                    currentView === item.id
                      ? 'primary.main'
                      : 'text.secondary',
                  minWidth: 40,
                }}
              >
                {item.icon}
              </ListItemIcon>
              <ListItemText
                primary={item.label}
                primaryTypographyProps={{
                  fontWeight: currentView === item.id ? 600 : 400,
                  color: currentView === item.id ? 'primary.main' : 'text.primary',
                }}
              />
              {item.badge && (
                <Chip
                  size="small"
                  label={item.badge}
                  color="primary"
                  sx={{ height: 20 }}
                />
              )}
            </ListItemButton>
          </ListItem>
        ))}
      </List>

      <Divider />

      {/* Footer */}
      <Box sx={{ p: 2 }}>
        <ListItemButton
          onClick={() => handleNavClick('settings')}
          sx={{
            borderRadius: 2,
          }}
        >
          <ListItemIcon sx={{ minWidth: 40 }}>
            <SettingsIcon />
          </ListItemIcon>
          <ListItemText primary="设置" />
        </ListItemButton>
      </Box>
    </Drawer>
  );
});

Sidebar.displayName = 'Sidebar';

export default Sidebar;