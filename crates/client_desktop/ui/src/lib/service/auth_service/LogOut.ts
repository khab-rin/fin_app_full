import {invoke} from "@tauri-apps/api/core";
import {currAuthStep} from "$lib/models/svelte_models/auth_service/SvelteAuthStep.svelte";

export async function logOut() {
    try {
        await invoke("cmd_logout");

        currAuthStep.reset();
    } catch (err) {
        console.error("LogOut ERROR:", err);
    }
}