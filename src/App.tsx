import React, { useEffect, useCallback } from 'react';
import { Box, ThemeProvider, CssBaseline, Snackbar, Alert } from '@mui/material';
import { useThemeStore, useAppStore, useLogStore } from './stores';
import Sidebar from './components/layout/Sidebar';
import Header from './components/layout/Header';
import SerialPage from './components/serial/SerialPage';
import GpioPage from './components/gpio/GpioPage';
import PwmPage from './components/pwm/PwmPage';
import LogPage from './components/log/LogPage';
import { useSerialStore } from './stores/serialStore';

const App: React.FC = () => {
  const { theme } = useThemeStore();
  const {
    currentView,
    isSidebarOpen,
    toasts,
    removeToast,
    globalError,
    setGlobalError,
    loadSystemInfo,
    checkOrangePi,
    systemInfo,
  } = useAppStore();
  const { addLog } = useLogStore();
  const { status, config } = useSerialStore();

  // Initialize app
  useEffect(() => {
    loadSystemInfo();
    checkOrangePi();
    addLog('info', 'App', 'OrangePi Debug Tool started');
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
  }, []);

  // Render current page
  const renderPage = useCallback(() => {
    switch (currentView) {
      case 'serial':
        return <SerialPage />;
      case 'gpio':
        return <GpioPage />;
      case 'pwm':
        return <PwmPage />;
      case 'log':
        return <LogPage />;
      default:
        return <SerialPage />;
    }
  }, [currentView]);

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box sx={{ display: 'flex', flexDirection: 'column', height: '100vh', overflow: 'hidden', bgcolor: '#0a0a0a' }}>
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
              transition: (theme) =>
                theme.transitions.create('margin', {
                  easing: theme.transitions.easing.sharp,
                  duration: theme.transitions.duration.leavingScreen,
                }),
              marginLeft: isSidebarOpen ? 0 : '-240px',
            }}
          >
            <Header />
            <Box
              sx={{
                flexGrow: 1,
                overflow: 'auto',
                p: 3,
                backgroundColor: '#0a0a0a',
              }}
            >
              {renderPage()}
            </Box>
          </Box>
        </Box>

        {/* Status Bar - Cursor-style bottom bar */}
        <Box
          sx={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            px: 2,
            py: 0.5,
            minHeight: 24,
            backgroundColor: '#141414',
            borderTop: '1px solid #2a2a2a',
            fontSize: '0.7rem',
            fontFamily: '"JetBrains Mono", "SF Mono", monospace',
            color: '#71717a',
            flexShrink: 0,
          }}
        >
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
            {/* Connection Status */}
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
              <Box
                component="span"
                sx={{
                  width: 6,
                  height: 6,
                  borderRadius: '50%',
                  backgroundColor: status.connected ? '#4ade80' : '#71717a',
                  boxShadow: status.connected ? '0 0 6px rgba(74, 222, 128, 0.5)' : 'none',
                }}
              />
              <span>{status.connected ? 'Connected' : 'Disconnected'}</span>
            </Box>
            {/* Port Info */}
            {status.connected && config.port_name && (
              <span>
                {config.port_name} @ {config.baud_rate} bps
              </span>
            )}
            {/* Data Stats */}
            {status.connected && (
              <span>
                RX: {status.rx_bytes} | TX: {status.tx_bytes}
              </span>
            )}
          </Box>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
            {/* Device Info */}
            {systemInfo?.hostname && (
              <span>{systemInfo.hostname}</span>
            )}
            <span>OrangePi Debug Tool v{systemInfo?.version || '2.0.0'}</span>
          </Box>
        </Box>
      </Box>

      {/* Global error snackbar */}
      <Snackbar
        open={!!globalError}
        autoHideDuration={6000}
        onClose={() => setGlobalError(null)}
        anchorOrigin={{ vertical: 'top', horizontal: 'center' }}
      >
        <Alert
          severity="error"
          onClose={() => setGlobalError(null)}
          sx={{ width: '100%', backgroundColor: '#1a1a1a', color: '#f87171', border: '1px solid rgba(239, 68, 68, 0.3)' }}
        >
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
    </ThemeProvider>
  );
};

export default App;
