import { useState, useEffect } from 'react'

interface SerialStats {
  tx_bytes: number
  rx_bytes: number
  tx_frames: number
  rx_frames: number
  errors: number
}

interface QuickCommand {
  name: string
  data: string
  hex_mode: boolean
  interval_ms?: number
}

export default function SerialPanel() {
  const [ports, setPorts] = useState<Array<{ name: string; description: string }>>([])
  const [selectedPort, setSelectedPort] = useState('')
  const [baudrate, setBaudrate] = useState(115200)
  const [dataBits, setDataBits] = useState(8)
  const [parity, setParity] = useState('None')
  const [stopBits, setStopBits] = useState(1)
  const [flowControl, setFlowControl] = useState(false)
  const [connected, setConnected] = useState(false)
  const [messages, setMessages] = useState<Array<{ time: string; type: 'tx' | 'rx'; data: string }>>([])
  const [sendData, setSendData] = useState('')
  const [hexMode, setHexMode] = useState(false)
  const [showTimestamp, setShowTimestamp] = useState(true)
  const [autoScroll, setAutoScroll] = useState(true)
  const [appendNewline, setAppendNewline] = useState(false)
  const [appendCr, setAppendCr] = useState(false)
  const [stats, setStats] = useState<SerialStats>({ tx_bytes: 0, rx_bytes: 0, tx_frames: 0, rx_frames: 0, errors: 0 })
  const [showStats, setShowStats] = useState(true)
  const [quickCommands, setQuickCommands] = useState<QuickCommand[]>([])
  const [autoSendInterval, setAutoSendInterval] = useState<number | null>(null)

  // 加载可用串口
  useEffect(() => {
    // TODO: 调用 Tauri API list_ports()
    setPorts([
      { name: '/dev/ttyUSB0', description: 'USB Serial' },
      { name: '/dev/ttyS0', description: 'UART0' },
      { name: '/dev/ttyAMA0', description: 'PL011 UART' },
    ])
  }, [])

  const handleConnect = async () => {
    // TODO: 调用 Tauri API connect_serial()
    setConnected(true)
  }

  const handleDisconnect = () => {
    setConnected(false)
    setStats({ tx_bytes: 0, rx_bytes: 0, tx_frames: 0, rx_frames: 0, errors: 0 })
  }

  const handleSend = () => {
    if (!sendData.trim()) return
    
    const timestamp = new Date().toLocaleTimeString()
    setMessages(prev => [...prev, {
      time: timestamp,
      type: 'tx',
      data: sendData
    }])
    setSendData('')
    
    // TODO: 调用 Tauri API send_data()
  }

  const handleClear = () => {
    setMessages([])
  }

  const handleAddQuickCommand = () => {
    const name = prompt('命令名称:')
    if (!name) return
    
    const data = prompt('数据:')
    if (!data) return
    
    // TODO: 调用 Tauri API add_quick_command()
    setQuickCommands(prev => [...prev, { name, data, hex_mode: hexMode }])
  }

  const handleExecuteQuickCommand = (index: number) => {
    const cmd = quickCommands[index]
    // TODO: 调用 Tauri API execute_quick_command()
    setMessages(prev => [...prev, {
      time: new Date().toLocaleTimeString(),
      type: 'tx',
      data: `[快捷命令] ${cmd.name}: ${cmd.data}`
    }])
  }

  const handleStartAutoSend = () => {
    const interval = prompt('发送间隔 (ms):', '1000')
    if (!interval) return
    
    setAutoSendInterval(Number(interval))
    // TODO: 调用 Tauri API start_auto_send()
  }

  const handleStopAutoSend = () => {
    setAutoSendInterval(null)
    // TODO: 调用 Tauri API stop_auto_send()
  }

  return (
    <div className="space-y-4">
      {/* 串口配置 */}
      <div className="bg-white rounded-lg shadow-sm border p-4">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">串口配置</h2>
        
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              串口端口
            </label>
            <select
              value={selectedPort}
              onChange={(e) => setSelectedPort(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
              disabled={connected}
            >
              <option value="">选择端口</option>
              {ports.map(port => (
                <option key={port.name} value={port.name}>
                  {port.name}
                </option>
              ))}
            </select>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              波特率
            </label>
            <select
              value={baudrate}
              onChange={(e) => setBaudrate(Number(e.target.value))}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
              disabled={connected}
            >
              <option value="1200">1200</option>
              <option value="2400">2400</option>
              <option value="4800">4800</option>
              <option value="9600">9600</option>
              <option value="19200">19200</option>
              <option value="38400">38400</option>
              <option value="57600">57600</option>
              <option value="115200">115200</option>
              <option value="230400">230400</option>
              <option value="460800">460800</option>
              <option value="921600">921600</option>
            </select>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              数据位
            </label>
            <select
              value={dataBits}
              onChange={(e) => setDataBits(Number(e.target.value))}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
              disabled={connected}
            >
              <option value="5">5</option>
              <option value="6">6</option>
              <option value="7">7</option>
              <option value="8">8</option>
            </select>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              校验位
            </label>
            <select
              value={parity}
              onChange={(e) => setParity(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
              disabled={connected}
            >
              <option value="None">None</option>
              <option value="Even">Even</option>
              <option value="Odd">Odd</option>
              <option value="Mark">Mark</option>
              <option value="Space">Space</option>
            </select>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              停止位
            </label>
            <select
              value={stopBits}
              onChange={(e) => setStopBits(Number(e.target.value))}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
              disabled={connected}
            >
              <option value="1">1</option>
              <option value="1.5">1.5</option>
              <option value="2">2</option>
            </select>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              流控制
            </label>
            <select
              value={flowControl ? 'Hardware' : 'None'}
              onChange={(e) => setFlowControl(e.target.value === 'Hardware')}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
              disabled={connected}
            >
              <option value="None">None</option>
              <option value="Hardware">Hardware (RTS/CTS)</option>
              <option value="Software">Software (XON/XOFF)</option>
            </select>
          </div>
        </div>
        
        <div className="mt-4 flex gap-3">
          {!connected ? (
            <button
              onClick={handleConnect}
              disabled={!selectedPort}
              className="px-6 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              连接
            </button>
          ) : (
            <>
              <button
                onClick={handleDisconnect}
                className="px-6 py-2 bg-danger text-white rounded-lg hover:bg-danger/90 transition-colors"
              >
                断开
              </button>
              <button
                onClick={() => setStats({ tx_bytes: 0, rx_bytes: 0, tx_frames: 0, rx_frames: 0, errors: 0 })}
                className="px-6 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors"
              >
                重置统计
              </button>
            </>
          )}
        </div>
      </div>

      {/* 统计信息 */}
      {showStats && connected && (
        <div className="bg-white rounded-lg shadow-sm border p-4">
          <div className="flex justify-between items-center mb-4">
            <h2 className="text-lg font-semibold text-gray-900">统计信息</h2>
            <button
              onClick={() => setShowStats(false)}
              className="text-gray-400 hover:text-gray-600"
            >
              ✕
            </button>
          </div>
          
          <div className="grid grid-cols-5 gap-4">
            <div className="text-center">
              <div className="text-sm text-gray-600">发送字节</div>
              <div className="text-2xl font-bold text-primary">{stats.tx_bytes.toLocaleString()}</div>
            </div>
            <div className="text-center">
              <div className="text-sm text-gray-600">接收字节</div>
              <div className="text-2xl font-bold text-success">{stats.rx_bytes.toLocaleString()}</div>
            </div>
            <div className="text-center">
              <div className="text-sm text-gray-600">发送帧数</div>
              <div className="text-2xl font-bold text-primary">{stats.tx_frames.toLocaleString()}</div>
            </div>
            <div className="text-center">
              <div className="text-sm text-gray-600">接收帧数</div>
              <div className="text-2xl font-bold text-success">{stats.rx_frames.toLocaleString()}</div>
            </div>
            <div className="text-center">
              <div className="text-sm text-gray-600">错误数</div>
              <div className="text-2xl font-bold text-danger">{stats.errors.toLocaleString()}</div>
            </div>
          </div>
        </div>
      )}

      {/* 数据收发区 */}
      <div className="bg-white rounded-lg shadow-sm border p-4">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-semibold text-gray-900">数据收发</h2>
          <div className="flex gap-2">
            <button
              onClick={handleClear}
              className="px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors text-sm"
            >
              清空接收区
            </button>
            <button
              onClick={() => setShowStats(!showStats)}
              className="px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors text-sm"
            >
              {showStats ? '隐藏统计' : '显示统计'}
            </button>
          </div>
        </div>
        
        {/* 接收区 */}
        <div className="bg-gray-50 rounded-lg p-4 h-64 overflow-y-auto font-mono text-sm mb-4">
          {messages.length === 0 ? (
            <p className="text-gray-400 text-center">暂无数据</p>
          ) : (
            messages.map((msg, idx) => (
              <div key={idx} className="mb-2">
                {showTimestamp && (
                  <span className="text-gray-400 text-xs mr-2">[{msg.time}]</span>
                )}
                <span className={msg.type === 'tx' ? 'text-blue-600' : 'text-green-600'}>
                  {msg.type === 'tx' ? 'TX › ' : 'RX ‹ '}
                </span>
                <span className="text-gray-900">{msg.data}</span>
              </div>
            ))
          )}
        </div>
        
        {/* 发送区 */}
        <div className="flex gap-3 mb-4">
          <input
            type="text"
            value={sendData}
            onChange={(e) => setSendData(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && handleSend()}
            placeholder="输入要发送的数据..."
            className="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
          />
          <button
            onClick={handleSend}
            disabled={!connected}
            className="px-6 py-2 bg-success text-white rounded-lg hover:bg-success/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            发送
          </button>
        </div>
        
        {/* 选项 */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={hexMode}
              onChange={(e) => setHexMode(e.target.checked)}
              className="rounded border-gray-300 text-primary focus:ring-primary"
            />
            <span className="text-sm text-gray-700">Hex 模式</span>
          </label>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={showTimestamp}
              onChange={(e) => setShowTimestamp(e.target.checked)}
              className="rounded border-gray-300 text-primary focus:ring-primary"
            />
            <span className="text-sm text-gray-700">显示时间戳</span>
          </label>
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
              checked={appendNewline}
              onChange={(e) => setAppendNewline(e.target.checked)}
              className="rounded border-gray-300 text-primary focus:ring-primary"
            />
            <span className="text-sm text-gray-700">添加 LF</span>
          </label>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={appendCr}
              onChange={(e) => setAppendCr(e.target.checked)}
              className="rounded border-gray-300 text-primary focus:ring-primary"
            />
            <span className="text-sm text-gray-700">添加 CR</span>
          </label>
        </div>
      </div>

      {/* 快捷命令 */}
      <div className="bg-white rounded-lg shadow-sm border p-4">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-semibold text-gray-900">快捷命令</h2>
          <div className="flex gap-2">
            <button
              onClick={handleAddQuickCommand}
              className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors text-sm"
            >
              添加命令
            </button>
            {autoSendInterval ? (
              <button
                onClick={handleStopAutoSend}
                className="px-4 py-2 bg-danger text-white rounded-lg hover:bg-danger/90 transition-colors text-sm"
              >
                停止定时发送
              </button>
            ) : (
              <button
                onClick={handleStartAutoSend}
                className="px-4 py-2 bg-success text-white rounded-lg hover:bg-success/90 transition-colors text-sm"
              >
                定时发送
              </button>
            )}
          </div>
        </div>
        
        <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
          {quickCommands.map((cmd, idx) => (
            <button
              key={idx}
              onClick={() => handleExecuteQuickCommand(idx)}
              className="px-4 py-3 bg-gray-50 border rounded-lg hover:bg-gray-100 transition-colors text-left"
            >
              <div className="font-medium text-gray-900">{cmd.name}</div>
              <div className="text-xs text-gray-500 truncate">{cmd.data}</div>
              {cmd.interval_ms && (
                <div className="text-xs text-primary mt-1">每 {cmd.interval_ms}ms</div>
              )}
            </button>
          ))}
        </div>
      </div>
    </div>
  )
}
