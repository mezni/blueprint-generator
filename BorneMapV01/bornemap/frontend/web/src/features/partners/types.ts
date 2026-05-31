export interface Partner {
  id: number;
  name: string;
  contact_email: string | null;
  phone: string | null;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface PartnerCreate {
  name: string;
  contact_email?: string;
  phone?: string;
  is_active?: boolean;
}

export interface PartnerUpdate {
  name?: string;
  contact_email?: string;
  phone?: string;
  is_active?: boolean;
}
