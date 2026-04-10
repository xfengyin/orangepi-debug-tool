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
      <Box
        sx={{
          height: 200,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        <Typography
          sx={{
            color: '#52525b',
            fontSize: '0.8rem',
            fontFamily: '"JetBrains Mono", monospace',
          }}
        >
          Waiting for data...
        </Typography>
      </Box>
    );
  }

  return (
    <Box sx={{ width: '100%', height: 200 }}>
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={chartData}>
          <CartesianGrid
            strokeDasharray="3 3"
            stroke="rgba(255, 255, 255, 0.05)"
          />
          <XAxis dataKey="time" hide />
          <YAxis
            domain={[0, 255]}
            tick={{ fill: '#71717a', fontSize: 10, fontFamily: '"JetBrains Mono", monospace' }}
            axisLine={{ stroke: '#2a2a2a' }}
            tickLine={{ stroke: '#2a2a2a' }}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: '#1a1a1a',
              border: '1px solid #2a2a2a',
              borderRadius: 6,
              color: '#e4e4e7',
              fontSize: '0.75rem',
              fontFamily: '"JetBrains Mono", monospace',
            }}
          />
          <Line
            type="monotone"
            dataKey="value"
            stroke="url(#cursorGradient)"
            strokeWidth={2}
            dot={false}
            isAnimationActive={false}
          />
          <defs>
            <linearGradient id="cursorGradient" x1="0%" y1="0%" x2="100%" y2="0%">
              <stop offset="0%" stopColor="#7c3aed" />
              <stop offset="100%" stopColor="#3b82f6" />
            </linearGradient>
          </defs>
        </LineChart>
      </ResponsiveContainer>
    </Box>
  );
});

SerialChart.displayName = 'SerialChart';

export default SerialChart;
