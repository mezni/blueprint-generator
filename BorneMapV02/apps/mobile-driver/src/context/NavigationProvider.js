import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { Platform } from 'react-native';
import MapScreen from '../screens/MapScreen';
import ExploreScreen from '../screens/ExploreScreen';
import SavedScreen from '../screens/SavedScreen';
import ProfileScreen from '../screens/ProfileScreen';
import { useAppContext } from '../context/AppContext';

const Tab = createBottomTabNavigator();

function MapTab() {
  return <MapScreen />;
}

export default function NavigationProvider({ children }) {
  const { activeTab, setActiveTab } = useAppContext();

  if (Platform.OS === 'web') {
    return <>{children}</>;
  }

  return (
    <NavigationContainer>
      <Tab.Navigator
        screenOptions={{
          headerShown: false,
          tabBarActiveTintColor: '#007AFF',
          tabBarInactiveTintColor: '#999999',
          tabBarStyle: {
            backgroundColor: '#FFFFFF',
            borderTopWidth: 1,
            borderTopColor: '#EEEEEE',
            paddingBottom: 8,
            paddingTop: 8,
            height: 60,
          },
          tabBarLabelStyle: {
            fontSize: 11,
            fontWeight: '600',
          },
        }}
      >
        <Tab.Screen
          name="Map"
          component={MapTab}
          options={{
            tabBarLabel: 'Map',
            tabBarIcon: ({ color }) => <TabIcon label="Map" color={color} />,
          }}
          listeners={{
            tabPress: () => setActiveTab('map'),
          }}
        />
        <Tab.Screen
          name="Explore"
          component={ExploreScreen}
          options={{
            tabBarLabel: 'Explore',
            tabBarIcon: ({ color }) => <TabIcon label="Explore" color={color} />,
          }}
          listeners={{
            tabPress: () => setActiveTab('explore'),
          }}
        />
        <Tab.Screen
          name="Saved"
          component={SavedScreen}
          options={{
            tabBarLabel: 'Saved',
            tabBarIcon: ({ color }) => <TabIcon label="Saved" color={color} />,
          }}
          listeners={{
            tabPress: () => setActiveTab('saved'),
          }}
        />
        <Tab.Screen
          name="Profile"
          component={ProfileScreen}
          options={{
            tabBarLabel: 'Profile',
            tabBarIcon: ({ color }) => <TabIcon label="Profile" color={color} />,
          }}
          listeners={{
            tabPress: () => setActiveTab('profile'),
          }}
        />
      </Tab.Navigator>
    </NavigationContainer>
  );
}

function TabIcon({ label, color }) {
  const icons = {
    Map: '🗺️',
    Explore: '🔍',
    Saved: '⭐',
    Profile: '👤',
  };
  return (
    <React.Fragment>
      <span style={{ fontSize: 20, color }}>{icons[label] || '•'}</span>
    </React.Fragment>
  );
}
