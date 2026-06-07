import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './styles/index.css';

// Import fonts if needed
import { Inter_400Regular, Inter_600SemiBold, Inter_700Bold } from '@expo/vector-icons';

const root = document.getElementById('root');

if (root) {
  createRoot(root).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
}
