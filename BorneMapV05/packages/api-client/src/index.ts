export type HttpMethod = 'GET' | 'POST' | 'PATCH' | 'DELETE';

export interface ApiClientConfig {
  baseUrl: string;
  getToken?: () => Promise<string | null>;
}

export class ApiClient {
  private config: ApiClientConfig;

  constructor(config: ApiClientConfig) {
    this.config = config;
  }

  async request<T>(method: HttpMethod, path: string, body?: unknown, extraHeaders?: Record<string, string>): Promise<T> {
    const headers: Record<string, string> = { 'Content-Type': 'application/json' };
    const token = this.config.getToken ? await this.config.getToken() : null;
    if (token) headers['Authorization'] = `Bearer ${token}`;
    if (extraHeaders) Object.assign(headers, extraHeaders);

    const res = await fetch(`${this.config.baseUrl}${path}`, {
      method,
      headers,
      body: body ? JSON.stringify(body) : undefined,
    });
    return res.json() as Promise<T>;
  }

  get<T>(path: string, options?: { headers?: Record<string, string> }): Promise<T> { return this.request<T>('GET', path, undefined, options?.headers); }
  post<T>(path: string, body?: unknown, options?: { headers?: Record<string, string> }): Promise<T> { return this.request<T>('POST', path, body, options?.headers); }
  patch<T>(path: string, body?: unknown, options?: { headers?: Record<string, string> }): Promise<T> { return this.request<T>('PATCH', path, body, options?.headers); }
  delete<T>(path: string, options?: { headers?: Record<string, string> }): Promise<T> { return this.request<T>('DELETE', path, undefined, options?.headers); }
}
