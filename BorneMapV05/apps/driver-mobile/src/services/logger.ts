import { Platform } from 'react-native';
import DeviceInfoService from './device-info-service';

export class Logger {
  private static isEnabled = true;
  private static logLevel: 'debug' | 'info' | 'warn' | 'error' = 'info';

  static setEnabled(enabled: boolean): void {
    this.isEnabled = enabled;
  }

  static setLogLevel(level: 'debug' | 'info' | 'warn' | 'error'): void {
    this.logLevel = level;
  }

  static debug(message: string, ...args: any[]): void {
    if (this.isEnabled && this.logLevel === 'debug') {
      this.log('DEBUG', message, ...args);
    }
  }

  static info(message: string, ...args: any[]): void {
    if (this.isEnabled) {
      this.log('INFO', message, ...args);
    }
  }

  static warn(message: string, ...args: any[]): void {
    if (this.isEnabled && this.logLevel <= 'warn') {
      this.log('WARN', message, ...args);
    }
  }

  static error(message: string, ...args: any[]): void {
    if (this.isEnabled && this.logLevel <= 'error') {
      this.log('ERROR', message, ...args);
    }
  }

  private static log(level: string, message: string, ...args: any[]): void {
    const timestamp = new Date().toISOString();
    const deviceId = DeviceInfoService.getUniqueId();
    const platform = Platform.OS;
    
    const logEntry = {
      timestamp,
      level,
      message,
      args,
      deviceId,
      platform,
    };

    // In development, log to console
    if (__DEV__) {
      console.log(`[${level}]`, message, ...args);
    }

    // TODO: Send to remote logging service
    // await RemoteLoggingService.log(logEntry);
  }
}

export default Logger;
