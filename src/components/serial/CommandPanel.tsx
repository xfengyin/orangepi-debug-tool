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
    <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      <CardContent sx={{ flexGrow: 1, overflow: 'auto', p: 2 }}>
        <Typography variant="subtitle2" gutterBottom>
          快捷指令
        </Typography>
        <List dense>
          {commands.map((cmd) => (
            <ListItem
              key={cmd.id}
              secondaryAction={
                <Box>
                  <IconButton
                    edge="end"
                    size="small"
                    onClick={() => handleSendCommand(cmd)}
                    disabled={!status.connected}
                  >
                    <SendIcon fontSize="small" />
                  </IconButton>
                  <IconButton
                    edge="end"
                    size="small"
                    onClick={() => handleDeleteCommand(cmd.id)}
                  >
                    <DeleteIcon fontSize="small" />
                  </IconButton>
                </Box>
              }
              disablePadding
            >
              <ListItemButton onClick={() => handleSendCommand(cmd)} disabled={!status.connected}>
                <ListItemText primary={cmd.name} secondary={cmd.command} />
              </ListItemButton>
            </ListItem>
          ))}
        </List>
      </CardContent>

      <Box sx={{ p: 2, borderTop: 1, borderColor: 'divider' }}>
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
          sx={{ mb: 1 }}
        />
        <Button
          fullWidth
          size="small"
          startIcon={<AddIcon />}
          onClick={handleAddCommand}
          disabled={!newName || !newCommand}
        >
          添加
        </Button>
      </Box>
    </Card>
  );
});

CommandPanel.displayName = 'CommandPanel';

export default CommandPanel;