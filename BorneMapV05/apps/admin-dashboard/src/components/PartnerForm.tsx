import { useState } from 'react'
import type { Partner, PartnerCreate, PartnerUpdate, PartnerStatus } from '@/lib/types'
import { Button } from '@/components/ui/button'

interface PartnerFormProps {
  initial?: Partner
  onSave: (data: PartnerCreate | PartnerUpdate) => Promise<void>
  onCancel: () => void
}

export function PartnerForm({ initial, onSave, onCancel }: PartnerFormProps) {
  const [name, setName] = useState(initial?.name || '')
  const [email, setEmail] = useState(initial?.email ?? '')
  const [phone, setPhone] = useState(initial?.phone ?? '')
  const [status, setStatus] = useState(initial?.status || 'active')
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setSaving(true)
    setError(null)
    try {
      await onSave({ name, email: email || undefined, phone: phone || undefined, status })
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save partner')
    } finally {
      setSaving(false)
    }
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-4">
      <div>
        <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">Name</label>
        <input
          value={name}
          onChange={(e) => setName(e.target.value)}
          required
          className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">Email</label>
        <input
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          type="email"
          className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">Phone</label>
        <input
          value={phone}
          onChange={(e) => setPhone(e.target.value)}
          type="tel"
          className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">Status</label>
        <select
          value={status}
          onChange={(e) => setStatus(e.target.value as PartnerStatus)}
          className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
        >
          <option value="active">Active</option>
          <option value="suspended">Suspended</option>
        </select>
      </div>
      {error && <p className="text-sm text-[var(--color-error-base)]">{error}</p>}
      <div className="flex justify-end gap-2">
        <Button type="button" variant="outline" onClick={onCancel}>Cancel</Button>
        <Button type="submit" disabled={saving}>
          {saving ? 'Saving...' : initial ? 'Update' : 'Create'}
        </Button>
      </div>
    </form>
  )
}
