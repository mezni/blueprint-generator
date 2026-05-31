import React, { useState } from 'react';
import theme from '../styles/theme';

export default function DataTable({ title, placeholder, data, searchFields, columns }) {
  const [query, setQuery] = useState('');

  const filtered = data.filter((row) => {
    if (!query) return true;
    const q = query.toLowerCase();
    return searchFields.some((field) => {
      const val = field.split('.').reduce((o, k) => o?.[k], row);
      return val != null && String(val).toLowerCase().includes(q);
    });
  });

  return (
    <div style={styles.tableSection}>
      <div style={styles.headerRow}>
        <h4 style={styles.sectionTitle}>{title}</h4>
        <input
          type="text"
          placeholder={placeholder}
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          style={styles.searchInput}
        />
      </div>
      <table style={styles.dataTable}>
        <thead>
          <tr style={styles.tableHeaderRow}>
            {columns.map((col) => (
              <th key={col.key}>{col.label}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {filtered.map((row, i) => (
            <tr key={row.id || i} style={styles.tableBodyRow}>
              {columns.map((col) => (
                <td key={col.key} style={col.style || styles.td}>
                  {col.render ? col.render(row) : String(row[col.key] ?? '')}
                </td>
              ))}
            </tr>
          ))}
          {filtered.length === 0 && (
            <tr>
              <td colSpan={columns.length} style={styles.emptyRow}>No results match your search.</td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
}

const styles = {
  tableSection: {
    backgroundColor: theme.colors.surface,
    border: `1px solid ${theme.colors.border}`,
    borderRadius: theme.borderRadius.md,
    padding: '24px',
  },
  headerRow: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '16px',
  },
  sectionTitle: {
    fontSize: theme.fontSize.lg,
    fontWeight: theme.fontWeight.extrabold,
    margin: 0,
  },
  searchInput: {
    padding: '8px 12px',
    border: `1px solid ${theme.colors.border}`,
    borderRadius: theme.borderRadius.md,
    fontSize: theme.fontSize.md,
    outline: 'none',
    width: '240px',
  },
  dataTable: {
    width: '100%',
    borderCollapse: 'collapse',
    textAlign: 'left',
  },
  tableHeaderRow: {
    borderBottom: '2px solid #EEE',
  },
  tableBodyRow: {
    borderBottom: '1px solid #F5F5F5',
  },
  td: {
    padding: '12px',
  },
  emptyRow: {
    padding: '24px',
    textAlign: 'center',
    color: theme.colors.textMuted,
    fontSize: theme.fontSize.md,
  },
};
