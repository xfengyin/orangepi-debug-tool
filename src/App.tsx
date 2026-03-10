import { useState } from 'react'
import SerialPanel from './components/SerialPanel'
import GPIOView from './components/GPIOView'
import PWMControl from './components/PWMControl'
import DataLogger from './components/DataLogger'
import ProtocolAnalyzer from './components/ProtocolAnalyzer'

function App() {
  const [activeTab, setActiveTab] = useState<'serial' | 'gpio' | 'pwm' | 'logger' | 'protocol'>('serial')

  return (
    <div className="min-h-screen bg-gray-50">
      {/* 顶部导航 */}
      <header className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 py-4">
          <h1 className="text-2xl font-bold text-gray-900">
            OrangePi Debug Tool <span className="text-primary">v2.0</span>
          </h1>
          <p className="text-sm text-gray-500 mt-1">
            现代化调试工具 - 串口调试 | GPIO 控制 | PWM 输出
          </p>
        </div>
      </header>

      {/* 标签页导航 */}
      <nav className="bg-white border-b">
        <div className="max-w-7xl mx-auto px-4">
          <div className="flex gap-6">
            <button
              onClick={() => setActiveTab('serial')}
              className={`py-4 px-2 border-b-2 font-medium text-sm transition-colors ${
                activeTab === 'serial'
                  ? 'border-primary text-primary'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              📡 串口调试
            </button>
            <button
              onClick={() => setActiveTab('gpio')}
              className={`py-4 px-2 border-b-2 font-medium text-sm transition-colors ${
                activeTab === 'gpio'
                  ? 'border-primary text-primary'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              🔌 GPIO 控制
            </button>
            <button
              onClick={() => setActiveTab('pwm')}
              className={`py-4 px-2 border-b-2 font-medium text-sm transition-colors ${
                activeTab === 'pwm'
                  ? 'border-primary text-primary'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              📈 PWM 输出
            </button>
            <button
              onClick={() => setActiveTab('logger')}
              className={`py-4 px-2 border-b-2 font-medium text-sm transition-colors ${
                activeTab === 'logger'
                  ? 'border-primary text-primary'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              📊 数据日志
            </button>
            <button
              onClick={() => setActiveTab('protocol')}
              className={`py-4 px-2 border-b-2 font-medium text-sm transition-colors ${
                activeTab === 'protocol'
                  ? 'border-primary text-primary'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              🔍 协议解析
            </button>
          </div>
        </div>
      </nav>

      {/* 主内容区 */}
      <main className="max-w-7xl mx-auto px-4 py-6">
        {activeTab === 'serial' && <SerialPanel />}
        {activeTab === 'gpio' && <GPIOView />}
        {activeTab === 'pwm' && <PWMControl />}
        {activeTab === 'logger' && <DataLogger />}
        {activeTab === 'protocol' && <ProtocolAnalyzer />}
      </main>

      {/* 底部状态栏 */}
      <footer className="fixed bottom-0 left-0 right-0 bg-white border-t py-2 px-4 text-sm text-gray-500">
        <div className="max-w-7xl mx-auto flex justify-between items-center">
          <span>OrangePi Debug Tool v2.0.0</span>
          <span>状态：就绪</span>
        </div>
      </footer>
    </div>
  )
}

export default App
