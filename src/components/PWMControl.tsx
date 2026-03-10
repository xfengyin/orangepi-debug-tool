import { useState } from 'react'

export default function PWMControl() {
  const [channel, setChannel] = useState(0)
  const [frequency, setFrequency] = useState(1000)
  const [dutyCycle, setDutyCycle] = useState(50)
  const [enabled, setEnabled] = useState(false)

  const handleStart = () => {
    // TODO: 启动 PWM
    setEnabled(true)
  }

  const handleStop = () => {
    setEnabled(false)
  }

  return (
    <div className="bg-white rounded-lg shadow-sm border p-4">
      <h2 className="text-lg font-semibold text-gray-900 mb-4">PWM 控制</h2>
      
      <div className="grid grid-cols-2 gap-6">
        {/* 通道选择 */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            PWM 通道
          </label>
          <select
            value={channel}
            onChange={(e) => setChannel(Number(e.target.value))}
            className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
          >
            <option value={0}>PWM0</option>
            <option value={1}>PWM1</option>
          </select>
        </div>
        
        {/* 频率设置 */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            频率 (Hz)
          </label>
          <input
            type="number"
            value={frequency}
            onChange={(e) => setFrequency(Number(e.target.value))}
            className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
            min="1"
            max="50000000"
          />
          <p className="text-xs text-gray-500 mt-1">范围：1Hz - 50MHz</p>
        </div>
        
        {/* 占空比 */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            占空比 (%)
          </label>
          <input
            type="range"
            min="0"
            max="100"
            value={dutyCycle}
            onChange={(e) => setDutyCycle(Number(e.target.value))}
            className="w-full"
          />
          <div className="text-center text-lg font-bold text-primary mt-2">
            {dutyCycle}%
          </div>
        </div>
        
        {/* 预览 */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            波形预览
          </label>
          <div className="bg-gray-50 rounded-lg h-24 flex items-center justify-center">
            {/* TODO: 实际波形显示 */}
            <div className="text-center text-gray-400">
              波形预览区域
            </div>
          </div>
        </div>
      </div>
      
      {/* 控制按钮 */}
      <div className="mt-6 flex gap-3">
        {!enabled ? (
          <button
            onClick={handleStart}
            className="px-6 py-2 bg-success text-white rounded-lg hover:bg-success/90 transition-colors"
          >
            启动 PWM
          </button>
        ) : (
          <button
            onClick={handleStop}
            className="px-6 py-2 bg-danger text-white rounded-lg hover:bg-danger/90 transition-colors"
          >
            停止 PWM
          </button>
        )}
        
        <button className="px-6 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors">
          预设：正弦波
        </button>
        <button className="px-6 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors">
          预设：方波
        </button>
        <button className="px-6 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors">
          预设：三角波
        </button>
      </div>
    </div>
  )
}
