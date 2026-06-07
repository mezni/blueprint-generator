import { useState } from 'react'
import type { Station, StationUpdate, StationStatus } from '@/lib/types'
import { Button } from '@/components/ui/button'

interface StationFormProps {
  initial: Station
  onSave: (data: StationUpdate) => Promise<void>
  onCancel: () => void
}

export function StationForm({ initial, onSave, onCancel }: StationFormProps) {
  const [name, setName] = useState(initial.name)
  const [description, setDescription] = useState(initial.description ?? '')
  const [latitude, setLatitude] = useState(String(initial.latitude))
  const [longitude, setLongitude] = useState(String(initial.longitude))
  const [status, setStatus] = useState<StationStatus>(initial.status)
  const [isLive, setIsLive] = useState(initial.is_live)
  const [isPublic, setIsPublic] = useState(initial.is_public)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [confirmCoord, setConfirmCoord] = useState(false)

  const coordsChanged =
    String(initial.latitude) !== latitude || String(initial.longitude) !== longitude

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (coordsChanged && !confirmCoord) {
      setConfirmCoord(true)
      return
    }
    setSaving(true)
    setError(null)
    try {
      await onSave({
        name,
        description: description || undefined,
        latitude: coordsChanged ? Number(latitude) : undefined,
        longitude: coordsChanged ? Number(longitude) : undefined,
        status,
        is_live: isLive,
        is_public: isPublic,
      })
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save station')
    } finally {
      setSaving(false)
    }
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-4">
      {confirmCoord && (
        <div className="rounded-md border border-[var(--color-warning-muted)] bg-[var(--color-warning-muted)]/10 p-3 text-sm text-[var(--color-warning-base)]">
          Changing coordinates will trigger a GIS resync. Are you sure you want to update the location?
        </div>
      )}
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
        <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">Description</label>
        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          rows={2}
          className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
        />
      </div>
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">Latitude</label>
          <input
            value={latitude}
            onChange={(e) => setLatitude(e.target.value)}
            type="number"
            step="any"
            className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">Longitude</label>
          <input
            value={longitude}
            onChange={(e) => setLongitude(e.target.value)}
            type="number"
            step="any"
            className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
          />
        </div>
      </div>
      <div>
        <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">Status</label>
        <select
          value={status}
          onChange={(e) => setStatus(e.target.value as StationStatus)}
          className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
        >
          <option value="active">Active</option>
          <option value="inactive">Inactive</option>
          <option value="maintenance">Maintenance</option>
          <option value="draft">Draft</option>
        </select>
      </div>
      <div className="flex gap-4">
        <label className="flex items-center gap-2 text-sm text-[var(--color-text-base)]">
          <input type="checkbox" checked={isLive} onChange={(e) => setIsLive(e.target.checked)} />
          Is Live
        </label>
        <label className="flex items-center gap-2 text-sm text-[var(--color-text-base)]">
          <input type="checkbox" checked={isPublic} onChange={(e) => setIsPublic(e.target.checked)} />
          Is Public
        </label>
      </div>
      {error && <p className="text-sm text-[var(--color-error-base)]">{error}</p>}
      <div className="flex justify-end gap-2">
        <Button type="button" variant="outline" onClick={onCancel}>Cancel</Button>
        <Button type="submit" disabled={saving}>
          {saving ? 'Saving...' : confirmCoord ? 'Confirm Changes' : 'Save'}
        </Button>
      </div>
    </form>
  )
}
