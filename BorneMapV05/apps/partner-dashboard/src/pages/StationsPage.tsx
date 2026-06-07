import { useState } from 'react'
import { usePartnerStations, useCreateStation, useUpdateStation, useDeleteStation } from '@/hooks/usePartnerStations'
import { useUpdateAvailability } from '@/hooks/usePartnerAvailability'
import { usePartnerChargers, useCreateCharger } from '@/hooks/usePartnerChargers'
import type { Station, StationCreate, StationUpdate, StationAvailabilityStatus } from '@/lib/types'
import { Button } from '@/components/ui/button'
import { Modal } from '@/components/Modal'
import { StationForm } from '@/components/StationForm'
import { ChargerForm } from '@/components/ChargerForm'

const availabilityColors: Record<StationAvailabilityStatus, string> = {
  available: 'bg-[var(--color-success-base)]',
  limited: 'bg-[var(--color-warning-base)]',
  unavailable: 'bg-[var(--color-error-base)]',
}

const statusColors: Record<string, string> = {
  active: 'text-[var(--color-success-base)]',
  inactive: 'text-[var(--color-text-muted)]',
  maintenance: 'text-[var(--color-warning-base)]',
  draft: 'text-[var(--color-secondary-base)]',
}

export function StationsPage() {
  const [page, setPage] = useState(1)
  const { data, isLoading } = usePartnerStations(page)
  const createStation = useCreateStation()
  const updateStation = useUpdateStation()
  const deleteStation = useDeleteStation()
  const updateAvailability = useUpdateAvailability()
  const createCharger = useCreateCharger()

  const [showCreateModal, setShowCreateModal] = useState(false)
  const [editingStation, setEditingStation] = useState<Station | null>(null)
  const [expandedStation, setExpandedStation] = useState<string | null>(null)
  const [showChargerModal, setShowChargerModal] = useState<string | null>(null)

  const stations = data?.data ?? []
  const meta = data?.meta

  const { data: chargersData } = usePartnerChargers(expandedStation ?? undefined)

  const handleCreate = async (data: StationCreate | StationUpdate) => {
    await createStation.mutateAsync(data as StationCreate)
    setShowCreateModal(false)
  }

  const handleUpdate = async (data: StationCreate | StationUpdate) => {
    if (!editingStation) return
    await updateStation.mutateAsync({
      id: editingStation.station_id,
      data: data as StationUpdate,
      etag: editingStation.updated_at,
    })
    setEditingStation(null)
  }

  const handleDelete = async (station: Station) => {
    if (!confirm(`Delete station "${station.name}"?`)) return
    await deleteStation.mutateAsync(station.station_id)
  }

  const handleAvailability = async (station: Station, status: StationAvailabilityStatus) => {
    await updateAvailability.mutateAsync({ stationId: station.station_id, data: { status } })
  }

  const handleAddCharger = async (data: any) => {
    if (!showChargerModal) return
    await createCharger.mutateAsync({
      station_id: showChargerModal,
      ...data,
    } as any)
    setShowChargerModal(null)
  }

  return (
    <div className="p-6">
      <div className="mb-6 flex items-center justify-between">
        <h1 className="text-2xl font-bold text-[var(--color-text-base)]">Stations</h1>
        <Button onClick={() => setShowCreateModal(true)}>New Station</Button>
      </div>

      {isLoading ? (
        <div className="text-[var(--color-text-muted)] py-8 text-center">Loading stations...</div>
      ) : stations.length === 0 ? (
        <div className="text-[var(--color-text-muted)] py-8 text-center">
          No stations yet. Create your first station.
        </div>
      ) : (
        <div className="space-y-3">
          {stations.map((station) => {
            const isExpanded = expandedStation === station.station_id
            return (
              <div
                key={station.station_id}
                className="rounded-lg border border-[var(--color-border-muted)] bg-[var(--color-surface-base)] shadow-card"
              >
                <div
                  className="flex items-center justify-between p-4 cursor-pointer hover:bg-[var(--color-surface-hover)] transition-colors"
                  onClick={() => setExpandedStation(isExpanded ? null : station.station_id)}
                >
                  <div className="flex items-center gap-4 min-w-0">
                    <div>
                      <div className="flex items-center gap-2">
                        <span className="font-medium text-[var(--color-text-base)]">
                          {station.name}
                        </span>
                        <span className={`text-xs font-medium ${statusColors[station.status]}`}>
                          {station.status}
                        </span>
                      </div>
                      {station.address && (
                        <p className="text-sm text-[var(--color-text-muted)] truncate">
                          {station.address}
                        </p>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-3">
                    <div className="flex items-center gap-1">
                      {(['available', 'limited', 'unavailable'] as StationAvailabilityStatus[]).map(
                        (s) => (
                          <button
                            key={s}
                            onClick={(e) => {
                              e.stopPropagation()
                              handleAvailability(station, s)
                            }}
                            className={`h-3 w-3 rounded-full transition-opacity ${
                              availabilityColors[s]
                            } ${
                              station.availability_status === s
                                ? 'opacity-100 ring-2 ring-[var(--color-primary-muted)]'
                                : 'opacity-40 hover:opacity-70'
                            }`}
                            title={`Set ${s}`}
                          />
                        ),
                      )}
                    </div>
                    <button
                      onClick={(e) => {
                        e.stopPropagation()
                        setEditingStation(station)
                      }}
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
                    <button
                      onClick={(e) => {
                        e.stopPropagation()
                        handleDelete(station)
                      }}
                      className="rounded-md p-1.5 text-[var(--color-text-muted)] hover:bg-[var(--color-surface-active)] hover:text-[var(--color-error-base)] transition-colors"
                    >
                      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <path
                          d="M2 4h12M5 4V2.5A.5.5 0 015.5 2h5a.5.5 0 01.5.5V4m-6 0h6M4 4l1 10h6l1-10"
                          stroke="currentColor"
                          strokeWidth="1.5"
                          strokeLinecap="round"
                          strokeLinejoin="round"
                        />
                      </svg>
                    </button>
                    <svg
                      width="16"
                      height="16"
                      viewBox="0 0 16 16"
                      fill="none"
                      className={`text-[var(--color-text-muted)] transition-transform ${
                        isExpanded ? 'rotate-180' : ''
                      }`}
                    >
                      <path
                        d="M4 6l4 4 4-4"
                        stroke="currentColor"
                        strokeWidth="1.5"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                      />
                    </svg>
                  </div>
                </div>

                {isExpanded && (
                  <div className="border-t border-[var(--color-border-muted)] p-4">
                    <div className="mb-3 flex items-center justify-between">
                      <h3 className="text-sm font-semibold text-[var(--color-text-base)]">
                        Chargers
                      </h3>
                      <Button
                        size="sm"
                        onClick={() => setShowChargerModal(station.station_id)}
                      >
                        Add Charger
                      </Button>
                    </div>
                    {chargersData?.data && chargersData.data.length > 0 ? (
                      <div className="space-y-2">
                        {chargersData.data
                          .filter((c) => c.station_id === station.station_id)
                          .map((charger) => (
                            <div
                              key={charger.charger_id}
                              className="flex items-center justify-between rounded-md bg-[var(--color-surface-hover)] px-3 py-2 text-sm"
                            >
                              <div className="flex items-center gap-3">
                                <span className="font-medium text-[var(--color-text-base)]">
                                  {charger.charger_type}
                                </span>
                                <span className="text-[var(--color-text-muted)]">
                                  {charger.power_kw} kW
                                </span>
                              </div>
                              <span
                                className={`text-xs font-medium ${
                                  charger.status === 'available'
                                    ? 'text-[var(--color-success-base)]'
                                    : charger.status === 'offline'
                                      ? 'text-[var(--color-text-muted)]'
                                      : 'text-[var(--color-error-base)]'
                                }`}
                              >
                                {charger.status}
                              </span>
                            </div>
                          ))}
                      </div>
                    ) : (
                      <p className="text-sm text-[var(--color-text-muted)]">
                        No chargers on this station.
                      </p>
                    )}
                  </div>
                )}
              </div>
            )
          })}
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
        open={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        title="New Station"
      >
        <StationForm
          onSubmit={handleCreate}
          onCancel={() => setShowCreateModal(false)}
          loading={createStation.isPending}
        />
      </Modal>

      <Modal
        open={editingStation !== null}
        onClose={() => setEditingStation(null)}
        title="Edit Station"
      >
        {editingStation && (
          <StationForm
            station={editingStation}
            onSubmit={handleUpdate}
            onCancel={() => setEditingStation(null)}
            loading={updateStation.isPending}
          />
        )}
      </Modal>

      <Modal
        open={showChargerModal !== null}
        onClose={() => setShowChargerModal(null)}
        title="Add Charger"
      >
        <ChargerForm
          stationId={showChargerModal ?? undefined}
          onSubmit={handleAddCharger}
          onCancel={() => setShowChargerModal(null)}
          loading={createCharger.isPending}
        />
      </Modal>
    </div>
  )
}
