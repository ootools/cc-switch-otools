import { invoke } from "@tauri-apps/api/core";
import { isOtoolsPluginRuntime, openExternal } from "otools-plugin-sdk";

type InvokePayload = Record<string, unknown> | undefined;

export const isDesktopTauriRuntime = (): boolean => !isOtoolsPluginRuntime();

export async function invokeDesktopCommandOr<T>(
  command: string,
  payload: InvokePayload,
  fallback: T,
): Promise<T> {
  if (!isDesktopTauriRuntime()) {
    return fallback;
  }
  return await invoke<T>(command, payload);
}

export async function restartDesktopApp(): Promise<boolean> {
  return await invokeDesktopCommandOr("restart_app", undefined, false);
}

export async function checkForUpdatesOrOpenRelease(
  releaseUrl: string,
): Promise<void> {
  if (!isDesktopTauriRuntime()) {
    openExternal(releaseUrl);
    return;
  }

  await invoke("check_for_updates");
}

export async function exitDesktopApp(code = 0): Promise<boolean> {
  if (!isDesktopTauriRuntime()) {
    return false;
  }

  const { exit } = await import("@tauri-apps/plugin-process");
  await exit(code);
  return true;
}

export async function relaunchDesktopApp(): Promise<boolean> {
  if (!isDesktopTauriRuntime()) {
    return false;
  }

  const { relaunch } = await import("@tauri-apps/plugin-process");
  await relaunch();
  return true;
}
