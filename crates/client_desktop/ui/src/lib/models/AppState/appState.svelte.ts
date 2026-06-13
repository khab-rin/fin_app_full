import {currAuthStep} from "$lib/models/Auth/AuthStep.svelte";

class SvelteAppState {
    totalOff = $derived(!currAuthStep.isAuthorized);
    settingsOnOff = $state(false);
}

export const appState = new SvelteAppState();