import {currAuthStep} from "$lib/models/Auth/AuthStep.svelte";

import AuthManager from "$lib/service/auth_service/AuthManager.svelte";

class SvelteAppState {
    totalOff = $derived(!currAuthStep.isAuthorized);
    settingsOnOff = $state(false);

    Page: string | null = $state("auth");

    getPage = $derived.by(() => {
        switch (this.Page) {
            case "auth": return AuthManager;
            default: return null;
        }
    });
}

export const appState = new SvelteAppState();