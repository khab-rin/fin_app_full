import {PageType} from "$lib/models/MainManager/PageValues";

import AuthManager from "$lib/service/auth_service/AuthManager.svelte";
import MchdManager from "$lib/service/mchd/MchdManager.svelte";
import OperationManager from "$lib/service/operation/OperationManager.svelte";

class PageManager {

    settingsOnOff = $state(false);

    Page = $state<string | null>(PageType.Auth);

    get getPage() {
        switch (this.Page) {
            case PageType.Auth: return AuthManager;
            case PageType.Mchd: return MchdManager;
            case PageType.Operation: return OperationManager;
            default: return null;
        }
    }

    totalOff = $derived(this.Page == PageType.Auth);
}

export const pageManager = new PageManager();