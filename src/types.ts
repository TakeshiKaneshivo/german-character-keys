export type BackendStatus = "running" | "disabled" | "permission_required" | "initialization_failed";

export type Status = {
  enabled: boolean;
  shortcut_registered: boolean;
  accessibility_granted: boolean;
  shortcut: string;
  platform: string;
  launch_at_login: boolean;
  backend_status: BackendStatus;
  message: string | null;
};

export type OperationResult = {
  success: boolean;
  message?: string | null;
  status: Status;
};
