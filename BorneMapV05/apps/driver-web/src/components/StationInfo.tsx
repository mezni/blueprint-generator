import { Card, CardContent } from './ui/card'

interface StationInfoProps {
  name: string
  description: string | null
  city: string | null
  country: string | null
  distanceKm: number | null
}

function StationInfo({ name, description, city, country, distanceKm }: StationInfoProps) {
  const address = [city, country].filter(Boolean).join(', ') || null

  return (
    <Card>
      <CardContent className="flex flex-col gap-2">
        <h3 className="text-xl font-bold text-[var(--color-text-base)]">{name}</h3>
        {description && (
          <p className="text-sm text-[var(--color-text-muted)]">{description}</p>
        )}
        {address && (
          <p className="text-sm text-[var(--color-text-muted)]">{address}</p>
        )}
        {distanceKm != null && (
          <p className="text-sm font-medium text-[var(--color-primary-base)]">
            {distanceKm.toFixed(1)} km away
          </p>
        )}
      </CardContent>
    </Card>
  )
}

export default StationInfo
