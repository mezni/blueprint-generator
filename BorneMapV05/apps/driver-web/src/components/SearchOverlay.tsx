import { useState, useRef, useEffect } from 'react'
import { useSearch } from '@/hooks/useSearch'
import { useClickstream } from '@/hooks/useClickstream'
import SearchResults from './SearchResults'
import { Input } from './ui/input'

interface SearchOverlayProps {
  isOpen: boolean
  onClose: () => void
  onSelectStation: (stationId: string) => void
}

function SearchOverlay({ isOpen, onClose, onSelectStation }: SearchOverlayProps) {
  const [query, setQuery] = useState('')
  const { data, isLoading, isError } = useSearch({ q: query })
  const { emit } = useClickstream()

  const prevResultCount = useRef<number | null>(null)
  useEffect(() => {
    if (data && prevResultCount.current !== data.totalResults) {
      prevResultCount.current = data.totalResults
      emit('search.performed', { query, resultCount: data.totalResults })
    }
  }, [data, query, emit])

  const hadError = useRef(false)
  useEffect(() => {
    if (isError && !hadError.current) {
      hadError.current = true
      emit('search.failed', { query })
    } else if (!isError) {
      hadError.current = false
    }
  }, [isError, query, emit])

  if (!isOpen) return null

  return (
    <div className="absolute start-4 end-4 top-4 z-[2000] mx-auto max-w-md">
      <div className="rounded-xl border border-[var(--color-border-base)] bg-[var(--color-surface-base)] shadow-xl">
        <div className="flex items-center gap-2 border-b border-[var(--color-border-base)] px-3 py-2">
          <Input
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search stations..."
            className="border-0 focus-visible:ring-0"
            autoFocus
          />
          <button
            onClick={onClose}
            className="rounded p-1 text-sm text-[var(--color-text-muted)] hover:bg-[var(--color-surface-hover)]"
          >
            ✕
          </button>
        </div>
        <div className="max-h-80 overflow-y-auto p-3">
          {isLoading && (
            <div className="flex items-center justify-center py-4">
              <div className="h-6 w-6 animate-spin rounded-full border-4 border-[var(--color-border-base)] border-t-[var(--color-primary-base)]" />
            </div>
          )}
          {isError && (
            <p className="py-4 text-center text-sm text-[var(--color-error-base)]">
              Search failed. Please try again.
            </p>
          )}
          {data && !isError && (
            <SearchResults
              results={data.results}
              totalResults={data.totalResults}
              onSelectStation={(id) => {
                onSelectStation(id)
                onClose()
              }}
            />
          )}
        </div>
      </div>
    </div>
  )
}

export default SearchOverlay
