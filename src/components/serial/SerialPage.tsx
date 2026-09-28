import React, { useState, memo } from 'react';
import { Box, Card, CardContent, Grid } from '@mui/material';
import ConnectionPanel from './ConnectionPanel';
import SendPanel from './SendPanel';
import SerialTerminal from './SerialTerminal';
import SerialChart from './SerialChart';
import CommandPanel from './CommandPanel';

/** 串口调试页：连接面板 / 图表 / 终端 + 快捷指令 / 发送面板 */
const SerialPage: React.FC = memo(() => {
  const [showChart, setShowChart] = useState(false);

  return (
    <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column', gap: 2 }}>
      <ConnectionPanel showChart={showChart} onToggleChart={() => setShowChart((v) => !v)} />

      {/* Chart Panel */}
      {showChart && (
        <Card>
          <CardContent>
            <SerialChart />
          </CardContent>
        </Card>
      )}

      {/* Terminal & Command Panel */}
      <Grid container spacing={2} sx={{ flexGrow: 1, minHeight: 0 }}>
        <Grid item xs={12} md={8} sx={{ height: '100%' }}>
          <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
            <CardContent sx={{ flexGrow: 1, p: 0, overflow: 'hidden' }}>
              <SerialTerminal />
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={4} sx={{ height: '100%' }}>
          <CommandPanel />
        </Grid>
      </Grid>

      <SendPanel />
    </Box>
  );
});

SerialPage.displayName = 'SerialPage';

export default SerialPage;
