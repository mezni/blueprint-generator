import type { Charger, ChargerTypeInfo, ConnectorType, ChargerStatus } from '@/lib/types'

interface ChargerListProps {
  chargers: Charger[]
  chargerTypes: ChargerTypeInfo[]
}

const connectorColors: Record<ConnectorType, string> = {
  CCS: 'bg-blue-100 text-blue-800',
  Type2: 'bg-green-100 text-green-800',
  CHAdeMO: 'bg-purple-100 text-purple-800',
}

const statusColors: Record<ChargerStatus, string> = {
  available: 'bg-green-500',
  offline: 'bg-gray-400',
  fault: 'bg-red-500',
}

function ChargerList({ chargers, chargerTypes }: ChargerListProps) {
  if (chargers.length === 0) {
    return (
      <div className="rounded-lg border border-[var(--color-border-muted)] p-4 text-center text-sm text-[var(--color-text-muted)]">
        No chargers available
      </div>
    )
  }

  return (
    <div className="flex flex-col gap-2">
      <h4 className="text-sm font-semibold text-[var(--color-text-base)]">Chargers</h4>
      <div className="flex flex-wrap gap-2">
        {chargerTypes.map((ct, i) => (
          <span
            key={i}
            className={`rounded-full px-2.5 py-0.5 text-xs font-medium ${connectorColors[ct.connector_type]}`}
          >
            {ct.connector_type} {ct.power_kw ? `${ct.power_kw}kW` : ''}
          </span>
        ))}
      </div>
      <div className="mt-2 flex flex-col gap-2">
        {chargers.map((charger) => (
          <div
            key={charger.id}
            className="flex items-center gap-3 rounded-lg border border-[var(--color-border-muted)] p-3"
          >
            <div className={`h-3 w-3 rounded-full ${statusColors[charger.status]}`} />
            <div className="flex-1">
              <span className="text-sm font-medium text-[var(--color-text-base)]">
                {charger.connector_type}
              </span>
              {charger.power_kw && (
                <span className="ms-2 text-xs text-[var(--color-text-muted)]">
                  {charger.power_kw} kW
                </span>
              )}
            </div>
            <span className="text-xs capitalize text-[var(--color-text-muted)]">
              {charger.status}
            </span>
          </div>
        ))}
      </div>
    </div>
  )
}

export default ChargerList
