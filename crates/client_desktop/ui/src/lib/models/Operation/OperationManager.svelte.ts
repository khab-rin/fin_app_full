import {OperationType} from '$lib/models/Operation/OperationValues';
import type {OperationStep} from '$lib/models/rustModels/OperationStep';
import type { Operation } from "$lib/models/rustModels/Operation";

import AccessDenied from '$lib/service/operation/AccessDenied.svelte';
import Loading from '$lib/service/operation/Loading.svelte';
import SuccessRaw from '$lib/service/operation/SuccessRaw.svelte';
import Success from '$lib/service/operation/Success.svelte';
import TryLater from '$lib/service/operation/TryLater.svelte';



class OperationManager {
    step = $state<OperationStep>({
        Loading: {text: "Выберите способ создания проводок"}
    });
    private steps: OperationStep[] = $state([]);
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

    add(next_step: OperationStep) {
        this.steps.length = this.index + 1;
        this.steps.push(next_step);
        this.index++;
        this.step = next_step;
    }

    private operations: Operation[] = $state([]);

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
        if (OperationType.AccessDenied in step) {
            return AccessDenied
        } else if (OperationType.Loading in step) {
            return Loading
        } else if (OperationType.SuccessRaw in step) {
            return SuccessRaw
        } else if (OperationType.Success in step) {
            return Success
        } else if (OperationType.TryLater in step) {
            return TryLater
        } else {
            return null
        }
    }


}

export const operationStep = new OperationManager;