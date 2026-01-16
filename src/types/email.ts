export interface EmailHeader {
  id: string;
  from_address: string;
  from_name: string | null;
  subject: string;
  received_at: number;
  is_unread: boolean;
  is_important: boolean;
  snippet: string | null;
}

export interface EmailConfig {
  enabled: boolean;
  imap_server: string | null;
  imap_port: number | null;
  username: string | null;
}
