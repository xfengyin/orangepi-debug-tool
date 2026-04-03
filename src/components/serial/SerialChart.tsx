import React, { memo } from 'react';
import { Box, Typography } from '@mui/material';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from 'recharts';
import { useSerialStore } from '../../stores';

const SerialChart: React.FC = memo(() => {
  const { dataBuffer } = useSerialStore();

  // Parse numeric data from serial buffer
  const chartData = React.useMemo(() => {
    const data: { time: number; value: number }[] = [];
    let time = 0;

    dataBuffer.forEach((packet) => {
      if (packet.is_rx) {
        packet.data.forEach((byte) => {
          data.push({ time: time++, value: byte });
        });
      }
    });

    // Keep only last 100 points
    return data.slice(-100);
  }, [dataBuffer]);

  if (chartData.length === 0) {
    return (
      <Box sx={{ height: 200, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
        <Typography color="text.secondary">等待数据...</Typography>
      </Box>
    );
  }

  return (
    <Box sx={{ width: '100%', height: 200 }}>
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="time" hide />
          <YAxis domain={[0, 255]} />
          <Tooltip />
          <Line
            type="monotone"
            dataKey="value"
            stroke="#165DFF"
            strokeWidth={2}
            dot={false}
            isAnimationActive={false}
          />
        </LineChart>
      </ResponsiveContainer>
    </Box>
  );
});

SerialChart.displayName = 'SerialChart';

export default SerialChart;