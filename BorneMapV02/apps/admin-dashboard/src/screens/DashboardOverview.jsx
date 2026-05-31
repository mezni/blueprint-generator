import React, { useState } from 'react';
import TopBar from '../components/TopBar';
import Sidebar from '../components/Sidebar';
import OverviewMetrics from '../components/OverviewMetrics';
import PartnersTable from '../components/PartnersTable';
import StationsTable from '../components/StationsTable';
import FallbackView from '../components/FallbackView';
import theme from '../styles/theme';

const FALLBACK_TABS = ['users', 'analytics', 'settings', 'logs'];

export default function DashboardOverview() {
  const [activeTab, setActiveTab] = useState('overview');
  const [entitiesOpen, setEntitiesOpen] = useState(true);

  const renderContent = () => {
    if (activeTab === 'overview') {
      return (
        <div>
          <h2 style={styles.viewHeading}>Vitals Summary Matrix</h2>
          <OverviewMetrics />
        </div>
      );
    }
    if (activeTab === 'partners') return <PartnersTable />;
    if (activeTab === 'stations') return <StationsTable />;
    if (FALLBACK_TABS.includes(activeTab)) return <FallbackView tabName={activeTab} />;
    return null;
  };

  return (
    <div style={styles.adminFrame}>
      <TopBar />
      <div style={styles.workspaceBody}>
        <Sidebar
          activeTab={activeTab}
          onTabChange={setActiveTab}
          entitiesOpen={entitiesOpen}
          onEntitiesToggle={() => setEntitiesOpen(!entitiesOpen)}
        />
        <main style={styles.contentCanvas}>{renderContent()}</main>
      </div>
    </div>
  );
}

const styles = {
  adminFrame: {
    display: 'flex',
    flexDirection: 'column',
    width: '100vw',
    height: '100vh',
    fontFamily: 'system-ui, sans-serif',
    backgroundColor: theme.colors.background,
    overflow: 'hidden',
  },
  workspaceBody: {
    flex: 1,
    display: 'flex',
    width: '100%',
  },
  contentCanvas: {
    flex: 1,
    padding: '32px',
    overflowY: 'auto',
  },
  viewHeading: {
    fontSize: theme.fontSize.xxl,
    fontWeight: theme.fontWeight.extrabold,
    marginBottom: '24px',
  },
};
