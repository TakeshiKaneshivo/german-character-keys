import { computed, onMounted, ref } from "vue";
import { getStatus, refreshPermission, resetShortcut, setEnabled, setLaunchAtLogin, setShortcut } from "../tauri";
import type { OperationResult, Status } from "../types";

export function useAppStatus() {
  const status = ref<Status | null>(null);
  const error = ref("");
  const operationMessage = ref("");
  const busy = ref(false);
  const permissionFeedback = ref("");

  const blocked = computed(() => status.value?.platform === "macos" && !status.value.accessibility_granted);

  async function refresh() {
    status.value = await getStatus();
  }

  async function apply(operation: () => Promise<OperationResult>) {
    busy.value = true;
    error.value = "";
    operationMessage.value = "";
    try {
      const result = await operation();
      status.value = result.status;
      if (result.message) { operationMessage.value = result.message; permissionFeedback.value = result.message; }
      if (!result.success) error.value = result.message ?? "";
      return result;
    } catch (reason) {
      error.value = String(reason);
      try { await refresh(); } catch (refreshError) { error.value = String(refreshError); }
      throw reason;
    } finally {
      busy.value = false;
    }
  }

  onMounted(() => refresh().catch((reason) => { error.value = String(reason); }));
  return {
    status, error, operationMessage, busy, blocked, permissionFeedback, refresh,
    setEnabled: (value: boolean) => apply(() => setEnabled(value)),
    setShortcut: (value: string) => apply(() => setShortcut(value)),
    resetShortcut: () => apply(resetShortcut),
    setLaunchAtLogin: (value: boolean) => apply(() => setLaunchAtLogin(value)),
    checkPermission: () => apply(refreshPermission),
    clearPermissionFeedback: () => { permissionFeedback.value = ""; },
  };
}
