import type {MchdStep} from "$lib/models/rustModels/MchdStep";
import { FieldValidator } from "../Auth/FieldValidator.svelte";
import { MchdStepType } from "./MchdValues";

import HomeMchdFull from "$lib/service/mchd/HomeMchdFull.svelte";
import HomeMchdMiss from "$lib/service/mchd/HomeMchdFull.svelte";
import TaxMchdMiss from "$lib/service/mchd/TaxMchdMiss.svelte";
import TaxMchdFull from "$lib/service/mchd/TaxMchdFull.svelte";
import Loading from "$lib/service/mchd/Loading.svelte";
import TryLater from "$lib/service/mchd/TryLater.svelte";

class MchdManager {
    step = $state<MchdStep>({
        Loading: {text: "Страница загружается, подождите..."}
    });
    private steps: MchdStep[] = $state([]);
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

    add(next_step: MchdStep) {
        this.steps.length = this.index + 1;
        this.steps.push(next_step);
        this.index++;
        this.step = next_step;
    }

    data = $state({
        userSurName: new FieldValidator("String1_50"),
        userFirstName: new FieldValidator("String1_50"),
        userMidName: new FieldValidator("MidName"),
    })

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

    get getPage() {
        const step = this.step;
        if (MchdStepType.Loading in step) {
            return Loading
        } else if (MchdStepType.TryLater in step) {
            return TryLater
        } else if (MchdStepType.TaxMchdMiss in step) {
            return TaxMchdMiss
        } else if (MchdStepType.TaxMchdFull in step) {
            return TaxMchdFull
        } else if (MchdStepType.HomeMchdMiss in step) {
            return HomeMchdMiss
        } else if (MchdStepType.HomeMchdFull in step) {
            return HomeMchdFull
        } else {
            return null
        }
    }

}

export const currentMchdStep = new MchdManager;