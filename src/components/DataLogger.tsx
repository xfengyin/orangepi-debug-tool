import { useState, useEffect } from 'react'
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts'

interface DataPoint {
  timestamp: number
  value: number
}

export default function DataLogger() {
  const [logging, setLogging] = useState(false)
  const [data, setData] = useState<DataPoint[]>([])
  const [exportFormat, setExportFormat] = useState<'csv' | 'json' | 'txt'>('csv')
  const [autoScroll, setAutoScroll] = useState(true)
  const [maxPoints, setMaxPoints] = useState(1000)
  const [samplingRate, setSamplingRate] = useState(100) // ms
  const [chartType, setChartType] = useState<'line' | 'bar' | 'scatter'>('line')
  const [showGrid, setShowGrid] = useState(true)
  const [autoScale, setAutoScale] = useState(true)
  const [yMin, setYMin] = useState<number | null>(null)
  const [yMax, setYMax] = useState<number | null>(null)

  useEffect(() => {
    let interval: NodeJS.Timeout | null = null
    
    if (logging) {
      interval = setInterval(() => {
        // TODO: 从串口读取数据
        const newValue = Math.random() * 100
        const newPoint = {
          timestamp: Date.now() / 1000,
          value: newValue
        }
        
        setData(prev => {
          const newData = [...prev, newPoint]
          if (newData.length > maxPoints) {
            newData.shift()
          }
          return newData
        })
      }, samplingRate)
    }
    
    return () => {
      if (interval) clearInterval(interval)
    }
  }, [logging, samplingRate, maxPoints])

  const handleStart = () => {
    setLogging(true)
    setData([])
  }

  const handleStop = () => {
    setLogging(false)
  }

  const handleExport = async (format: 'csv' | 'json' | 'txt') => {
    // TODO: 调用 Tauri API 导出
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `data_${new Date().getTime()}.${format}`
    a.click()
    URL.revokeObjectURL(url)
  }

  const handleClear = () => {
    setData([])
  }

  return (
    <div className="space-y-4">
      {/* 控制区 */}
      <div className="bg-white rounded-lg shadow-sm border p-4">
        <div className="flex flex-wrap gap-3 mb-4">
          {!logging ? (
            <button
              onClick={handleStart}
              className="px-6 py-2 bg-success text-white rounded-lg hover:bg-success/90 transition-colors"
            >
              ▶️ 开始记录
            </button>
          ) : (
            <button
              onClick={handleStop}
              className="px-6 py-2 bg-danger text-white rounded-lg hover:bg-danger/90 transition-colors"
            >
              ⏹️ 停止记录
            </button>
          )}
          
          <button
            onClick={() => handleExport(exportFormat)}
            disabled={data.length === 0}
            className="px-6 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            📥 导出 {exportFormat.toUpperCase()}
          </button>
          
          <select
            value={exportFormat}
            onChange={(e) => setExportFormat(e.target.value as 'csv' | 'json' | 'txt')}
            className="px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
          >
            <option value="csv">CSV</option>
            <option value="json">JSON</option>
            <option value="txt">TXT</option>
          </select>
          
          <button
            onClick={handleClear}
            disabled={data.length === 0}
            className="px-6 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            🗑️ 清空数据
          </button>
        </div>
        
        {/* 设置 */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              采样率 (ms)
            </label>
            <select
              value={samplingRate}
              onChange={(e) => setSamplingRate(Number(e.target.value))}
              disabled={logging}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            >
              <option value="10">10ms</option>
              <option value="50">50ms</option>
              <option value="100">100ms</option>
              <option value="200">200ms</option>
              <option value="500">500ms</option>
              <option value="1000">1000ms</option>
            </select>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              最大数据点
            </label>
            <select
              value={maxPoints}
              onChange={(e) => setMaxPoints(Number(e.target.value))}
              disabled={logging}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            >
              <option value="100">100</option>
              <option value="500">500</option>
              <option value="1000">1000</option>
              <option value="5000">5000</option>
              <option value="10000">10000</option>
            </select>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              图表类型
            </label>
            <select
              value={chartType}
              onChange={(e) => setChartType(e.target.value as 'line' | 'bar' | 'scatter')}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            >
              <option value="line">折线图</option>
              <option value="bar">柱状图</option>
              <option value="scatter">散点图</option>
            </select>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Y 轴范围
            </label>
            <div className="flex gap-2">
              <input
                type="number"
                placeholder="Min"
                value={yMin ?? ''}
                onChange={(e) => setYMin(e.target.value ? Number(e.target.value) : null)}
                className="w-full px-2 py-2 border border-gray-300 rounded-lg text-sm"
              />
              <input
                type="number"
                placeholder="Max"
                value={yMax ?? ''}
                onChange={(e) => setYMax(e.target.value ? Number(e.target.value) : null)}
                className="w-full px-2 py-2 border border-gray-300 rounded-lg text-sm"
              />
            </div>
          </div>
        </div>
        
        {/* 选项 */}
        <div className="mt-4 flex gap-6">
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={autoScroll}
              onChange={(e) => setAutoScroll(e.target.checked)}
              className="rounded border-gray-300 text-primary focus:ring-primary"
            />
            <span className="text-sm text-gray-700">自动滚动</span>
          </label>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={showGrid}
              onChange={(e) => setShowGrid(e.target.checked)}
              className="rounded border-gray-300 text-primary focus:ring-primary"
            />
            <span className="text-sm text-gray-700">显示网格</span>
          </label>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={autoScale}
              onChange={(e) => setAutoScale(e.target.checked)}
              className="rounded border-gray-300 text-primary focus:ring-primary"
            />
            <span className="text-sm text-gray-700">自动缩放</span>
          </label>
        </div>
      </div>

      {/* 状态 */}
      <div className="bg-white rounded-lg shadow-sm border p-4">
        <div className="flex justify-between items-center">
          <div className="flex gap-6">
            <div>
              <span className="text-sm text-gray-600">记录状态：</span>
              <span className={`font-medium ${logging ? 'text-success' : 'text-gray-400'}`}>
                {logging ? '● 记录中' : '○ 已停止'}
              </span>
            </div>
            <div>
              <span className="text-sm text-gray-600">数据点数：</span>
              <span className="font-medium text-gray-900">{data.length}</span>
            </div>
            <div>
              <span className="text-sm text-gray-600">采样率：</span>
              <span className="font-medium text-gray-900">{samplingRate}ms</span>
            </div>
          </div>
          <div className="text-sm text-gray-500">
            记录时长：{((data.length * samplingRate) / 1000).toFixed(1)}s
          </div>
        </div>
      </div>

      {/* 图表区 */}
      <div className="bg-white rounded-lg shadow-sm border p-4">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">实时波形</h2>
        <div className="h-80">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={data}>
              {showGrid && <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />}
              <XAxis
                dataKey="timestamp"
                tickFormatter={(value) => ((value - (data[0]?.timestamp || 0)).toFixed(1))}
                label={{ value: '时间 (s)', position: 'insideBottom', offset: -5 }}
              />
              <YAxis
                domain={[
                  autoScale ? 'auto' : (yMin ?? 0),
                  autoScale ? 'auto' : (yMax ?? 100)
                ]}
                label={{ value: '数值', angle: -90, position: 'insideLeft' }}
              />
              <Tooltip
                labelFormatter={(value) => `时间：${((value - (data[0]?.timestamp || 0)).toFixed(3))}s`}
                contentStyle={{
                  backgroundColor: '#F8FAFC',
                  border: '1px solid #E2E8F0',
                  borderRadius: '6px',
                }}
              />
              <Legend />
              <Line
                type="monotone"
                dataKey="value"
                stroke="#6366f1"
                strokeWidth={2}
                dot={false}
                name="数值"
                isAnimationActive={false}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </div>

      {/* 数据表格 */}
      <div className="bg-white rounded-lg shadow-sm border p-4">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">数据列表</h2>
        <div className="max-h-64 overflow-y-auto">
          <table className="w-full">
            <thead className="bg-gray-50 sticky top-0">
              <tr>
                <th className="px-4 py-2 text-left text-sm font-medium text-gray-700">#</th>
                <th className="px-4 py-2 text-left text-sm font-medium text-gray-700">时间戳</th>
                <th className="px-4 py-2 text-left text-sm font-medium text-gray-700">相对时间 (s)</th>
                <th className="px-4 py-2 text-left text-sm font-medium text-gray-700">数值</th>
              </tr>
            </thead>
            <tbody>
              {data.length === 0 ? (
                <tr>
                  <td colSpan={4} className="px-4 py-8 text-center text-gray-400">
                    暂无数据
                  </td>
                </tr>
              ) : (
                data.slice(-20).reverse().map((row, idx) => (
                  <tr key={idx} className="border-t hover:bg-gray-50">
                    <td className="px-4 py-2 text-sm text-gray-900">{data.length - idx}</td>
                    <td className="px-4 py-2 text-sm text-gray-900">
                      {new Date(row.timestamp * 1000).toLocaleTimeString()}
                    </td>
                    <td className="px-4 py-2 text-sm text-gray-900">
                      {(row.timestamp - (data[0]?.timestamp || 0)).toFixed(3)}
                    </td>
                    <td className="px-4 py-2 text-sm text-gray-900 font-mono">
                      {row.value.toFixed(4)}
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  )
}
