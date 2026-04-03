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

const levelColors: Record<string, 'default' | 'info' | 'success' | 'warning' | 'error'> = {
  debug: 'default',
  info: 'info',
  warn: 'warning',
  error: 'error',
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
      <Typography variant="h5" gutterBottom>
        数据日志
      </Typography>

      {/* Filters */}
      <Card sx={{ mb: 2 }}>
        <CardContent>
          <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap' }}>
            <FormControl size="small" sx={{ minWidth: 120 }}>
              <InputLabel>日志级别</InputLabel>
              <Select
                multiple
                value={filter.levels}
                onChange={(e) => setFilter({ levels: e.target.value as string[] })}
                renderValue={(selected) => (selected as string[]).join(', ')}
              >
                {['debug', 'info', 'warn', 'error'].map((level) => (
                  <MenuItem key={level} value={level}>
                    {level.toUpperCase()}
                  </MenuItem>
                ))}
              </Select>
            </FormControl>

            <TextField
              size="small"
              placeholder="搜索..."
              value={filter.search}
              onChange={(e) => setFilter({ search: e.target.value })}
            />

            <Box sx={{ flexGrow: 1 }} />

            <Button startIcon={<DownloadIcon />} onClick={handleExport}>
              导出
            </Button>
            <Button startIcon={<DeleteIcon />} onClick={clearLogs} color="error">
              清空
            </Button>
          </Box>
        </CardContent>
      </Card>

      {/* Log Table */}
      <Card>
        <TableContainer sx={{ maxHeight: 500 }}>
          <Table stickyHeader size="small">
            <TableHead>
              <TableRow>
                <TableCell>时间</TableCell>
                <TableCell>级别</TableCell>
                <TableCell>来源</TableCell>
                <TableCell>消息</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {filteredEntries.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={4} align="center">
                    <Typography color="text.secondary">暂无日志</Typography>
                  </TableCell>
                </TableRow>
              ) : (
                filteredEntries.map((entry) => (
                  <TableRow key={entry.id}>
                    <TableCell>
                      {new Date(entry.timestamp).toLocaleTimeString()}
                    </TableCell>
                    <TableCell>
                      <Chip
                        size="small"
                        label={entry.level.toUpperCase()}
                        color={levelColors[entry.level]}
                      />
                    </TableCell>
                    <TableCell>{entry.source}</TableCell>
                    <TableCell>{entry.message}</TableCell>
                  </TableRow>
                ))
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