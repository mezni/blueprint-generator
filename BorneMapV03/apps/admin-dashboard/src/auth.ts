import { KeycloakAdapter, TokenStorage } from 'auth-client';

const storage = new TokenStorage();
const adapter = new KeycloakAdapter({
  realm: 'ev-platform',
  clientId: 'admin-dashboard',
  redirectUri: window.location.origin + '/admin',
  silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html',
}, storage);

export async function initAuth(): Promise<void> {
  const token = await adapter.getToken();
  if (!token) {
    await adapter.login();
    return;
  }
  showApp();
}

function showApp(): void {
  const user = adapter.getUser();
  document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
    <div style="display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:100vh;gap:16px;">
      <h1>Admin Dashboard</h1>
      <p>Welcome, <strong>${user?.displayName || 'Admin'}</strong></p>
      <p>Role: ${user?.roles.join(', ') || 'N/A'}</p>
      <button id="logout-btn" style="padding:8px 16px;border-radius:6px;border:1px solid #ccc;cursor:pointer;">
        Log Out
      </button>
    </div>
  `;
  document.querySelector('#logout-btn')!.addEventListener('click', () => {
    adapter.logout();
  });
}
