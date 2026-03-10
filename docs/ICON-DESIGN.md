# OrangePi Debug Tool - 应用图标设计

---

## 📐 图标规格

### 主图标 (SVG)

**文件:** `public/icon.svg`

**尺寸:** 512x512 px

**格式:** SVG (矢量)

---

## 🎨 设计理念

### 核心元素

1. **串口连接器 (DB9)**
   - 代表串口调试功能
   - 9 个引脚孔 (3x3 布局)
   - 白色主体，紫色引脚

2. **电路板纹理**
   - 背景电路图案
   - 象征电子设备调试

3. **信号波纹**
   - 表示数据传输
   - 动态视觉效果

4. **渐变背景**
   - 紫色系 (#6366f1 → #8b5cf6)
   - 现代科技感

5. **版本标识**
   - "v2.0" 徽章
   - 底部居中

---

## 📏 多尺寸导出

### Windows

| 文件 | 尺寸 | 用途 |
|------|------|------|
| `icons/32x32.png` | 32x32 | 任务栏 |
| `icons/128x128.png` | 128x128 | 应用列表 |
| `icons/256x256.png` | 256x256 | 程序文件 |
| `icons/icon.ico` | 多尺寸 | Windows 图标 |

### macOS

| 文件 | 尺寸 | 用途 |
|------|------|------|
| `icons/icon.icns` | 多尺寸 | macOS 图标 |
| `icons/512x512.png` | 512x512 | App Store |
| `icons/1024x1024.png` | 1024x1024 | 高分辨率 |

### Linux

| 文件 | 尺寸 | 用途 |
|------|------|------|
| `icons/32x32.png` | 32x32 | 桌面图标 |
| `icons/128x128.png` | 128x128 | 应用菜单 |
| `icons/256x256.png` | 256x256 | 高分辨率 |

---

## 🎨 配色方案

### 主色调

```
Primary:    #6366f1 (Indigo 500)
Secondary:  #8b5cf6 (Purple 500)
Background: 线性渐变 (45°)
```

### 辅助色

```
White:      #ffffff
Light Gray: #e2e8f0
Dark Gray:  #1e293b
```

---

## 📱 使用场景

### 桌面应用

- Windows 任务栏
- macOS Dock
- Linux 桌面

### 应用商店

- GitHub Releases
- 软件包管理器
- 应用列表

### 文档

- README 封面
- 用户手册
- 宣传材料

---

## 🔧 生成图标

### 使用 ImageMagick

```bash
# 从 SVG 生成 PNG
convert -background none icon.svg -resize 32x32 icons/32x32.png
convert -background none icon.svg -resize 128x128 icons/128x128.png
convert -background none icon.svg -resize 256x256 icons/256x256.png
convert -background none icon.svg -resize 512x512 icons/512x512.png
convert -background none icon.svg -resize 1024x1024 icons/1024x1024.png

# 生成 Windows ICO
convert icons/32x32.png icons/128x128.png icons/256x256.png icons/icon.ico

# 生成 macOS ICNS (需要 icnsutil)
icnsutil -o icons/icon.icns icons/
```

### 在线工具

- [CloudConvert](https://cloudconvert.com/svg-to-png)
- [ConvertICO](https://www.convertico.com/)
- [RealFaviconGenerator](https://realfavicongenerator.net/)

---

## ✅ 图标清单

- [x] SVG 主图标设计
- [ ] PNG 导出 (32/128/256/512/1024)
- [ ] Windows ICO 生成
- [ ] macOS ICNS 生成
- [ ] Linux PNG 生成

---

_OrangePi Debug Tool v2.0 - 图标设计_ ✨
