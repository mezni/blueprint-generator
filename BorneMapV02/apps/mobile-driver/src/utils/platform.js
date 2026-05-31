import { Platform, Dimensions } from 'react-native';

const { width } = Dimensions.get('window');

const isDesktop = Platform.OS === 'web' && width >= 768;
const isMobile = Platform.OS !== 'web' || width < 768;
const isIOS = Platform.OS === 'ios';
const isAndroid = Platform.OS === 'android';

export { isDesktop, isMobile, isIOS, isAndroid };
