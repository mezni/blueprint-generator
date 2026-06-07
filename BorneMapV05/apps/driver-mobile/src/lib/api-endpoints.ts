export const ApiEndpoints = {
  STATIONS: '/stations',
  STATION_DETAIL: '/stations/:id',
  STATION_NEARBY: '/stations/nearby',
  STATION_CHARGERS: '/stations/:id/chargers',
  FAVORITES: '/favorites',
  FAVORITE_ADD: '/favorites',
  FAVORITE_REMOVE: '/favorites/:id',
  REVIEWS: '/stations/:id/reviews',
  REVIEW_SUBMIT: '/stations/:id/reviews',
  LOGIN: '/auth/login',
  LOGOUT: '/auth/logout',
  PROFILE: '/profile',
  FCM_TOKEN: '/push/tokens',
  ANALYTICS_EVENT: '/analytics/events',
} as const;

export const AnalyticsEvents = {
  MAP_VIEW_INITIATED: 'map_view_initiated',
  STATION_CLICKED: 'station_clicked',
  CHARGER_SELECTED: 'charger_selected',
  STATION_FAVORITED: 'station_favorited',
  STATION_UNFAVORITED: 'station_unfavorited',
  REVIEW_SUBMITTED: 'review_submitted',
  MAP_FILTER_APPLIED: 'map_filter_applied',
  SEARCH_QUERY: 'search_query',
  OFFLINE_MODE_ENABLED: 'offline_mode_enabled',
  OFFLINE_DATA_SYNCED: 'offline_data_synced',
} as const;
