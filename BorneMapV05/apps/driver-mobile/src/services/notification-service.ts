import { Platform } from 'react-native';
import * as Device from 'expo-device';
import * as Notifications from 'expo-notifications';

export class NotificationService {
  /**
   * Request notification permissions
   */
  static async requestPermissions(): Promise<boolean> {
    try {
      if (Platform.OS === 'android') {
        await Notifications.setNotificationChannelAsync('bornemap-notifications', {
          name: 'Bornemap Notifications',
          importance: Notifications.AndroidImportance.MAX,
          vibrationPattern: [0, 250, 250, 250],
          lightColor: '#FF231F7C',
        });
      }

      const { status: existingStatus } = await Notifications.getPermissionsAsync();
      let finalStatus = existingStatus;

      if (existingStatus !== 'granted') {
        const { status } = await Notifications.requestPermissionsAsync();
        finalStatus = status;
      }

      return finalStatus === 'granted';
    } catch (error) {
      console.error('Failed to request notification permissions:', error);
      return false;
    }
  }

  /**
   * Register for push notifications
   */
  static async registerForPushNotifications(): Promise<string | null> {
    try {
      let token = null;

      if (Device.isDevice) {
        const { status: existingStatus } = await Notifications.getPermissionsAsync();
        let finalStatus = existingStatus;

        if (existingStatus !== 'granted') {
          const { status } = await Notifications.requestPermissionsAsync();
          finalStatus = status;
        }

        if (finalStatus !== 'granted') {
          console.log('Failed to get push token for push notification!');
          return null;
        }

        token = (await Notifications.getExpoPushTokenAsync()).data;
      } else {
        console.log('Must use physical device for Push Notifications');
      }

      if (Platform.OS === 'android') {
        token = token?.replace('ExponentPushToken[', '').replace(']', '');
      }

      return token;
    } catch (error) {
      console.error('Failed to register for push notifications:', error);
      return null;
    }
  }

  /**
   * Send notification
   */
  static async sendNotification(
    title: string,
    body: string,
    data?: any
  ): Promise<void> {
    try {
      await Notifications.scheduleNotificationAsync({
        content: {
          title,
          body,
          data,
          sound: true,
          priority: Notifications.AndroidNotificationPriority.HIGH,
        },
        trigger: null, // Send immediately
      });
    } catch (error) {
      console.error('Failed to send notification:', error);
    }
  }

  /**
   * Listen for notification receipts
   */
  static addNotificationReceivedListener(
    callback: (notification: any) => void
  ) {
    return Notifications.addNotificationReceivedListener(callback);
  }

  /**
   * Listen for notification responses
   */
  static addNotificationResponseReceivedListener(
    callback: (response: any) => void
  ) {
    return Notifications.addNotificationResponseReceivedListener(callback);
  }
}

export default NotificationService;
