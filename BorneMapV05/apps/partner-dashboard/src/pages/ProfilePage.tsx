import { usePartnerProfile } from '@/hooks/usePartnerProfile'

export function ProfilePage() {
  const { data: profile, isLoading } = usePartnerProfile()

  if (isLoading) {
    return (
      <div className="p-6">
        <div className="text-[var(--color-text-muted)] py-8 text-center">Loading profile...</div>
      </div>
    )
  }

  if (!profile) {
    return (
      <div className="p-6">
        <div className="text-[var(--color-text-muted)] py-8 text-center">Could not load profile.</div>
      </div>
    )
  }

  return (
    <div className="p-6 max-w-lg">
      <h1 className="text-2xl font-bold text-[var(--color-text-base)] mb-6">Profile</h1>
      <div className="rounded-lg border border-[var(--color-border-muted)] bg-[var(--color-surface-base)] shadow-card p-6 space-y-4">
        <div>
          <label className="text-xs font-medium text-[var(--color-text-muted)] uppercase tracking-wide">
            Partner Name
          </label>
          <p className="text-[var(--color-text-base)] font-medium">
            {profile.partner_name ?? '—'}
          </p>
        </div>
        <div>
          <label className="text-xs font-medium text-[var(--color-text-muted)] uppercase tracking-wide">
            Email
          </label>
          <p className="text-[var(--color-text-base)]">{profile.email ?? '—'}</p>
        </div>
        <div>
          <label className="text-xs font-medium text-[var(--color-text-muted)] uppercase tracking-wide">
            Role
          </label>
          <p className="text-[var(--color-text-base)]">
            {profile.membership_role ?? '—'}
          </p>
        </div>
        <div>
          <label className="text-xs font-medium text-[var(--color-text-muted)] uppercase tracking-wide">
            User ID
          </label>
          <p className="text-[var(--color-text-muted)] font-mono text-xs break-all">
            {profile.user_id}
          </p>
        </div>
        {profile.partner_id && (
          <div>
            <label className="text-xs font-medium text-[var(--color-text-muted)] uppercase tracking-wide">
              Partner ID
            </label>
            <p className="text-[var(--color-text-muted)] font-mono text-xs break-all">
              {profile.partner_id}
            </p>
          </div>
        )}
      </div>
    </div>
  )
}
