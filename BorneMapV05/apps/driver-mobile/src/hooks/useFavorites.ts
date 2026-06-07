import { useState, useCallback } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import { AsyncStorage } from '@react-native-async-storage/async-storage';
import { StationService } from '../services/station-service';

const FAVORITES_KEY = 'user_favorites';

export function useFavorites() {
  const queryClient = useQueryClient();
  const [favorites, setFavorites] = useState<Set<string>>(new Set());

  // Load favorites from storage on mount
  useEffect(() => {
    loadFavorites();
  }, []);

  const loadFavorites = useCallback(async () => {
    try {
      const favoritesData = await AsyncStorage.getItem(FAVORITES_KEY);
      if (favoritesData) {
        const favoritesArray = JSON.parse(favoritesData);
        setFavorites(new Set(favoritesArray));
      }
    } catch (error) {
      console.error('Failed to load favorites:', error);
    }
  }, []);

  const addFavorite = useCallback(async (stationId: string) => {
    try {
      const newFavorites = new Set(favorites);
      newFavorites.add(stationId);
      setFavorites(newFavorites);
      
      await AsyncStorage.setItem(FAVORITES_KEY, JSON.stringify(Array.from(newFavorites)));
      
      // Invalidate stations query to update UI
      queryClient.invalidateQueries({ queryKey: ['stations'] });
      
      return true;
    } catch (error) {
      console.error('Failed to add favorite:', error);
      return false;
    }
  }, [favorites, queryClient]);

  const removeFavorite = useCallback(async (stationId: string) => {
    try {
      const newFavorites = new Set(favorites);
      newFavorites.delete(stationId);
      setFavorites(newFavorites);
      
      await AsyncStorage.setItem(FAVORITES_KEY, JSON.stringify(Array.from(newFavorites)));
      
      // Invalidate stations query to update UI
      queryClient.invalidateQueries({ queryKey: ['stations'] });
      
      return true;
    } catch (error) {
      console.error('Failed to remove favorite:', error);
      return false;
    }
  }, [favorites, queryClient]);

  const toggleFavorite = useCallback(async (stationId: string) => {
    if (favorites.has(stationId)) {
      return await removeFavorite(stationId);
    } else {
      return await addFavorite(stationId);
    }
  }, [favorites, addFavorite, removeFavorite]);

  const isFavorite = useCallback((stationId: string) => {
    return favorites.has(stationId);
  }, [favorites]);

  const getFavoritesList = useCallback(() => {
    return Array.from(favorites);
  }, [favorites]);

  return {
    favorites,
    addFavorite,
    removeFavorite,
    toggleFavorite,
    isFavorite,
    getFavoritesList,
    loadFavorites,
  };
}

export default useFavorites;
