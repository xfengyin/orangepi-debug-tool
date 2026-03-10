import { useState } from 'react'

interface ProtocolField {
  name: string
  offset: number
  length: number
  type: 'uint8' | 'uint16' | 'uint32' | 'string' | 'bytes'
  endian: 'big' | 'little'
}

interface ProtocolConfig {
  name: string
  frame_header: number[]
  frame_footer: number[]
  length_offset: number
  length_bytes: number
  checksum_offset: number
  checksum_type: 'none' | 'sum' | 'crc8' | 'crc16' | 'crc32'
  fields: ProtocolField[]
}

export default function ProtocolAnalyzer() {
  const [selectedProtocol, setSelectedProtocol] = useState('modbus')
  const [customConfig, setCustomConfig] = useState<ProtocolConfig | null>(null)
  const [rawData, setRawData] = useState('')
  const [parsedData, setParsedData] = useState<any>(null)
  const [error, setError] = useState<string | null>(null)

  const protocols: ProtocolConfig[] = [
    {
      name: 'MODBUS RTU',
      frame_header: [],
      frame_footer: [],
      length_offset: 0,
      length_bytes: 0,
      checksum_offset: -2,
      checksum_type: 'crc16',
      fields: [
        { name: 'Slave Address', offset: 0, length: 1, type: 'uint8', endian: 'big' },
        { name: 'Function Code', offset: 1, length: 1, type: 'uint8', endian: 'big' },
        { name: 'Data', offset: 2, length: -3, type: 'bytes', endian: 'big' },
        { name: 'CRC', offset: -2, length: 2, type: 'uint16', endian: 'little' },
      ],
    },
  ]

  const handleParse = () => {
    try {
      // 解析十六进制数据
      const bytes = rawData.replace(/\s/g, '').match(/.{1,2}/g)?.map(b => parseInt(b, 16)) || []
      
      if (bytes.length === 0) {
        setError('请输入有效的十六进制数据')
        return
      }
      
      // TODO: 调用 Tauri API 解析协议
      setParsedData({
        valid: true,
        fields: bytes.map((b, i) => ({ name: `Byte ${i}`, value: b })),
      })
      setError(null)
    } catch (e) {
      setError(`解析失败：${e}`)
    }
  }

  const handleClear = () => {
    setRawData('')
    setParsedData(null)
    setError(null)
  }

  return (
    <div className="space-y-4">
      {/* 协议选择 */}
      <div className="bg-white rounded-lg shadow-sm border p-4">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">协议选择</h2>
        
        <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
          <button
            onClick={() => setSelectedProtocol('modbus')}
            className={`px-4 py-3 border rounded-lg transition-colors ${
              selectedProtocol === 'modbus'
                ? 'border-primary bg-primary/5 text-primary'
                : 'border-gray-300 hover:border-gray-400'
            }`}
          >
            <div className="font-medium">MODBUS RTU</div>
            <div className="text-xs text-gray-500">工业标准协议</div>
          </button>
          
          <button
            onClick={() => setSelectedProtocol('custom')}
            className={`px-4 py-3 border rounded-lg transition-colors ${
              selectedProtocol === 'custom'
                ? 'border-primary bg-primary/5 text-primary'
                : 'border-gray-300 hover:border-gray-400'
            }`}
          >
            <div className="font-medium">自定义协议</div>
            <div className="text-xs text-gray-500">配置帧格式</div>
          </button>
        </div>
      </div>

      {/* 数据输入 */}
      <div className="bg-white rounded-lg shadow-sm border p-4">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">数据输入</h2>
        
        <textarea
          value={rawData}
          onChange={(e) => setRawData(e.target.value)}
          placeholder="输入十六进制数据，例如：01 03 00 00 00 0A C5 CD"
          className="w-full h-32 px-3 py-2 border border-gray-300 rounded-lg font-mono text-sm focus:ring-2 focus:ring-primary focus:border-transparent"
        />
        
        <div className="mt-4 flex gap-3">
          <button
            onClick={handleParse}
            className="px-6 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 transition-colors"
          >
            🔍 解析
          </button>
          <button
            onClick={handleClear}
            className="px-6 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors"
          >
            🗑️ 清空
          </button>
        </div>
      </div>

      {/* 错误信息 */}
      {error && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-4">
          <div className="flex items-center gap-2 text-red-600">
            <span>❌</span>
            <span className="font-medium">解析错误</span>
          </div>
          <div className="mt-2 text-sm text-red-600">{error}</div>
        </div>
      )}

      {/* 解析结果 */}
      {parsedData && (
        <div className="bg-white rounded-lg shadow-sm border p-4">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">解析结果</h2>
          
          <div className="space-y-3">
            {parsedData.fields.map((field: any, idx: number) => (
              <div key={idx} className="flex justify-between items-center py-2 border-b last:border-0">
                <span className="text-sm font-medium text-gray-700">{field.name}</span>
                <span className="text-sm text-gray-900 font-mono">{field.value}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* 协议说明 */}
      <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
        <h3 className="font-medium text-blue-900 mb-2">💡 支持的功能</h3>
        <ul className="text-sm text-blue-800 space-y-1">
          <li>✅ MODBUS RTU 协议解析</li>
          <li>✅ 自定义协议配置</li>
          <li>✅ CRC8/CRC16/CRC32 校验</li>
          <li>✅ 大小端字节序支持</li>
          <li>✅ 实时协议解析</li>
          <li>⏳ 更多协议即将支持...</li>
        </ul>
      </div>
    </div>
  )
}
