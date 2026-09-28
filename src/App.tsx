import React, { useEffect, useCallback } from 'react';
import { Box, ThemeProvider, CssBaseline, Snackbar, Alert } from '@mui/material';
import { useThemeStore, useAppStore, useLogStore } from './stores';
import Sidebar from './components/layout/Sidebar';
import CommandPalette from './components/layout/CommandPalette';
import Header from './components/layout/Header';
import StatusBar from './components/layout/StatusBar';
import { DRAWER_WIDTH } from './components/layout/constants';
import OverviewPage from './components/overview/OverviewPage';
import SerialPage from './components/serial/SerialPage';
import GpioPage from './components/gpio/GpioPage';
import PwmPage from './components/pwm/PwmPage';
import LogPage from './components/log/LogPage';
import SettingsPage from './components/settings/SettingsPage';
import { useUiTokens } from './theme/useUiTokens';

const App: React.FC = () => {
  const [commandOpen, setCommandOpen] = React.useState(false);
  const { theme } = useThemeStore();
  const t = useUiTokens();
  const {
    currentView,
    isSidebarOpen,
    toasts,
    removeToast,
    globalError,
    setGlobalError,
    loadSystemInfo,
    checkOrangePi,
  } = useAppStore();
  const { addLog } = useLogStore();

  // Initialize app
  useEffect(() => {
    loadSystemInfo();
    checkOrangePi();
    addLog('info', 'App', 'OrangePi Debug Tool started');
    // 仅在挂载时初始化一次；store 动作引用稳定，故意不加入依赖数组
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Global error handler
  useEffect(() => {
    const handleError = (event: ErrorEvent) => {
      setGlobalError(event.message);
      addLog('error', 'Global', event.message, { stack: event.error?.stack });
    };

    const handleUnhandledRejection = (event: PromiseRejectionEvent) => {
      const message = String(event.reason);
      setGlobalError(message);
      addLog('error', 'Global', `Unhandled Promise Rejection: ${message}`);
    };

    window.addEventListener('error', handleError);
    window.addEventListener('unhandledrejection', handleUnhandledRejection);

    return () => {
      window.removeEventListener('error', handleError);
      window.removeEventListener('unhandledrejection', handleUnhandledRejection);
    };
    // 全局错误监听只需注册一次（handler 为一次性闭包），故意不加入依赖数组
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Command palette shortcut
  useEffect(() => {
    const handleKey = (event: KeyboardEvent) => {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'k') {
        event.preventDefault();
        setCommandOpen((v) => !v);
      }
    };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, []);

  // Render current page
  const renderPage = useCallback(() => {
    switch (currentView) {
      case 'overview':
        return <OverviewPage />;
      case 'serial':
        return <SerialPage />;
      case 'gpio':
        return <GpioPage />;
      case 'pwm':
        return <PwmPage />;
      case 'log':
        return <LogPage />;
      case 'settings':
        return <SettingsPage />;
      default:
        return <SerialPage />;
    }
  }, [currentView]);

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box sx={{ display: 'flex', flexDirection: 'column', height: '100vh', overflow: 'hidden', bgcolor: t.bg }}>
        {/* Main area */}
        <Box sx={{ display: 'flex', flexGrow: 1, overflow: 'hidden' }}>
          {/* Sidebar */}
          <Sidebar />

          {/* Main content */}
          <Box
            component="main"
            sx={{
              flexGrow: 1,
              display: 'flex',
              flexDirection: 'column',
              overflow: 'hidden',
              transition: (muiTheme) =>
                muiTheme.transitions.create('margin', {
                  easing: muiTheme.transitions.easing.sharp,
                  duration: muiTheme.transitions.duration.leavingScreen,
                }),
              marginLeft: isSidebarOpen ? 0 : `-${DRAWER_WIDTH}px`,
            }}
          >
            <Header onOpenCommand={() => setCommandOpen(true)} />
            <Box
              sx={{
                flexGrow: 1,
                overflow: 'auto',
                p: 3,
                backgroundColor: t.bg,
              }}
            >
              {renderPage()}
            </Box>
          </Box>
        </Box>

        <StatusBar />
      </Box>

      {/* Global error snackbar */}
      <Snackbar
        open={!!globalError}
        autoHideDuration={6000}
        onClose={() => setGlobalError(null)}
        anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
      >
        <Alert severity="error" onClose={() => setGlobalError(null)} sx={{ width: '100%' }}>
          {globalError}
        </Alert>
      </Snackbar>

      {/* Toast notifications */}
      {toasts.map((toast) => (
        <Snackbar
          key={toast.id}
          open
          autoHideDuration={toast.duration}
          onClose={() => removeToast(toast.id)}
          anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
        >
          <Alert
            severity={toast.type}
            onClose={() => removeToast(toast.id)}
            sx={{ width: '100%' }}
          >
            {toast.message}
          </Alert>
        </Snackbar>
      ))}
      {/* Command palette */}
      <CommandPalette open={commandOpen} onClose={() => setCommandOpen(false)} />
    </ThemeProvider>
  );
};

export default App;
