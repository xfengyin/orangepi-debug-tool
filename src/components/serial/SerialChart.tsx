import React, { memo } from 'react';
import { Box } from '@mui/material';
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
import { monoFontFamily } from '../../theme';
import { useUiTokens } from '../../theme/useUiTokens';
import EmptyState from '../common/EmptyState';

const SerialChart: React.FC = memo(() => {
  const t = useUiTokens();
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
    return <EmptyState title="Waiting for data..." height={200} icon={null} />;
  }

  return (
    <Box sx={{ width: '100%', height: 200 }}>
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" stroke={t.chart.grid} />
          <XAxis dataKey="time" hide />
          <YAxis
            domain={[0, 255]}
            tick={{ fill: t.chart.axis, fontSize: 10, fontFamily: monoFontFamily }}
            axisLine={{ stroke: t.border }}
            tickLine={{ stroke: t.border }}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: t.chart.tooltipBg,
              border: `1px solid ${t.chart.tooltipBorder}`,
              borderRadius: 8,
              color: t.text,
              fontSize: '0.75rem',
              fontFamily: monoFontFamily,
            }}
          />
          <Line
            type="monotone"
            dataKey="value"
            stroke={t.chart.rx}
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
