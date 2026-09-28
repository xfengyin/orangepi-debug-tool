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
  Button,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
} from '@mui/material';
import { Delete as DeleteIcon, Download as DownloadIcon } from '@mui/icons-material';
import { useLogStore } from '../../stores';
import type { LogEntry } from '../../types';
import { monoFontFamily } from '../../theme';
import { useUiTokens } from '../../theme/useUiTokens';
import type { SemanticColor } from '../../theme';
import PageHeader from '../common/PageHeader';
import EmptyState from '../common/EmptyState';

const LOG_LEVELS = ['debug', 'info', 'warn', 'error'] as const;
const TABLE_COLUMNS = ['时间', '级别', '来源', '消息'] as const;

const LogPage: React.FC = memo(() => {
  const t = useUiTokens();
  const {
    entries,
    filter,
    clearLogs,
    exportLogs,
    setFilter,
    getFilteredEntries,
  } = useLogStore();

  const levelColorMap: Record<LogEntry['level'], SemanticColor> = {
    debug: { main: t.textMuted, soft: t.surfaceAlt },
    info: t.info,
    warn: t.warning,
    error: t.danger,
  };

  // getFilteredEntries 的结果由 entries/filter 决定，二者变化时已触发重算；
  // 函数来自 store（引用稳定），故只声明真实数据依赖。
  // eslint-disable-next-line react-hooks/exhaustive-deps
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
      <PageHeader title="数据日志" />

      {/* Filters */}
      <Card sx={{ mb: 2 }}>
        <CardContent sx={{ py: 1.5, '&:last-child': { pb: 1.5 } }}>
          <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap', alignItems: 'center' }}>
            <FormControl size="small" sx={{ minWidth: 120 }}>
              <InputLabel>日志级别</InputLabel>
              <Select
                multiple
                value={filter.levels}
                onChange={(e) => setFilter({ levels: e.target.value as string[] })}
                renderValue={(selected) => (
                  <Typography sx={{ fontFamily: monoFontFamily, fontSize: '0.8rem' }}>
                    {(selected as string[]).join(', ')}
                  </Typography>
                )}
              >
                {LOG_LEVELS.map((level) => (
                  <MenuItem key={level} value={level}>
                    <Typography sx={{ fontFamily: monoFontFamily, fontSize: '0.8rem' }}>
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

            <Button startIcon={<DownloadIcon />} onClick={handleExport}>
              导出
            </Button>
            <Button
              startIcon={<DeleteIcon />}
              onClick={clearLogs}
              sx={{
                color: t.textMuted,
                '&:hover': { color: t.danger.main, backgroundColor: t.danger.soft },
              }}
            >
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
                {TABLE_COLUMNS.map((col) => (
                  <TableCell
                    key={col}
                    sx={{
                      fontSize: '0.7rem',
                      fontFamily: monoFontFamily,
                      letterSpacing: '0.05em',
                      textTransform: 'uppercase',
                    }}
                  >
                    {col}
                  </TableCell>
                ))}
              </TableRow>
            </TableHead>
            <TableBody>
              {filteredEntries.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={4} sx={{ borderBottom: 'none' }}>
                    <EmptyState title="No log entries" height={160} icon={null} />
                  </TableCell>
                </TableRow>
              ) : (
                filteredEntries.map((entry) => {
                  const lc = levelColorMap[entry.level] ?? t.info;
                  return (
                    <TableRow key={entry.id}>
                      <TableCell
                        sx={{
                          color: t.textMuted,
                          fontFamily: monoFontFamily,
                          fontSize: '0.75rem',
                        }}
                      >
                        {new Date(entry.timestamp).toLocaleTimeString()}
                      </TableCell>
                      <TableCell>
                        <Box
                          component="span"
                          sx={{
                            px: 1,
                            py: 0.25,
                            borderRadius: 0.75,
                            fontSize: '0.65rem',
                            fontFamily: monoFontFamily,
                            fontWeight: 600,
                            letterSpacing: '0.05em',
                            backgroundColor: lc.soft,
                            color: lc.main,
                            border: `1px solid ${lc.soft}`,
                          }}
                        >
                          {entry.level.toUpperCase()}
                        </Box>
                      </TableCell>
                      <TableCell
                        sx={{
                          color: t.textSecondary,
                          fontFamily: monoFontFamily,
                          fontSize: '0.75rem',
                        }}
                      >
                        {entry.source}
                      </TableCell>
                      <TableCell sx={{ color: t.text, fontSize: '0.85rem' }}>
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
