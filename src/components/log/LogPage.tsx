import React, { useMemo, memo } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Chip,
  IconButton,
  Tooltip,
  Button,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
} from '@mui/material';
import { Delete as DeleteIcon, Download as DownloadIcon } from '@mui/icons-material';
import { useLogStore } from '../../stores';

const levelConfig: Record<string, { color: string; bgColor: string; borderColor: string }> = {
  debug: { color: '#71717a', bgColor: 'rgba(113, 113, 122, 0.08)', borderColor: 'rgba(113, 113, 122, 0.15)' },
  info: { color: '#60a5fa', bgColor: 'rgba(96, 165, 250, 0.08)', borderColor: 'rgba(96, 165, 250, 0.15)' },
  warn: { color: '#fbbf24', bgColor: 'rgba(251, 191, 36, 0.08)', borderColor: 'rgba(251, 191, 36, 0.15)' },
  error: { color: '#f87171', bgColor: 'rgba(248, 113, 113, 0.08)', borderColor: 'rgba(248, 113, 113, 0.15)' },
};

const LogPage: React.FC = memo(() => {
  const {
    entries,
    filter,
    clearLogs,
    exportLogs,
    setFilter,
    getFilteredEntries,
  } = useLogStore();

  const filteredEntries = useMemo(() => getFilteredEntries(), [entries, filter]);

  const handleExport = () => {
    const data = exportLogs();
    const blob = new Blob([data], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `debug-tool-log-${new Date().toISOString().slice(0, 10)}.txt`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <Box>
      <Typography
        variant="h5"
        sx={{
          fontWeight: 600,
          color: '#ffffff',
          mb: 3,
          letterSpacing: '-0.01em',
        }}
      >
        数据日志
      </Typography>

      {/* Filters - Cursor dark card */}
      <Card sx={{ mb: 2, backgroundColor: '#1a1a1a', border: '1px solid #2a2a2a' }}>
        <CardContent sx={{ py: 1.5, '&:last-child': { pb: 1.5 } }}>
          <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap', alignItems: 'center' }}>
            <FormControl size="small" sx={{ minWidth: 120 }}>
              <InputLabel sx={{ color: '#71717a', '&.Mui-focused': { color: '#a78bfa' } }}>日志级别</InputLabel>
              <Select
                multiple
                value={filter.levels}
                onChange={(e) => setFilter({ levels: e.target.value as string[] })}
                renderValue={(selected) => (
                  <Typography sx={{ fontFamily: '"JetBrains Mono", monospace', fontSize: '0.8rem' }}>
                    {(selected as string[]).join(', ')}
                  </Typography>
                )}
                sx={{
                  backgroundColor: '#0a0a0a',
                  '& .MuiSelect-icon': { color: '#71717a' },
                }}
              >
                {['debug', 'info', 'warn', 'error'].map((level) => (
                  <MenuItem key={level} value={level}>
                    <Typography sx={{ fontFamily: '"JetBrains Mono", monospace', fontSize: '0.8rem' }}>
                      {level.toUpperCase()}
                    </Typography>
                  </MenuItem>
                ))}
              </Select>
            </FormControl>

            <TextField
              size="small"
              placeholder="搜索..."
              value={filter.search}
              onChange={(e) => setFilter({ search: e.target.value })}
              sx={{ flexGrow: 1, maxWidth: 300 }}
            />

            <Box sx={{ flexGrow: 1 }} />

            <Button
              startIcon={<DownloadIcon />}
              onClick={handleExport}
              sx={{
                color: '#71717a',
                '&:hover': { color: '#a78bfa', backgroundColor: 'rgba(124, 58, 237, 0.05)' },
              }}
            >
              导出
            </Button>
            <Button
              startIcon={<DeleteIcon />}
              onClick={clearLogs}
              sx={{
                color: '#52525b',
                '&:hover': { color: '#f87171', backgroundColor: 'rgba(239, 68, 68, 0.05)' },
              }}
            >
              清空
            </Button>
          </Box>
        </CardContent>
      </Card>

      {/* Log Table - Cursor dark style */}
      <Card sx={{ backgroundColor: '#1a1a1a', border: '1px solid #2a2a2a' }}>
        <TableContainer sx={{ maxHeight: 500 }}>
          <Table stickyHeader size="small">
            <TableHead>
              <TableRow>
                <TableCell
                  sx={{
                    backgroundColor: '#141414 !important',
                    color: '#71717a',
                    fontWeight: 600,
                    fontSize: '0.7rem',
                    fontFamily: '"JetBrains Mono", monospace',
                    letterSpacing: '0.05em',
                    textTransform: 'uppercase',
                    borderBottom: '1px solid #2a2a2a !important',
                  }}
                >
                  时间
                </TableCell>
                <TableCell
                  sx={{
                    backgroundColor: '#141414 !important',
                    color: '#71717a',
                    fontWeight: 600,
                    fontSize: '0.7rem',
                    fontFamily: '"JetBrains Mono", monospace',
                    letterSpacing: '0.05em',
                    textTransform: 'uppercase',
                    borderBottom: '1px solid #2a2a2a !important',
                  }}
                >
                  级别
                </TableCell>
                <TableCell
                  sx={{
                    backgroundColor: '#141414 !important',
                    color: '#71717a',
                    fontWeight: 600,
                    fontSize: '0.7rem',
                    fontFamily: '"JetBrains Mono", monospace',
                    letterSpacing: '0.05em',
                    textTransform: 'uppercase',
                    borderBottom: '1px solid #2a2a2a !important',
                  }}
                >
                  来源
                </TableCell>
                <TableCell
                  sx={{
                    backgroundColor: '#141414 !important',
                    color: '#71717a',
                    fontWeight: 600,
                    fontSize: '0.7rem',
                    fontFamily: '"JetBrains Mono", monospace',
                    letterSpacing: '0.05em',
                    textTransform: 'uppercase',
                    borderBottom: '1px solid #2a2a2a !important',
                  }}
                >
                  消息
                </TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {filteredEntries.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={4} align="center" sx={{ borderBottom: 'none' }}>
                    <Box sx={{ py: 4 }}>
                      <Typography
                        sx={{
                          color: '#52525b',
                          fontSize: '0.8rem',
                          fontFamily: '"JetBrains Mono", monospace',
                        }}
                      >
                        No log entries
                      </Typography>
                    </Box>
                  </TableCell>
                </TableRow>
              ) : (
                filteredEntries.map((entry) => {
                  const lc = levelConfig[entry.level] || levelConfig.info;
                  return (
                    <TableRow
                      key={entry.id}
                      sx={{
                        '&:hover': { backgroundColor: 'rgba(255, 255, 255, 0.02) !important' },
                      }}
                    >
                      <TableCell
                        sx={{
                          borderBottom: '1px solid rgba(255,255,255,0.04)',
                          color: '#52525b',
                          fontFamily: '"JetBrains Mono", monospace',
                          fontSize: '0.75rem',
                        }}
                      >
                        {new Date(entry.timestamp).toLocaleTimeString()}
                      </TableCell>
                      <TableCell sx={{ borderBottom: '1px solid rgba(255,255,255,0.04)' }}>
                        <Box
                          component="span"
                          sx={{
                            px: 1,
                            py: 0.25,
                            borderRadius: 0.75,
                            fontSize: '0.65rem',
                            fontFamily: '"JetBrains Mono", monospace',
                            fontWeight: 600,
                            letterSpacing: '0.05em',
                            backgroundColor: lc.bgColor,
                            color: lc.color,
                            border: '1px solid',
                            borderColor: lc.borderColor,
                          }}
                        >
                          {entry.level.toUpperCase()}
                        </Box>
                      </TableCell>
                      <TableCell
                        sx={{
                          borderBottom: '1px solid rgba(255,255,255,0.04)',
                          color: '#a1a1aa',
                          fontFamily: '"JetBrains Mono", monospace',
                          fontSize: '0.75rem',
                        }}
                      >
                        {entry.source}
                      </TableCell>
                      <TableCell
                        sx={{
                          borderBottom: '1px solid rgba(255,255,255,0.04)',
                          color: '#e4e4e7',
                          fontSize: '0.85rem',
                        }}
                      >
                        {entry.message}
                      </TableCell>
                    </TableRow>
                  );
                })
              )}
            </TableBody>
          </Table>
        </TableContainer>
      </Card>
    </Box>
  );
});

LogPage.displayName = 'LogPage';

export default LogPage;
