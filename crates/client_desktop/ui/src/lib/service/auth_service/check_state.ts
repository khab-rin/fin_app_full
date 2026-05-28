import { invoke } from "@tauri-apps/api/core";

export async function checkSessionInit(): Promise<boolean> {
    try {
        const isActive = await invoke<boolean>("is_state_active_init");
        return isActive;
    } catch (error) {
        console.error("COMMAND is_state_active_init FAILED, ERR = ", error);
        return false;
    }
}