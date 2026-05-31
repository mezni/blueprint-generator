import React from 'react';
import DataTable from './DataTable';
import { mockPartners } from '../data/mockData';
import theme from '../styles/theme';

const COLUMNS = [
  { key: 'id', label: 'ID', style: { padding: '12px', fontFamily: 'monospace', color: theme.colors.textSecondary } },
  { key: 'name', label: 'BRAND ENTITY NAME', style: { padding: '12px', fontWeight: theme.fontWeight.bold } },
  {
    key: 'hubs', label: 'HUBS',
    render: (p) => `${p.hubs} Nodes`,
    style: { padding: '12px' },
  },
  {
    key: 'status', label: 'STATUS',
    render: (p) => `● ${p.status}`,
    style: { padding: '12px', color: theme.colors.success, fontWeight: theme.fontWeight.bold },
  },
];

export default function PartnersTable() {
  return (
    <DataTable
      title="Strategic Partners Registry"
      placeholder="Search partners..."
      data={mockPartners}
      searchFields={['name', 'id']}
      columns={COLUMNS}
    />
  );
}
