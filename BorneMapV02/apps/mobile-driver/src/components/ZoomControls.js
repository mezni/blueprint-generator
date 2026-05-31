import React from 'react';
import { View, TouchableOpacity, Text, StyleSheet, Platform } from 'react-native';
import { isDesktop } from '../utils/platform';

export default function ZoomControls({ onZoomIn, onZoomOut, onLocateMe, locationDisabled }) {
  if (isDesktop) {
    return (
      <View style={styles.desktopGroup} accessibilityRole="toolbar" aria-label="Map zoom controls">
        <button
          onClick={onZoomIn}
          style={styles.desktopBtn}
          aria-label="Zoom in"
          title="Zoom in"
        >
          +
        </button>
        <button
          onClick={onZoomOut}
          style={styles.desktopBtn}
          aria-label="Zoom out"
          title="Zoom out"
        >
          −
        </button>
        <button
          onClick={onLocateMe}
          style={styles.desktopBtn}
          disabled={locationDisabled}
          aria-label={locationDisabled ? 'Location unavailable' : 'Locate me'}
          title={locationDisabled ? 'Location permission denied' : 'Locate me'}
        >
          ◎
        </button>
      </View>
    );
  }

  return (
    <View style={styles.mobileContainer}>
      <TouchableOpacity
        style={styles.mobileBtn}
        onPress={onZoomIn}
        accessibilityLabel="Zoom in"
        accessibilityRole="button"
      >
        <Text style={styles.btnText}>+</Text>
      </TouchableOpacity>
      <TouchableOpacity
        style={styles.mobileBtn}
        onPress={onZoomOut}
        accessibilityLabel="Zoom out"
        accessibilityRole="button"
      >
        <Text style={styles.btnText}>−</Text>
      </TouchableOpacity>
      <TouchableOpacity
        style={[styles.mobileBtn, locationDisabled && styles.disabledBtn]}
        onPress={onLocateMe}
        disabled={locationDisabled}
        accessibilityLabel={locationDisabled ? 'Location unavailable — check permissions' : 'Locate me'}
        accessibilityRole="button"
      >
        <Text style={[styles.btnText, locationDisabled && styles.disabledText]}>◎</Text>
      </TouchableOpacity>
    </View>
  );
}

const styles = StyleSheet.create({
  desktopGroup: {
    position: 'absolute',
    bottom: 24,
    right: 16,
    display: 'flex',
    flexDirection: 'column',
    gap: 4,
    zIndex: 30,
  },
  desktopBtn: {
    width: 44,
    height: 44,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: '#FFFFFF',
    border: '1px solid #EEEEEE',
    borderRadius: 8,
    cursor: 'pointer',
    fontSize: 20,
    fontWeight: '600',
    color: '#333333',
    boxShadow: '0 2px 6px rgba(0,0,0,0.1)',
  },
  mobileContainer: {
    position: 'absolute',
    bottom: 100,
    right: 12,
    zIndex: 30,
    gap: 8,
  },
  mobileBtn: {
    width: 44,
    height: 44,
    borderRadius: 22,
    backgroundColor: '#FFFFFF',
    justifyContent: 'center',
    alignItems: 'center',
    elevation: 4,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.15,
    shadowRadius: 4,
    marginBottom: 8,
  },
  disabledBtn: {
    opacity: 0.4,
  },
  btnText: {
    fontSize: 22,
    fontWeight: '600',
    color: '#333333',
  },
  disabledText: {
    color: '#999999',
  },
});
