import { useState } from 'react'
import { useAdminPartners, useCreatePartner, useUpdatePartner, useDeletePartner } from '@/hooks/useAdminPartners'
import { Modal } from '@/components/Modal'
import { PartnerForm } from '@/components/PartnerForm'
import { Button } from '@/components/ui/button'
import type { Partner, PartnerCreate, PartnerUpdate } from '@/lib/types'

export default function PartnersPage() {
  const { data, isLoading, isError, refetch } = useAdminPartners()
  const createPartner = useCreatePartner()
  const updatePartner = useUpdatePartner()
  const deletePartner = useDeletePartner()

  const [modal, setModal] = useState<{ mode: 'create' | 'edit'; partner?: Partner } | null>(null)
  const [deleteId, setDeleteId] = useState<string | null>(null)
  const [deleteError, setDeleteError] = useState<string | null>(null)

  const handleSave = async (data: PartnerCreate | PartnerUpdate) => {
    if (modal?.mode === 'create') {
      await createPartner.mutateAsync(data as PartnerCreate)
    } else if (modal?.mode === 'edit' && modal.partner) {
      await updatePartner.mutateAsync({ id: modal.partner.id, data: data as PartnerUpdate })
    }
    setModal(null)
  }

  const handleDelete = async () => {
    if (!deleteId) return
    setDeleteError(null)
    try {
      await deletePartner.mutateAsync(deleteId)
      setDeleteId(null)
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Failed to delete partner'
      setDeleteError(msg)
    }
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
        <p className="text-[var(--color-text-muted)]">Failed to load partners</p>
        <Button onClick={() => refetch()}>Retry</Button>
      </div>
    )
  }

  const partners = data?.data ?? []

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-xl font-bold text-[var(--color-text-base)]">Partners</h1>
        <Button onClick={() => setModal({ mode: 'create' })}>Add Partner</Button>
      </div>

      {partners.length === 0 ? (
        <div className="flex flex-col items-center gap-3 py-20">
          <p className="text-[var(--color-text-muted)]">No partners yet</p>
          <Button onClick={() => setModal({ mode: 'create' })}>Create your first partner</Button>
        </div>
      ) : (
        <div className="overflow-x-auto rounded-lg border border-[var(--color-border-muted)]">
          <table className="w-full text-sm">
            <thead>
              <tr className="bg-[var(--color-surface-hover)] text-left text-[var(--color-text-muted)]">
                <th className="px-4 py-3 font-medium">Name</th>
                <th className="px-4 py-3 font-medium">Email</th>
                <th className="px-4 py-3 font-medium">Phone</th>
                <th className="px-4 py-3 font-medium">Status</th>
                <th className="px-4 py-3 font-medium">Actions</th>
              </tr>
            </thead>
            <tbody>
              {partners.map((p: Partner) => (
                <tr key={p.id} className="border-t border-[var(--color-border-muted)] hover:bg-[var(--color-surface-hover)]">
                  <td className="px-4 py-3 text-[var(--color-text-base)]">{p.name}</td>
                  <td className="px-4 py-3 text-[var(--color-text-muted)]">{p.email || '-'}</td>
                  <td className="px-4 py-3 text-[var(--color-text-muted)]">{p.phone || '-'}</td>
                  <td className="px-4 py-3">
                    <span className={`inline-block rounded-full px-2 py-0.5 text-xs font-medium ${
                      p.status === 'active' ? 'bg-[var(--color-success-muted)] text-[var(--color-success-base)]'
                        : 'bg-[var(--color-surface-muted)] text-[var(--color-text-muted)]'
                    }`}>
                      {p.status}
                    </span>
                  </td>
                  <td className="px-4 py-3">
                    <div className="flex gap-2">
                      <button
                        onClick={() => setModal({ mode: 'edit', partner: p })}
                        className="text-sm text-[var(--color-primary-base)] hover:underline"
                      >
                        Edit
                      </button>
                      <button
                        onClick={() => setDeleteId(p.id)}
                        className="text-sm text-[var(--color-error-base)] hover:underline"
                      >
                        Delete
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {modal && (
        <Modal
          open
          onClose={() => setModal(null)}
          title={modal.mode === 'create' ? 'Add Partner' : 'Edit Partner'}
        >
          <PartnerForm
            initial={modal.partner}
            onSave={handleSave}
            onCancel={() => setModal(null)}
          />
        </Modal>
      )}

      {deleteId && (
        <Modal open onClose={() => setDeleteId(null)} title="Delete Partner">
          <p className="text-[var(--color-text-base)] mb-4">
            Are you sure you want to delete this partner?
          </p>
          {deleteError && (
            <p className="text-sm text-[var(--color-error-base)] mb-4">{deleteError}</p>
          )}
          <div className="flex justify-end gap-2">
            <Button variant="outline" onClick={() => setDeleteId(null)}>Cancel</Button>
            <Button onClick={handleDelete} disabled={deletePartner.isPending}>
              {deletePartner.isPending ? 'Deleting...' : 'Delete'}
            </Button>
          </div>
        </Modal>
      )}
    </div>
  )
}
