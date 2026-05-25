import { useState } from "react";
import UsersTable from "../features/users/UsersTable";
import UserDialog from "../features/users/UserDialog";
import { useUsers, useCreateUser, useUpdateUser, useToggleActive, useDeleteUser } from "../features/users/api";
import type { User, UserCreate, UserUpdate } from "../features/users/types";

export default function UsersPage() {
  const { data: users, isLoading } = useUsers();
  const createUser = useCreateUser();
  const updateUser = useUpdateUser();
  const toggleActive = useToggleActive();
  const deleteUser = useDeleteUser();

  const [dialogOpen, setDialogOpen] = useState(false);
  const [editingUser, setEditingUser] = useState<User | null>(null);

  const handleAdd = () => {
    setEditingUser(null);
    setDialogOpen(true);
  };

  const handleEdit = (user: User) => {
    setEditingUser(user);
    setDialogOpen(true);
  };

  const handleSave = (data: UserCreate | UserUpdate) => {
    if (editingUser) {
      updateUser.mutate(
        { id: editingUser.id, data: data as UserUpdate },
        { onSuccess: () => setDialogOpen(false) },
      );
    } else {
      createUser.mutate(data as UserCreate, {
        onSuccess: () => setDialogOpen(false),
      });
    }
  };

  const handleToggleActive = (user: User) => {
    toggleActive.mutate(user.id);
  };

  const handleDelete = (user: User) => {
    if (window.confirm(`Delete user "${user.username}"? This cannot be undone.`)) {
      deleteUser.mutate(user.id);
    }
  };

  const saving = createUser.isPending || updateUser.isPending;

  return (
    <div className="mx-auto max-w-4xl p-6">
      <div className="mb-6">
        <h1 className="text-lg font-semibold text-slate-900">Users</h1>
        <p className="text-sm text-muted">Manage admin portal user accounts</p>
      </div>

      {isLoading ? (
        <div className="flex items-center justify-center py-12">
          <div className="h-6 w-6 animate-spin rounded-full border-2 border-accent border-t-transparent" />
        </div>
      ) : (
        <div className="rounded-2xl border border-border bg-white p-6 shadow-card">
          <UsersTable
            users={users ?? []}
            onEdit={handleEdit}
            onToggleActive={handleToggleActive}
            onDelete={handleDelete}
            onAdd={handleAdd}
          />
        </div>
      )}

      <UserDialog
        open={dialogOpen}
        onClose={() => setDialogOpen(false)}
        onSave={handleSave}
        user={editingUser}
        saving={saving}
      />
    </div>
  );
}
