import React, { useEffect, useMemo, useState } from 'react';
import {
  Dialog,
  DialogContent,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  TextField,
  Box,
  Typography,
} from '@mui/material';
import {
  Usb as UsbIcon,
  SettingsInputComponent as GpioIcon,
  Waves as PwmIcon,
  Article as LogIcon,
  Settings as SettingsIcon,
  Home as HomeIcon,
} from '@mui/icons-material';
import { useAppStore } from '../../stores';
import type { ViewType } from '../../types';

interface Command {
  id: ViewType;
  label: string;
  keywords: string;
  icon: React.ReactNode;
}

const commands: Command[] = [
  { id: 'overview', label: '概览', keywords: 'overview home 首页', icon: <HomeIcon /> },
  { id: 'serial', label: '串口调试', keywords: 'serial 串口 usb', icon: <UsbIcon /> },
  { id: 'gpio', label: 'GPIO 控制', keywords: 'gpio 引脚', icon: <GpioIcon /> },
  { id: 'pwm', label: 'PWM 输出', keywords: 'pwm 波形 频率', icon: <PwmIcon /> },
  { id: 'log', label: '数据日志', keywords: 'log 日志', icon: <LogIcon /> },
  { id: 'settings', label: '设置', keywords: 'settings 设置 配置', icon: <SettingsIcon /> },
];

interface CommandPaletteProps {
  open: boolean;
  onClose: () => void;
}

const CommandPalette: React.FC<CommandPaletteProps> = ({ open, onClose }) => {
  const { setCurrentView } = useAppStore();
  const [query, setQuery] = useState('');

  useEffect(() => {
    if (open) {
      setQuery('');
    }
  }, [open]);

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return commands;
    return commands.filter((cmd) => {
      return [cmd.label, cmd.keywords].join(' ').toLowerCase().includes(q);
    });
  }, [query]);

  const handleSelect = (view: ViewType) => {
    setCurrentView(view);
    onClose();
  };

  return (
    <Dialog
      open={open}
      onClose={onClose}
      fullWidth
      maxWidth="sm"
      PaperProps={{
        sx: {
          backgroundColor: '#121820',
          backgroundImage: 'none',
          border: '1px solid #232C38',
          borderRadius: 2,
          boxShadow: '0 0 30px rgba(255, 107, 53, 0.08)',
        },
      }}
    >
      <DialogContent sx={{ p: 2 }}>
        <TextField
          autoFocus
          fullWidth
          placeholder="输入命令或页面名称..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          sx={{
            mb: 1.5,
            input: { color: '#E5EAF3', fontFamily: 'JetBrains Mono, monospace', fontSize: '0.9rem' },
            '& .MuiOutlinedInput-root': {
              backgroundColor: '#0A0E14',
              '& fieldset': { borderColor: '#232C38' },
              '&:hover fieldset': { borderColor: '#FF6B35' },
            },
          }}
        />
        <List dense sx={{ p: 0 }}>
          {filtered.length === 0 && (
            <Typography sx={{ color: '#7A8798', fontSize: '0.85rem', p: 1 }}>
              没有匹配的命令
            </Typography>
          )}
          {filtered.map((cmd) => (
            <ListItemButton
              key={cmd.id}
              onClick={() => handleSelect(cmd.id)}
              sx={{
                borderRadius: 1,
                mx: 0,
                color: '#E5EAF3',
                '&:hover': { backgroundColor: 'rgba(255, 107, 53, 0.08)', color: '#FF8F5E' },
              }}
            >
              <ListItemIcon sx={{ color: 'inherit', minWidth: 34 }}>
                {cmd.icon}
              </ListItemIcon>
              <ListItemText primary={cmd.label} primaryTypographyProps={{ fontSize: '0.9rem' }} />
              <Box component="span" sx={{ color: '#7A8798', fontSize: '0.7rem', fontFamily: 'monospace' }}>
                go:{cmd.id}
              </Box>
            </ListItemButton>
          ))}
        </List>
      </DialogContent>
    </Dialog>
  );
};

export default CommandPalette;
