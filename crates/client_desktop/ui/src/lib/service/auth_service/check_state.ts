import { invoke } from "@tauri-apps/api/core";
import type {AuthStep} from "$lib/models/AuthStep"

export async function checkSessionInit(): Promise<AuthStep> {
    try {
        return await invoke<AuthStep>("cmd_is_state_active_init");
    } catch (error) {
        console.error("COMMAND is_state_active_init FAILED, ERR = ", error);
        return {Init: {}};
    }
}