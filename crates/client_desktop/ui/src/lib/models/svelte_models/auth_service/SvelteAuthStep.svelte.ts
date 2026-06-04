import type { AuthStep } from '$lib/models/AuthStep';
import type { NickData } from '$lib/models/NickData';
import {FieldValidator} from "$lib/service/validate/FieldValidator.svelte";

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
        this.steps.length = this.index + 1;
        this.steps.push(next_step);
        this.index++;
        this.step = next_step;
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
}

export const currAuthStep = new SvelteAuthStep();