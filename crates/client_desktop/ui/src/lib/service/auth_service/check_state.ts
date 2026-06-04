import { invoke } from "@tauri-apps/api/core";
import { currAuthStep } from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";
import type {AuthStep} from "$lib/models/AuthStep"

export async function checkSessionInit() {
    try {
        const next_step: AuthStep  = await invoke<AuthStep>("cmd_is_state_active_init");
        currAuthStep.add(next_step);
    } catch (error) {
        console.error("COMMAND is_state_active_init FAILED, ERR = ", error);
        const next_step: AuthStep = {TryLater: {text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение"}};
        currAuthStep.add(next_step);
    }
}