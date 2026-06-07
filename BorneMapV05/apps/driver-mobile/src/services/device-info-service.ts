import * as Device from 'expo-device';
import * as Application from 'expo-application';
import { Platform } from 'react-native';

export class DeviceInfoService {
  /**
   * Get device information
   */
  static getDeviceInfo() {
    return {
      deviceId: Device.deviceId || 'unknown',
      modelName: Device.modelName || 'unknown',
      brand: Device.brand || 'unknown',
      osVersion: Device.osVersion || 'unknown',
      platform: Platform.OS,
      isDevice: Device.isDevice,
      appVersion: Application.nativeApplicationVersion || '1.0.0',
      appBuildNumber: Application.nativeBuildVersion || '1',
    };
  }

  /**
   * Get unique device identifier
   */
  static getUniqueId() {
    return Device.deviceId || `device_${Date.now()}_${Math.random().toString(36).substring(7)}`;
  }

  /**
   * Check if device is Android
   */
  static isAndroid(): boolean {
    return Platform.OS === 'android';
  }

  /**
   * Check if device is iOS
   */
  static isIOS(): boolean {
    return Platform.OS === 'ios';
  }

  /**
   * Check if device is in RTL mode
   */
  static isRTL(): boolean {
    // This would be checked against the device locale
    return false; // TODO: Implement proper RTL detection
  }
}

export default DeviceInfoService;
