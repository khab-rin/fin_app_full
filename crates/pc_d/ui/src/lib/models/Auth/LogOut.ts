import {invoke} from "@tauri-apps/api/core";
import {currAuthStep} from "$lib/models/Auth/AuthStep.svelte";

export async function logOut() {
    try {
        await invoke("cmd_logout");

        currAuthStep.reset();
    } catch (err) {
        console.error("LogOut ERROR:", err);
    }
}