export interface PaginationMeta {
  page: number;
  size: number;
  total: number;
  total_pages: number;
  has_next: boolean;
  has_prev: boolean;
}

export interface SuccessEnvelope<T = Record<string, unknown>> {
  success: true;
  data: T;
  meta: PaginationMeta;
}

export interface ItemEnvelope<T = Record<string, unknown>> {
  success: true;
  data: T;
  meta: Record<string, never>;
}

export interface ErrorEnvelope {
  success: false;
  error: {
    code: string;
    message: string;
    details?: Record<string, unknown>;
  };
}
