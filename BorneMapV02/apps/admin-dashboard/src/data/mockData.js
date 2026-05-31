export const mockPartners = [
  { id: 'prt-0000000a', name: 'TotalEnergies Tunisia', hubs: 42, status: 'Active' },
  { id: 'prt-0000000b', name: 'Shell Tunisia', hubs: 18, status: 'Active' },
];

export const mockStations = [
  {
    id: 'stn-00000001',
    name: 'Les Berges du Lac 2 Hub',
    latitude: 36.8325,
    longitude: 10.2415,
    partner: { id: 'prt-total', name: 'TotalEnergies Tunisia' },
    location: 'Tunis',
    status: 'Online',
    chargers: [
      { id: 'chg-1', plug_type: 'CCS2', power_output: 120, status: 'Available' },
      { id: 'chg-2', plug_type: 'CCS2', power_output: 50, status: 'Available' },
      { id: 'chg-3', plug_type: 'Type2', power_output: 22, status: 'Occupied' },
    ],
  },
  {
    id: 'stn-00000002',
    name: 'Sidi Bou Said Marina Station',
    latitude: 36.8704,
    longitude: 10.3475,
    partner: { id: 'prt-shell', name: 'Shell Tunisia' },
    location: 'Carthage',
    status: 'Online',
    chargers: [
      { id: 'chg-4', plug_type: 'CCS2', power_output: 50, status: 'Available' },
    ],
  },
];
