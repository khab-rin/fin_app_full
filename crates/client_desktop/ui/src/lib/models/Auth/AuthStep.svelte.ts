import type { AuthStep } from '$lib/models/rustModels/AuthStep';
import {AuthStepType} from "$lib/models/Auth/AuthValues";

import {FieldValidator} from "$lib/models/Auth/FieldValidator.svelte";

import { pageManager } from '../MainManager/MainManager.svelte';
import { PageType } from '../MainManager/PageValues';

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
        if (AuthStepType.SuccessShort in next_step) {
            pageManager.Page = null;
            this.data.password.value = "";
        }
        
        this.steps = [...this.steps.slice(0, this.index + 1), next_step];
        
        this.index++;
        this.step = next_step;
    }

    reset() {
        const next_step: AuthStep = { Loading: {text: "Страница загружается, подождите пожалуйста. В случае зависания попробуйте обновить или перезагрузить приложение"}};
        this.index = -1;
        pageManager.Page = PageType.Auth;
        this.add(next_step);
    }

    get isAuthorized() {
        return AuthStepType.SuccessShort in this.step;
    }

    data = $state({
        nick: new FieldValidator("String1_50", ""),
        surName: new FieldValidator("SurName", ""),
        firstName: new FieldValidator("FirstName", ""),
        midName: new FieldValidator("MidName", ""),
        persInn: new FieldValidator("PersInn", ""),
        snils: new FieldValidator("Snils", ""),
        compInn: new FieldValidator("CompInn", ""),
        kpp: new FieldValidator("Kpp", ""),
        password: new FieldValidator("Password", ""),
        phone: new FieldValidator("Phone", ""),
        email: new FieldValidator("Email", ""),
    })

    nick_names = $state<string[]>([]);

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

    set currentText(value: Partial<Record<keyof AuthStep, string>>) {
        if (!this.step || typeof this.step !== 'object') {
            return;
        }

        const stepKey = Object.keys(this.step)[0] as keyof AuthStep;
        if (!stepKey) return;

        if (value && typeof value === 'object' && stepKey in value) {
            const newText = value[stepKey];
            
            if (typeof newText === 'string') {
                const currentStepObj = this.step[stepKey];

                if (currentStepObj && typeof currentStepObj === 'object' && 'text' in currentStepObj) {
                    this.step = {
                        ...this.step,
                        [stepKey]: {
                            ...(currentStepObj as Record<string, unknown>),
                            text: newText
                        }
                    };
                }
            }
        }
    }

    private async init() {
        try {
            const names = await invoke<string[]>('cmd_get_nick_names');
            this.nick_names = names;

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