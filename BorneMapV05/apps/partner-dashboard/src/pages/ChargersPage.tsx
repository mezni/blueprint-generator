import { useState } from 'react'
import { usePartnerChargers, useUpdateCharger } from '@/hooks/usePartnerChargers'
import type { Charger, ChargerUpdate } from '@/lib/types'
import { Button } from '@/components/ui/button'
import { Modal } from '@/components/Modal'
import { ChargerForm } from '@/components/ChargerForm'

const statusColors: Record<string, string> = {
  available: 'text-[var(--color-success-base)]',
  offline: 'text-[var(--color-text-muted)]',
  fault: 'text-[var(--color-error-base)]',
}

export function ChargersPage() {
  const [page, setPage] = useState(1)
  const { data, isLoading } = usePartnerChargers(undefined, page)
  const updateCharger = useUpdateCharger()

  const [editingCharger, setEditingCharger] = useState<Charger | null>(null)

  const chargers = data?.data ?? []
  const meta = data?.meta

  const handleUpdate = async (data: ChargerUpdate) => {
    if (!editingCharger) return
    await updateCharger.mutateAsync({
      id: editingCharger.charger_id,
      data,
      etag: editingCharger.updated_at,
    })
    setEditingCharger(null)
  }

  return (
    <div className="p-6">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-[var(--color-text-base)]">Chargers</h1>
        <p className="text-sm text-[var(--color-text-muted)] mt-1">
          All chargers across your stations
        </p>
      </div>

      {isLoading ? (
        <div className="text-[var(--color-text-muted)] py-8 text-center">Loading chargers...</div>
      ) : chargers.length === 0 ? (
        <div className="text-[var(--color-text-muted)] py-8 text-center">
          No chargers found. Add chargers to your stations.
        </div>
      ) : (
        <div className="overflow-hidden rounded-lg border border-[var(--color-border-muted)]">
          <table className="w-full">
            <thead>
              <tr className="bg-[var(--color-surface-hover)] text-left text-sm font-medium text-[var(--color-text-muted)]">
                <th className="px-4 py-3">Type</th>
                <th className="px-4 py-3">Station ID</th>
                <th className="px-4 py-3">Power</th>
                <th className="px-4 py-3">Status</th>
                <th className="px-4 py-3">Updated</th>
                <th className="px-4 py-3" />
              </tr>
            </thead>
            <tbody className="divide-y divide-[var(--color-border-muted)]">
              {chargers.map((charger) => (
                <tr
                  key={charger.charger_id}
                  className="text-sm text-[var(--color-text-base)] hover:bg-[var(--color-surface-hover)] transition-colors"
                >
                  <td className="px-4 py-3 font-medium">{charger.charger_type}</td>
                  <td className="px-4 py-3 text-[var(--color-text-muted)] font-mono text-xs">
                    {charger.station_id}
                  </td>
                  <td className="px-4 py-3">{charger.power_kw} kW</td>
                  <td className={`px-4 py-3 font-medium ${statusColors[charger.status]}`}>
                    {charger.status}
                  </td>
                  <td className="px-4 py-3 text-[var(--color-text-muted)] text-xs">
                    {new Date(charger.updated_at).toLocaleDateString()}
                  </td>
                  <td className="px-4 py-3">
                    <button
                      onClick={() => setEditingCharger(charger)}
                      className="rounded-md p-1.5 text-[var(--color-text-muted)] hover:bg-[var(--color-surface-active)] hover:text-[var(--color-text-base)] transition-colors"
                    >
                      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <path
                          d="M11 1.5l3.5 3.5L5.5 14H2v-3.5L11 1.5z"
                          stroke="currentColor"
                          strokeWidth="1.5"
                          strokeLinecap="round"
                          strokeLinejoin="round"
                        />
                      </svg>
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {meta && meta.total_pages > 1 && (
        <div className="mt-6 flex items-center justify-center gap-2">
          <Button
            variant="outline"
            size="sm"
            disabled={!meta.has_prev}
            onClick={() => setPage((p) => p - 1)}
          >
            Previous
          </Button>
          <span className="text-sm text-[var(--color-text-muted)]">
            Page {meta.page} of {meta.total_pages}
          </span>
          <Button
            variant="outline"
            size="sm"
            disabled={!meta.has_next}
            onClick={() => setPage((p) => p + 1)}
          >
            Next
          </Button>
        </div>
      )}

      <Modal
        open={editingCharger !== null}
        onClose={() => setEditingCharger(null)}
        title="Edit Charger"
      >
        {editingCharger && (
          <ChargerForm
            charger={editingCharger}
            onSubmit={handleUpdate}
            onCancel={() => setEditingCharger(null)}
            loading={updateCharger.isPending}
          />
        )}
      </Modal>
    </div>
  )
}
