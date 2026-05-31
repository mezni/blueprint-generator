import { Platform } from 'react-native';

const SESSION_KEY = 'bornemap_session_id';

function generateId() {
  const hex = Math.random().toString(16).slice(2, 10);
  return `ses-${hex}${Math.random().toString(16).slice(2, 6)}`;
}

function getStorage() {
  if (Platform.OS === 'web') {
    try {
      return window.localStorage;
    } catch {
      return null;
    }
  }
  try {
    const AsyncStorage = require('@react-native-async-storage/async-storage').default;
    return AsyncStorage;
  } catch {
    return null;
  }
}

const storage = getStorage();
let memorySession = null;

export function getSessionId() {
  if (memorySession) return memorySession;

  if (storage) {
    if (Platform.OS === 'web') {
      const stored = storage.getItem(SESSION_KEY);
      if (stored) {
        memorySession = stored;
        return stored;
      }
    }
  }

  const newId = generateId();
  memorySession = newId;

  if (storage) {
    if (Platform.OS === 'web') {
      storage.setItem(SESSION_KEY, newId);
    } else {
      storage.setItem(SESSION_KEY, newId).catch(() => {});
    }
  }

  return newId;
}
