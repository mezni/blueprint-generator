import { useState } from 'react'
import { useAdminStations, useUpdateStation, useDeleteStation } from '@/hooks/useAdminStations'
import { Modal } from '@/components/Modal'
import { StationForm } from '@/components/StationForm'
import { Button } from '@/components/ui/button'
import type { Station, StationUpdate } from '@/lib/types'

export default function StationsPage() {
  const [showDeleted, setShowDeleted] = useState(false)
  const { data, isLoading, isError, refetch } = useAdminStations(showDeleted)
  const updateStation = useUpdateStation()
  const deleteStation = useDeleteStation()

  const [editStation, setEditStation] = useState<Station | null>(null)
  const [deleteId, setDeleteId] = useState<string | null>(null)
  const [expandedId, setExpandedId] = useState<string | null>(null)

  const handleSave = async (data: StationUpdate) => {
    if (!editStation) return
    await updateStation.mutateAsync({ id: editStation.id, data })
    setEditStation(null)
  }

  const handleDelete = async () => {
    if (!deleteId) return
    await deleteStation.mutateAsync(deleteId)
    setDeleteId(null)
  }

  if (isLoading) {
    return (
      <div className="space-y-3">
        {[...Array(5)].map((_, i) => (
          <div key={i} className="h-12 animate-pulse rounded bg-[var(--color-surface-base)] border border-[var(--color-border-muted)]" />
        ))}
      </div>
    )
  }

  if (isError) {
    return (
      <div className="flex flex-col items-center gap-3 py-20">
        <p className="text-[var(--color-text-muted)]">Failed to load stations</p>
        <Button onClick={() => refetch()}>Retry</Button>
      </div>
    )
  }

  const stations = data?.data ?? []

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-xl font-bold text-[var(--color-text-base)]">Stations</h1>
        <label className="flex items-center gap-2 text-sm text-[var(--color-text-muted)]">
          <input type="checkbox" checked={showDeleted} onChange={(e) => setShowDeleted(e.target.checked)} />
          Show deleted
        </label>
      </div>

      {stations.length === 0 ? (
        <div className="flex flex-col items-center gap-3 py-20">
          <p className="text-[var(--color-text-muted)]">No stations found</p>
        </div>
      ) : (
        <div className="overflow-x-auto rounded-lg border border-[var(--color-border-muted)]">
          <table className="w-full text-sm">
            <thead>
              <tr className="bg-[var(--color-surface-hover)] text-left text-[var(--color-text-muted)]">
                <th className="px-4 py-3 font-medium">Name</th>
                <th className="px-4 py-3 font-medium">Partner</th>
                <th className="px-4 py-3 font-medium">Status</th>
                <th className="px-4 py-3 font-medium">City</th>
                <th className="px-4 py-3 font-medium">Live</th>
                <th className="px-4 py-3 font-medium">Public</th>
                <th className="px-4 py-3 font-medium">Chargers</th>
                <th className="px-4 py-3 font-medium">Actions</th>
              </tr>
            </thead>
            <tbody>
              {stations.map((s: Station) => (
                <>
                  <tr
                    key={s.id}
                    className={`border-t border-[var(--color-border-muted)] hover:bg-[var(--color-surface-hover)] cursor-pointer ${s.deleted_at ? 'opacity-50' : ''}`}
                    onClick={() => setExpandedId(expandedId === s.id ? null : s.id)}
                  >
                    <td className="px-4 py-3 text-[var(--color-text-base)]">{s.name}</td>
                    <td className="px-4 py-3 text-[var(--color-text-muted)]">{s.partner_name}</td>
                    <td className="px-4 py-3">
                      <span className={`inline-block rounded-full px-2 py-0.5 text-xs font-medium ${
                        s.status === 'active' ? 'bg-[var(--color-success-muted)] text-[var(--color-success-base)]'
                          : s.status === 'maintenance' ? 'bg-[var(--color-warning-muted)] text-[var(--color-warning-base)]'
                          : 'bg-[var(--color-surface-muted)] text-[var(--color-text-muted)]'
                      }`}>
                        {s.status}
                      </span>
                    </td>
                    <td className="px-4 py-3 text-[var(--color-text-muted)]">{s.city ?? '-'}</td>
                    <td className="px-4 py-3">{s.is_live ? '✅' : '❌'}</td>
                    <td className="px-4 py-3">{s.is_public ? '✅' : '❌'}</td>
                    <td className="px-4 py-3 text-[var(--color-text-muted)]">{s.chargers?.length ?? 0}</td>
                    <td className="px-4 py-3">
                      <div className="flex gap-2">
                        <button
                          onClick={(e) => { e.stopPropagation(); setEditStation(s) }}
                          className="text-sm text-[var(--color-primary-base)] hover:underline"
                        >
                          Edit
                        </button>
                        <button
                          onClick={(e) => { e.stopPropagation(); setDeleteId(s.id) }}
                          className="text-sm text-[var(--color-error-base)] hover:underline"
                        >
                          Delete
                        </button>
                      </div>
                    </td>
                  </tr>
                  {expandedId === s.id && s.chargers?.length > 0 && (
                    <tr key={`${s.id}-chargers`}>
                      <td colSpan={8} className="bg-[var(--color-surface-hover)] px-6 py-3">
                        <div className="text-sm">
                          <p className="font-medium text-[var(--color-text-base)] mb-2">Chargers</p>
                          <div className="grid grid-cols-4 gap-4">
                            {s.chargers.map((c) => (
                              <div key={c.id} className="rounded border border-[var(--color-border-muted)] bg-[var(--color-surface-base)] p-3">
                                <p className="font-medium text-[var(--color-text-base)]">{c.type}</p>
                                <p className="text-[var(--color-text-muted)]">{c.power_kw} kW</p>
                                <span className={`inline-block rounded-full px-2 py-0.5 text-xs font-medium mt-1 ${
                                  c.status === 'available' ? 'bg-[var(--color-success-muted)] text-[var(--color-success-base)]'
                                    : 'bg-[var(--color-surface-muted)] text-[var(--color-text-muted)]'
                                }`}>
                                  {c.status}
                                </span>
                              </div>
                            ))}
                          </div>
                        </div>
                      </td>
                    </tr>
                  )}
                </>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {editStation && (
        <Modal open onClose={() => setEditStation(null)} title="Edit Station">
          <StationForm
            initial={editStation}
            onSave={handleSave}
            onCancel={() => setEditStation(null)}
          />
        </Modal>
      )}

      {deleteId && (
        <Modal open onClose={() => setDeleteId(null)} title="Delete Station">
          <p className="text-[var(--color-text-base)] mb-4">Are you sure you want to soft-delete this station?</p>
          <div className="flex justify-end gap-2">
            <Button variant="outline" onClick={() => setDeleteId(null)}>Cancel</Button>
            <Button onClick={handleDelete} disabled={deleteStation.isPending}>
              {deleteStation.isPending ? 'Deleting...' : 'Delete'}
            </Button>
          </div>
        </Modal>
      )}
    </div>
  )
}
