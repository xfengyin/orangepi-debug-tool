import { useThemeStore } from '../stores/themeStore';
import { tokensForMode } from './tokens';
import type { UiTokens } from './tokens';

/** 取当前模式的设计令牌（组件级样式统一入口） */
export const useUiTokens = (): UiTokens => {
  const mode = useThemeStore((s) => s.mode);
  return tokensForMode(mode);
};
