import { useState, useEffect } from 'react'

interface GPIOPin {
  pin: number
  state: 'high' | 'low' | 'input'
  mode: string
}

export default function GPIOView() {
  const [pins, setPins] = useState<GPIOPin[]>([])

  useEffect(() => {
    // TODO: 加载 GPIO 引脚
    setPins([
      { pin: 17, state: 'low', mode: 'output' },
      { pin: 27, state: 'high', mode: 'output' },
      { pin: 22, state: 'low', mode: 'output' },
      { pin: 23, state: 'high', mode: 'output' },
      { pin: 24, state: 'low', mode: 'output' },
      { pin: 25, state: 'high', mode: 'output' },
      { pin: 5, state: 'low', mode: 'output' },
      { pin: 6, state: 'high', mode: 'output' },
    ])
  }, [])

  const handleToggle = (pin: number) => {
    // TODO: 切换 GPIO 状态
    setPins(prev => prev.map(p => 
      p.pin === pin 
        ? { ...p, state: p.state === 'high' ? 'low' : 'high' }
        : p
    ))
  }

  return (
    <div className="bg-white rounded-lg shadow-sm border p-4">
      <h2 className="text-lg font-semibold text-gray-900 mb-4">GPIO 控制</h2>
      
      {/* 电源引脚 */}
      <div className="grid grid-cols-8 gap-2 mb-4">
        <div className="col-span-2 bg-red-500 text-white text-center py-2 rounded font-medium">3.3V</div>
        <div className="col-span-2 bg-black text-white text-center py-2 rounded font-medium">GND</div>
        <div className="col-span-2 bg-red-500 text-white text-center py-2 rounded font-medium">5V</div>
        <div className="col-span-2 bg-black text-white text-center py-2 rounded font-medium">GND</div>
      </div>
      
      {/* GPIO 引脚 */}
      <div className="grid grid-cols-4 gap-4">
        {pins.map(pin => (
          <div
            key={pin.pin}
            className="bg-gray-50 rounded-lg p-4 border hover:shadow-md transition-shadow"
          >
            <div className="text-center mb-3">
              <span className="text-lg font-bold text-gray-900">GPIO{pin.pin}</span>
            </div>
            
            <button
              onClick={() => handleToggle(pin.pin)}
              className={`w-full py-3 rounded-lg font-medium transition-colors ${
                pin.state === 'high'
                  ? 'bg-success text-white hover:bg-success/90'
                  : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
              }`}
            >
              {pin.state === 'high' ? 'HIGH' : 'LOW'}
            </button>
            
            <div className="mt-2 text-center">
              <span className="text-xs text-gray-500">{pin.mode}</span>
            </div>
          </div>
        ))}
      </div>
      
      {/* 批量操作 */}
      <div className="mt-6 flex gap-3">
        <button className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors">
          全部 HIGH
        </button>
        <button className="px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors">
          全部 LOW
        </button>
        <button className="px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors">
          导出配置
        </button>
        <button className="px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors">
          导入配置
        </button>
      </div>
    </div>
  )
}
