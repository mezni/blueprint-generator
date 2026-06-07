import Keycloak from 'keycloak-js'

let keycloak: Keycloak | null = null
let initPromise: Promise<boolean> | null = null
let initAttempted = false

function getKeycloak(): Keycloak | null {
  if (!keycloak && !initAttempted) {
    try {
      keycloak = new Keycloak({
        url: 'http://localhost/auth',
        realm: 'bornemap',
        clientId: 'bornemap-api',
      })
    } catch {
      return null
    }
  }
  return keycloak
}

export async function initAuth(): Promise<boolean> {
  if (initPromise) return initPromise

  const kc = getKeycloak()
  if (!kc) {
    initAttempted = true
    initPromise = Promise.resolve(false)
    return false
  }

  initAttempted = true
  initPromise = kc
    .init({
      onLoad: 'check-sso',
      pkceMethod: 'S256',
      silentCheckSsoRedirectUri:
        window.location.origin + '/silent-check-sso.html',
      checkLoginIframe: false,
    })
    .then((authenticated) => authenticated)
    .catch(() => false)

  return initPromise
}

export async function getToken(): Promise<string | null> {
  try {
    const kc = getKeycloak()
    if (!kc || !kc.authenticated) return null
    await kc.updateToken(5).catch(() => {})
    return kc.token ?? null
  } catch {
    return null
  }
}

export async function login(provider?: string): Promise<void> {
  const kc = getKeycloak()
  if (!kc) return
  if (provider) {
    await kc.login({ idpHint: provider })
  } else {
    await kc.login({ redirectUri: window.location.origin })
  }
}

export async function logout(): Promise<void> {
  const kc = getKeycloak()
  if (!kc) return
  await kc.logout({ redirectUri: window.location.origin }).catch(() => {
    window.location.href = window.location.origin
  })
}

export function isAuthenticated(): boolean {
  try {
    return getKeycloak()?.authenticated ?? false
  } catch {
    return false
  }
}

export function getUser(): { id?: string; email?: string; name?: string } | null {
  try {
    const kc = getKeycloak()
    if (!kc?.authenticated || !kc.tokenParsed) return null
    return {
      id: kc.subject,
      email: kc.tokenParsed.email as string | undefined,
      name: kc.tokenParsed.name as string | undefined,
    }
  } catch {
    return null
  }
}
