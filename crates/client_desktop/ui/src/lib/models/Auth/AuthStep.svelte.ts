import type { AuthStep } from '$lib/models/rustModels/AuthStep';
import {AuthStepType} from "$lib/models/Auth/AuthValues";

import {FieldValidator} from "$lib/models/Auth/FieldValidator.svelte";

import { pageManager } from '../MainManager/MainManager.svelte';
import { PageType } from '../MainManager/PageValues';

import CallIn from "$lib/service/auth_service/CallIn.svelte";
import Loading from "$lib/service/auth_service/Loading.svelte";
import NickName from "$lib/service/auth_service/NickName.svelte";
import Password from "$lib/service/auth_service/PassWord.svelte";
import RegisterStep1 from "$lib/service/auth_service/RegisterStep1.svelte";
import RegisterStep1Duplicate from '$lib/service/auth_service/RegisterStep1Duplicate.svelte';
import RegisterStep1Success from '$lib/service/auth_service/RegisterStep1Success.svelte';
import RegisterStep2 from "$lib/service/auth_service/RegisterStep2.svelte";
import TryLater from "$lib/service/auth_service/TryLater.svelte";



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
        surName: new FieldValidator("SurName", "Хабипов"),
        firstName: new FieldValidator("FirstName", "Ринат"),
        midName: new FieldValidator("MidName", "Ришатович"),
        persInn: new FieldValidator("PersInn", "161101510882"),
        snils: new FieldValidator("Snils", "11021217665"),
        compInn: new FieldValidator("CompInn", "1655476106"),
        kpp: new FieldValidator("Kpp", "165501001"),
        password: new FieldValidator("Password", "asdfasdf"),
        phone: new FieldValidator("Phone", "+79173949166"),
        email: new FieldValidator("Email", "Oreolkazan@gmail.com"),
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

    get getPage() {
        const step = this.step;
        if (AuthStepType.CallIn in step) {
            return CallIn
        } else if (AuthStepType.Loading in step) {
            return Loading
        } else if (AuthStepType.NickName in step) {
            return NickName
        } else if (AuthStepType.Password in step) {
            return Password
        } else if (AuthStepType.RegisterStep1 in step) {
            return RegisterStep1
        } else if (AuthStepType.RegisterStep1Duplicate in step) {
            return RegisterStep1Duplicate
        } else if (AuthStepType.RegisterStep1Success in step) {
            return RegisterStep1Success
        } else if (AuthStepType.RegisterStep2 in step) {
            return RegisterStep2
        } else {
            return TryLater
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

    async updateNickNames() {
        try {
            const names = await invoke<string[]>('cmd_get_nick_names');
            this.nick_names = names;
        } catch (error) {
            console.error("Не удалось обновить никнеймы:", error);
        }
    }
}

export const currAuthStep = new SvelteAuthStep();