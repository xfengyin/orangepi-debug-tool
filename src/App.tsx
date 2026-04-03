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
  } = useAppStore();
  const { addLog } = useLogStore();
  const { status } = useSerialStore();

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
      <Box sx={{ display: 'flex', height: '100vh', overflow: 'hidden' }}>
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
              backgroundColor: (theme) => theme.palette.background.default,
            }}
          >
            {renderPage()}
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
          sx={{ width: '100%' }}
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