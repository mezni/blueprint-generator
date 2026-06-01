import { KeycloakAdapter, TokenStorage } from 'auth-client';

const storage = new TokenStorage();
const adapter = new KeycloakAdapter({
  realm: 'ev-platform',
  clientId: 'driver-web',
  redirectUri: window.location.origin,
  silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html',
}, storage);

export async function initAuth(): Promise<void> {
  const token = await adapter.getToken();
  if (!token) {
    showLoginButton();
    return;
  }
  showApp();
}

function showLoginButton(): void {
  const app = document.querySelector<HTMLDivElement>('#app')!;
  app.innerHTML = `
    <div style="display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:100vh;gap:16px;">
      <h1>BorneMap Driver Portal</h1>
      <p>Please log in to access the driver portal.</p>
      <button id="login-btn" style="padding:12px 24px;font-size:16px;border-radius:8px;border:none;background:#aa3bff;color:#fff;cursor:pointer;">
        Log In
      </button>
    </div>
  `;
  document.querySelector('#login-btn')!.addEventListener('click', () => {
    adapter.login();
  });
}

function showApp(): void {
  const user = adapter.getUser();
  document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
    <div style="display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:100vh;gap:16px;">
      <h1>Welcome, ${user?.displayName || 'Driver'}!</h1>
      <p>You are logged in as <strong>${user?.email || ''}</strong></p>
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
