import { KeycloakAdapter, TokenStorage } from 'auth-client';

const storage = new TokenStorage();
const adapter = new KeycloakAdapter({
  realm: 'ev-platform',
  clientId: 'partner-dashboard',
  redirectUri: window.location.origin + '/partner',
  silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html',
}, storage);

export async function initAuth(): Promise<void> {
  const token = await adapter.getToken();
  if (!token) {
    await adapter.login();
    return;
  }

  const user = adapter.getUser();
  const isAuthorized = user?.roles.includes('partner') || user?.roles.includes('admin');

  if (!isAuthorized) {
    document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
      <div style="display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:100vh;gap:16px;">
        <h1>Access Denied</h1>
        <p>You need a <strong>partner</strong> or <strong>admin</strong> role to access this dashboard.</p>
        <p>Your role: ${user?.roles.join(', ') || 'N/A'}</p>
        <button id="logout-btn" style="padding:8px 16px;border-radius:6px;border:1px solid #ccc;cursor:pointer;">
          Log Out
        </button>
      </div>
    `;
    document.querySelector('#logout-btn')!.addEventListener('click', () => {
      adapter.logout();
    });
    return;
  }

  showApp(user);
}

function showApp(user: any): void {
  document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
    <div style="display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:100vh;gap:16px;">
      <h1>Partner Dashboard</h1>
      <p>Welcome, <strong>${user?.displayName || 'Partner'}</strong></p>
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
