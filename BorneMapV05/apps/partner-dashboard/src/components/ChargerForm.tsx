import { useState, type FormEvent } from 'react'
import type { Charger, ChargerCreate, ChargerUpdate, ChargerType } from '@/lib/types'
import { Button } from '@/components/ui/button'

interface ChargerFormProps {
  stationId?: string
  charger?: Charger
  onSubmit: (data: ChargerCreate | ChargerUpdate) => Promise<void>
  onCancel: () => void
  loading?: boolean
}

const chargerTypes: ChargerType[] = ['CCS', 'Type2', 'CHAdeMO']

export function ChargerForm({ stationId, charger, onSubmit, onCancel, loading }: ChargerFormProps) {
  const [chargerType, setChargerType] = useState<ChargerType>(charger?.charger_type ?? 'CCS')
  const [powerKw, setPowerKw] = useState(String(charger?.power_kw ?? ''))

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault()
    if (charger) {
      await onSubmit({
        charger_type: chargerType !== charger.charger_type ? chargerType : undefined,
        power_kw: powerKw ? Number(powerKw) : undefined,
      } as ChargerUpdate)
    } else {
      await onSubmit({
        station_id: stationId!,
        charger_type: chargerType,
        power_kw: Number(powerKw),
        status: 'available',
      } as ChargerCreate)
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">
          Charger Type *
        </label>
        <select
          value={chargerType}
          onChange={(e) => setChargerType(e.target.value as ChargerType)}
          className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
        >
          {chargerTypes.map((t) => (
            <option key={t} value={t}>{t}</option>
          ))}
        </select>
      </div>
      <div>
        <label className="block text-sm font-medium text-[var(--color-text-base)] mb-1">
          Power (kW) *
        </label>
        <input
          required
          type="number"
          step="0.1"
          value={powerKw}
          onChange={(e) => setPowerKw(e.target.value)}
          className="w-full rounded-md border border-[var(--color-border-base)] bg-[var(--color-surface-base)] px-3 py-2 text-sm text-[var(--color-text-base)] focus:outline-none focus:ring-2 focus:ring-[var(--color-primary-base)]"
          placeholder="50"
        />
      </div>
      <div className="flex justify-end gap-3 pt-2">
        <Button type="button" variant="outline" onClick={onCancel} disabled={loading}>
          Cancel
        </Button>
        <Button type="submit" disabled={loading}>
          {loading ? 'Saving...' : charger ? 'Update Charger' : 'Add Charger'}
        </Button>
      </div>
    </form>
  )
}
