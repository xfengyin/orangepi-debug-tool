import React, { useState, useCallback, memo } from 'react';
import {
  Box,
  Card,
  CardContent,
  TextField,
  Button,
  IconButton,
  Tooltip,
  ToggleButton,
  ToggleButtonGroup,
  Grid,
} from '@mui/material';
import {
  Send as SendIcon,
  Clear as ClearIcon,
  Hexagon as HexIcon,
  TextFields as TextIcon,
} from '@mui/icons-material';
import { useSerialStore, useLogStore } from '../../stores';
import { monoFontFamily, sansFontFamily } from '../../theme';

type InputMode = 'text' | 'hex';

/** 数据发送面板：文本/HEX 输入、发送与清空 */
const SendPanel: React.FC = memo(() => {
  const { status, clearData } = useSerialStore();
  const { addLog } = useLogStore();
  const [inputMode, setInputMode] = useState<InputMode>('text');
  const [inputText, setInputText] = useState('');

  const handleSend = useCallback(async () => {
    if (!inputText.trim() || !status.connected) return;

    let data: string | number[];
    if (inputMode === 'hex') {
      const hexStr = inputText.replace(/\s/g, '');
      data = hexStr.match(/.{1,2}/g)?.map((b) => parseInt(b, 16)) || [];
    } else {
      data = inputText;
    }

    const success = await useSerialStore.getState().write(data);
    if (success) {
      setInputText('');
      addLog('debug', 'Serial TX', inputText);
    }
  }, [inputText, inputMode, status.connected, addLog]);

  return (
    <Card>
      <CardContent>
        <Grid container spacing={2} alignItems="center">
          <Grid item xs={12} md={9}>
            <TextField
              fullWidth
              size="small"
              placeholder={inputMode === 'hex' ? '输入十六进制 (如: 01 02 03)' : '输入要发送的数据...'}
              value={inputText}
              onChange={(e) => setInputText(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSend()}
              disabled={!status.connected}
              InputProps={{
                startAdornment: (
                  <ToggleButtonGroup
                    size="small"
                    value={inputMode}
                    exclusive
                    onChange={(_, v) => v && setInputMode(v as InputMode)}
                    sx={{
                      mr: 1,
                      '& .MuiToggleButton-root': {
                        py: 0.25,
                        px: 1,
                      },
                    }}
                  >
                    <ToggleButton value="text">
                      <TextIcon sx={{ fontSize: '0.9rem' }} />
                    </ToggleButton>
                    <ToggleButton value="hex">
                      <HexIcon sx={{ fontSize: '0.9rem' }} />
                    </ToggleButton>
                  </ToggleButtonGroup>
                ),
                sx: {
                  fontFamily: inputMode === 'hex' ? monoFontFamily : sansFontFamily,
                  fontSize: '0.85rem',
                },
              }}
            />
          </Grid>
          <Grid item xs={12} md={3}>
            <Box sx={{ display: 'flex', gap: 1 }}>
              <Button
                variant="contained"
                startIcon={<SendIcon />}
                onClick={handleSend}
                disabled={!status.connected || !inputText.trim()}
                fullWidth
              >
                发送
              </Button>
              <Tooltip title="清空" arrow>
                <span>
                  <IconButton onClick={clearData} disabled={!status.connected}>
                    <ClearIcon />
                  </IconButton>
                </span>
              </Tooltip>
            </Box>
          </Grid>
        </Grid>
      </CardContent>
    </Card>
  );
});

SendPanel.displayName = 'SendPanel';

export default SendPanel;
