import {OperationType} from '$lib/models/Operation/OperationValues';
import type {OperationStep} from '$lib/models/rustModels/OperationStep';
import type { OperationInfo } from '../rustModels/OperationInfo';


import AccInput from '$lib/service/operation/AccInput.svelte';
import Loading from '$lib/service/operation/Loading.svelte';
import ManualInput from '$lib/service/operation/ManualInput.svelte';
import ProcessSuccess from '$lib/service/operation/ProcessSuccess.svelte';
import StatementLoader from '$lib/service/operation/StatementLoader.svelte';
import StatementSuccess from '$lib/service/operation/StatementSuccess.svelte';
import TryLater from '$lib/service/operation/TryLater.svelte';



class OperationManager {
    private _step = $state<OperationStep>({
        Loading: {text: "Выберите функционал работы с проводками"}
    });

	get step() {
		return this._step;
	}

	set step(next_step: OperationStep) {
		this._step = next_step;
	}

	get currentText(): string {
		const key = Object.keys(this._step)[0];
		return (this._step as any)[key]?.text ?? '';
	}

	set currentText(value: string) {
		const key = Object.keys(this._step)[0];
		if (!key) return;

		// Перезаписываем объект целиком, чтобы Svelte 5 увидел реактивность
		this._step = { 
			...this._step, 
			[key]: { 
				...(this._step as any)[key],
				text: value 
			} 
		};
	}

    get getPage() {

        if (OperationType.AccInput in this._step) {
            return AccInput
        } else if (OperationType.Loading in this._step) {
            return Loading
        } else if (OperationType.ManualInput in this._step) {
            return ManualInput
		} else if (OperationType.ProcessSuccess in this._step) {
			return ProcessSuccess
        } else if (OperationType.StatementLoader in this._step) {
            return StatementLoader
        } else if (OperationType.StatementSuccess in this._step) {
			return StatementSuccess
		} else {
            return TryLater
        }
    }

	reload() {
		console.error('[operStep reload] before');
		this.step = {Loading: {text: "Выберите функционал работы с проводками"}};
		console.error('[operStep reload] after');
	}
}

export const operStep = new OperationManager;