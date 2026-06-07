import { useState, type FormEvent } from 'react'
import type { Station, StationCreate, StationUpdate, StationStatus } from '@/lib/types'
import { Button } from '@/components/ui/button'

interface StationFormProps {
  station?: Station
  onSubmit: (data: StationCreate | StationUpdate) => Promise<void>
  onCancel: () => void
  loading?: boolean
}

const statusOptions: StationStatus[] = ['draft', 'active', 'inactive', 'maintenance']

export function StationForm({ station, onSubmit, onCancel, loading }: StationFormProps) {
  const [name, setName] = useState(station?.name ?? '')
  const [address, setAddress] = useState(station?.address ?? '')
  const [latitude, setLatitude] = useState(String(station?.latitude ?? ''))
  const [longitude, setLongitude] = useState(String(station?.longitude ?? ''))
  const [status, setStatus] = useState<StationStatus>(station?.status ?? 'draft')

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault()
    if (station) {
      await onSubmit({
        name: name !== station.name ? name : undefined,
        address: address !== (station.address ?? '') ? address || undefined : undefined,
        latitude: latitude ? Number(latitude) : undefined,
        longitude: longitude ? Number(longitude) : undefined,
        status: status !== station.status ? status : undefined,
      } as StationUpdate)
    } else {
      await onSubmit({
        name,
        address: address || undefined,
        latitude: Number(latitude),
        longitude: Number(longitude),
      } as StationCreate)
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">
          Name *
        </label>
        <input
          required
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] placeholder:text-[var(--color-text-muted)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
          placeholder="Station name"
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">
          Address
        </label>
        <input
          value={address}
          onChange={(e) => setAddress(e.target.value)}
          className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] placeholder:text-[var(--color-text-muted)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
          placeholder="Street, city"
        />
      </div>
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">
            Latitude *
          </label>
          <input
            required
            type="number"
            step="any"
            value={latitude}
            onChange={(e) => setLatitude(e.target.value)}
            className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
            placeholder="36.8065"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">
            Longitude *
          </label>
          <input
            required
            type="number"
            step="any"
            value={longitude}
            onChange={(e) => setLongitude(e.target.value)}
            className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
            placeholder="10.1815"
          />
        </div>
      </div>
      {station && (
        <div>
          <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">
            Status
          </label>
          <select
            value={status}
            onChange={(e) => setStatus(e.target.value as StationStatus)}
            className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
          >
            {statusOptions.map((s) => (
              <option key={s} value={s}>{s}</option>
            ))}
          </select>
        </div>
      )}
      <div className="flex justify-end gap-3 pt-2">
        <Button type="button" variant="outline" onClick={onCancel} disabled={loading}>
          Cancel
        </Button>
        <Button type="submit" disabled={loading}>
          {loading ? 'Saving...' : station ? 'Update Station' : 'Create Station'}
        </Button>
      </div>
    </form>
  )
}
