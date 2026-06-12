import type { AuthStep } from '$lib/models/AuthStep';
import type { NickData } from '$lib/models/NickData';

import {AuthStepType} from "$lib/service/auth_service/AuthValues";
import {FieldValidator} from "$lib/service/validate/FieldValidator.svelte";
import { invoke } from '@tauri-apps/api/core';

class SvelteAuthStep {
    step = $state<AuthStep>({
        Loading: { 
            text: "Страница загружается, подождите пожалуйста. В случае зависания попробуйте обновить или перезагрузить приложение"
        }
    });

    private steps: AuthStep[] = $state([]);
    private index = $state(0);
    
    constructor() {
        this.steps.push(this.step);
        this.init();
    }

    next() {
        if (this.index < this.steps.length - 1) {
            this.index++;
            this.step = this.steps[this.index];
        }
    }

    back() {
        if (this.index > 0) {
            this.index--;
            this.step = this.steps[this.index];
        }
    }

    add(next_step: AuthStep) {
        if (AuthStepType.Loading in next_step) {
            this.step = next_step;
            return
        }
        if (AuthStepType.SuccessShort in next_step) {
            this.data.password.value = "";
        }
        this.steps.length = this.index + 1;
        this.steps.push(next_step);
        this.index++;
        this.step = next_step;
    }

    reset() {
        const next_step: AuthStep = { Loading: {text: "Страница загружается, подождите пожалуйста. В случае зависания попробуйте обновить или перезагрузить приложение"}};
        this.index = -1;
        this.add(next_step);
    }

    get isAuthorized() {
        return AuthStepType.SuccessShort in this.step;
    }

    data = $state({
        nick: new FieldValidator("String1_50"),
        surName: new FieldValidator("SurName"),
        firstName: new FieldValidator("FirstName"),
        midName: new FieldValidator("MidName"),
        persInn: new FieldValidator("PersInn"),
        snils: new FieldValidator("Snils"),
        compInn: new FieldValidator("CompInn"),
        kpp: new FieldValidator("Kpp"),
        password: new FieldValidator("Password"),
        phone: new FieldValidator("Phone"),
        email: new FieldValidator("Email"),
    })

    nick_names = $state<NickData>({ nick_names: [] });

    get currentText(): string {
        if (!this.step || typeof this.step !== 'object') {
            return '';
        }

        const currentStepObj = Object.values(this.step)[0];

        if (
            currentStepObj && 
            typeof currentStepObj === 'object' && 
            'text' in currentStepObj && 
            typeof (currentStepObj as { text: unknown }).text === 'string'
        ) {
            return (currentStepObj as { text: string }).text;
        }
        
        return '';
    }

    private async init() {
        try {
            const data = await invoke<NickData>('cmd_get_nick_names');
            this.nick_names = data; 

            const nextStep = await invoke<AuthStep>("cmd_is_state_active_init");
            
            this.add(nextStep);

        } catch (error) {
            console.error("Ошибка инициализации в синглтоне:", error);
            this.add({ 
                TryLater: { text: "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение" } 
            });
        }
    }
}

export const currAuthStep = new SvelteAuthStep();