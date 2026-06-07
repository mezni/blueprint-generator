import { useNavigate } from 'react-router'
import { useAdminOverview } from '@/hooks/useAdminOverview'
import { DataCard } from '@/components/DataCard'

const cardIcons = {
  partners: <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><path d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" /></svg>,
  stations: <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><path d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>,
  active: <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>,
  reviews: <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"><path d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" /></svg>,
}

export default function DashboardPage() {
  const navigate = useNavigate()
  const { data, isLoading, isError, refetch } = useAdminOverview()

  if (isLoading) {
    return (
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {[...Array(4)].map((_, i) => (
          <div
            key={i}
            className="h-24 animate-pulse rounded-lg bg-[var(--color-surface-base)] border border-[var(--color-border-muted)]"
          />
        ))}
      </div>
    )
  }

  if (isError || !data?.data) {
    return (
      <div className="flex flex-col items-center gap-3 py-20">
        <p className="text-[var(--color-text-muted)]">Failed to load dashboard data</p>
        <button
          onClick={() => refetch()}
          className="rounded-md bg-[var(--color-primary-base)] px-4 py-2 text-sm text-white hover:bg-[var(--color-primary-hover)]"
        >
          Retry
        </button>
      </div>
    )
  }

  const { total_partners, total_stations, active_stations, pending_reviews } = data.data

  return (
    <div>
      <h1 className="text-xl font-bold text-[var(--color-text-base)] mb-6">Dashboard Overview</h1>
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <DataCard
          label="Total Partners"
          value={total_partners}
          icon={cardIcons.partners}
          onClick={() => navigate('/partners')}
        />
        <DataCard
          label="Total Stations"
          value={total_stations}
          icon={cardIcons.stations}
          onClick={() => navigate('/stations')}
        />
        <DataCard
          label="Active Stations"
          value={active_stations}
          icon={cardIcons.active}
        />
        <DataCard
          label="Pending Reviews"
          value={pending_reviews}
          icon={cardIcons.reviews}
          onClick={() => navigate('/reviews')}
        />
      </div>
    </div>
  )
}
