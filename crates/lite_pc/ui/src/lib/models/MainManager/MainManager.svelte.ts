import {invoke} from '@tauri-apps/api/core';

import {PageType} from "$lib/models/MainManager/PageValues";
import { currAuthStep } from '$lib/models/Auth/AuthStep.svelte';

import AuthManager from "$lib/service/auth_service/AuthManager.svelte";
import MchdManager from "$lib/service/mchd/MchdManager.svelte";
import OperationManager from "$lib/service/operation/OperationManager.svelte";
import ReportManager from "$lib/service/Reports/ReportManager.svelte";

class PageManager {

    settingsOnOff = $state(false);

    Page = $state<string | null>(PageType.Auth);

    get getPage() {
        switch (this.Page) {
            case PageType.Auth: return AuthManager;
            case PageType.Mchd: return MchdManager;
            case PageType.Operation: return OperationManager;
			case PageType.Report: return ReportManager;
            default: return null;
        }
    }

    totalOff = $derived(this.Page == PageType.Auth);

	private _compName = $state("");
	get compName() {
		return this._compName
	}
	set compName(value: string) {
		this._compName = value;
	}

	async logOut() {
		try {
			currAuthStep.reset();
			await invoke('cmd_logout', {});
			this.Page = PageType.Auth;
		} catch(err) {
			console.error("cmd_logout failed, err = ", err);
			currAuthStep.reset();	
			this.Page = PageType.Auth;			
		}
	}
}

export const pageManager = new PageManager();