import React, { useState, memo } from 'react';
import {
  Card,
  CardContent,
  Typography,
  List,
  ListItem,
  ListItemButton,
  ListItemText,
  IconButton,
  TextField,
  Button,
  Box,
} from '@mui/material';
import { Delete as DeleteIcon, Add as AddIcon, Send as SendIcon } from '@mui/icons-material';
import { useSerialStore, useLogStore, useAppStore } from '../../stores';

interface Command {
  id: string;
  name: string;
  command: string;
  waitResponse: boolean;
}

const defaultCommands: Command[] = [
  { id: '1', name: 'AT测试', command: 'AT', waitResponse: true },
  { id: '2', name: '获取版本', command: 'AT+VERSION', waitResponse: true },
  { id: '3', name: '重启设备', command: 'AT+RESET', waitResponse: false },
];

const CommandPanel: React.FC = memo(() => {
  const [commands, setCommands] = useState<Command[]>(defaultCommands);
  const [newName, setNewName] = useState('');
  const [newCommand, setNewCommand] = useState('');
  const { write, status } = useSerialStore();
  const { addLog } = useLogStore();
  const { addToast } = useAppStore();

  const handleSendCommand = async (cmd: Command) => {
    if (!status.connected) {
      addToast('请先连接串口', 'warning');
      return;
    }

    const success = await write(cmd.command);
    if (success) {
      addLog('info', 'Command', `Sent: ${cmd.name}`);
    }
  };

  const handleAddCommand = () => {
    if (!newName || !newCommand) return;

    setCommands([
      ...commands,
      {
        id: Date.now().toString(),
        name: newName,
        command: newCommand,
        waitResponse: true,
      },
    ]);
    setNewName('');
    setNewCommand('');
  };

  const handleDeleteCommand = (id: string) => {
    setCommands(commands.filter((c) => c.id !== id));
  };

  return (
    <Card
      sx={{
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        backgroundColor: '#1a1a1a',
        border: '1px solid #2a2a2a',
      }}
    >
      <CardContent sx={{ flexGrow: 1, overflow: 'auto', p: 2 }}>
        <Typography
          variant="subtitle2"
          sx={{
            color: '#a1a1aa',
            fontSize: '0.75rem',
            fontWeight: 600,
            textTransform: 'uppercase',
            letterSpacing: '0.05em',
            mb: 1.5,
            fontFamily: '"JetBrains Mono", monospace',
          }}
        >
          Quick Commands
        </Typography>
        <List dense sx={{ py: 0 }}>
          {commands.map((cmd) => (
            <ListItem
              key={cmd.id}
              secondaryAction={
                <Box sx={{ display: 'flex', gap: 0 }}>
                  <IconButton
                    edge="end"
                    size="small"
                    onClick={() => handleSendCommand(cmd)}
                    disabled={!status.connected}
                    sx={{
                      color: status.connected ? '#7c3aed' : '#3f3f46',
                      '&:hover': { color: '#a78bfa', backgroundColor: 'rgba(124, 58, 237, 0.1)' },
                    }}
                  >
                    <SendIcon fontSize="small" />
                  </IconButton>
                  <IconButton
                    edge="end"
                    size="small"
                    onClick={() => handleDeleteCommand(cmd.id)}
                    sx={{
                      color: '#52525b',
                      '&:hover': { color: '#f87171', backgroundColor: 'rgba(239, 68, 68, 0.1)' },
                    }}
                  >
                    <DeleteIcon fontSize="small" />
                  </IconButton>
                </Box>
              }
              disablePadding
              sx={{
                mb: 0.5,
              }}
            >
              <ListItemButton
                onClick={() => handleSendCommand(cmd)}
                disabled={!status.connected}
                sx={{
                  borderRadius: 1,
                  py: 0.75,
                  px: 1.5,
                  '&:hover': { backgroundColor: 'rgba(255, 255, 255, 0.03)' },
                }}
              >
                <ListItemText
                  primary={cmd.name}
                  secondary={cmd.command}
                  primaryTypographyProps={{
                    fontSize: '0.85rem',
                    fontWeight: 500,
                    color: '#e4e4e7',
                  }}
                  secondaryTypographyProps={{
                    fontSize: '0.7rem',
                    fontFamily: '"JetBrains Mono", monospace',
                    color: '#71717a',
                  }}
                />
              </ListItemButton>
            </ListItem>
          ))}
        </List>
      </CardContent>

      <Box
        sx={{
          p: 2,
          borderTop: '1px solid #2a2a2a',
        }}
      >
        <TextField
          size="small"
          placeholder="名称"
          value={newName}
          onChange={(e) => setNewName(e.target.value)}
          fullWidth
          sx={{ mb: 1 }}
        />
        <TextField
          size="small"
          placeholder="指令"
          value={newCommand}
          onChange={(e) => setNewCommand(e.target.value)}
          fullWidth
          sx={{ mb: 1.5 }}
          InputProps={{
            sx: { fontFamily: '"JetBrains Mono", monospace', fontSize: '0.8rem' },
          }}
        />
        <Button
          fullWidth
          size="small"
          startIcon={<AddIcon />}
          onClick={handleAddCommand}
          disabled={!newName || !newCommand}
          variant="outlined"
          sx={{
            borderColor: '#2a2a2a',
            color: '#a1a1aa',
            '&:hover': {
              borderColor: '#7c3aed',
              color: '#a78bfa',
              backgroundColor: 'rgba(124, 58, 237, 0.05)',
            },
          }}
        >
          添加指令
        </Button>
      </Box>
    </Card>
  );
});

CommandPanel.displayName = 'CommandPanel';

export default CommandPanel;
