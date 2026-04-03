import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { LogEntry, LogFilter } from '../types';
import { v4 as uuidv4 } from 'uuid';

interface LogState {
  entries: LogEntry[];
  filter: LogFilter;
  isAutoScroll: boolean;
  maxEntries: number;
  
  addEntry: (entry: Omit<LogEntry, 'id' | 'timestamp'>) => void;
  addLog: (level: LogEntry['level'], source: string, message: string, data?: unknown) => void;
  clearLogs: () => void;
  setFilter: (filter: Partial<LogFilter>) => void;
  setAutoScroll: (enabled: boolean) => void;
  setMaxEntries: (max: number) => void;
  getFilteredEntries: () => LogEntry[];
  exportLogs: () => string;
}

const defaultFilter: LogFilter = {
  levels: ['debug', 'info', 'warn', 'error'],
  sources: [],
  search: '',
};

export const useLogStore = create<LogState>()(
  persist(
    (set, get) => ({
      entries: [],
      filter: defaultFilter,
      isAutoScroll: true,
      maxEntries: 10000,

      addEntry: (entry) => {
        const newEntry: LogEntry = {
          ...entry,
          id: uuidv4(),
          timestamp: Date.now(),
        };
        
        set((state) => ({
          entries: [...state.entries, newEntry].slice(-state.maxEntries),
        }));
      },

      addLog: (level, source, message, data) => {
        get().addEntry({ level, source, message, data });
      },

      clearLogs: () => set({ entries: [] }),

      setFilter: (filter) => set((state) => ({
        filter: { ...state.filter, ...filter },
      })),

      setAutoScroll: (isAutoScroll) => set({ isAutoScroll }),

      setMaxEntries: (maxEntries) => set({ maxEntries }),

      getFilteredEntries: () => {
        const { entries, filter } = get();
        
        return entries.filter((entry) => {
          // Level filter
          if (!filter.levels.includes(entry.level)) {
            return false;
          }
          
          // Source filter
          if (filter.sources.length > 0 && !filter.sources.includes(entry.source)) {
            return false;
          }
          
          // Time filter
          if (filter.startTime && entry.timestamp < filter.startTime) {
            return false;
          }
          if (filter.endTime && entry.timestamp > filter.endTime) {
            return false;
          }
          
          // Search filter
          if (filter.search) {
            const searchLower = filter.search.toLowerCase();
            const matches =
              entry.message.toLowerCase().includes(searchLower) ||
              entry.source.toLowerCase().includes(searchLower) ||
              (entry.data && JSON.stringify(entry.data).toLowerCase().includes(searchLower));
            if (!matches) return false;
          }
          
          return true;
        });
      },

      exportLogs: () => {
        const { entries } = get();
        return entries
          .map(
            (e) =>
              `[${new Date(e.timestamp).toISOString()}] [${e.level.toUpperCase()}] [${
                e.source
              }] ${e.message}`
          )
          .join('\n');
      },
    }),
    {
      name: 'log-storage',
      partialize: (state) => ({
        filter: state.filter,
        isAutoScroll: state.isAutoScroll,
        maxEntries: state.maxEntries,
      }),
    }
  )
);