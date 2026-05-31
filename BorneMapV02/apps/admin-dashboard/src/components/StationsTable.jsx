import React from 'react';
import DataTable from './DataTable';
import { mockStations } from '../data/mockData';
import theme from '../styles/theme';

const COLUMNS = [
  { key: 'id', label: 'HUB ID', style: { padding: '12px', fontFamily: 'monospace', color: theme.colors.textSecondary } },
  { key: 'name', label: 'NAME DESIGNATION', style: { padding: '12px', fontWeight: theme.fontWeight.bold } },
  { key: 'location', label: 'ZONAL PLACEMENT', style: { padding: '12px' } },
  {
    key: 'status', label: 'STATUS',
    render: (s) => `● ${s.status}`,
    style: { padding: '12px', color: theme.colors.success, fontWeight: theme.fontWeight.bold },
  },
];

export default function StationsTable() {
  return (
    <DataTable
      title="Deployed System Hub Nodes"
      placeholder="Search stations..."
      data={mockStations}
      searchFields={['name', 'id', 'location', 'partner.name']}
      columns={COLUMNS}
    />
  );
}
