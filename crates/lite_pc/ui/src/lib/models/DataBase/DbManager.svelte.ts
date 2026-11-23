import {invoke} from '@tauri-apps/api/core';

import type {DatabaseStep} from '$lib/models/rustModels/DatabaseStep';
import {DbStepValues} from '$lib/models/DataBase/DbValues';

import DbLoading from '$lib/service/DataBase/DbLoading.svelte';
import DbTryLater from '$lib/service/DataBase/DbTryLater.svelte';

class DbManager {
	private _step = $state<DatabaseStep>({
		Loading: {text: 'Выберите функционал для работы с базой данных'}
	});

	get step() {return this._step};
	set step(nextStep: DatabaseStep) {
		this._step = nextStep;
	};

	get currentText(): string {
		const key = Object.keys(this._step)[0];
		return (this._step as any)[key]?.text ?? '';
	}

	set currentText(value: string) {
		const key = Object.keys(this._step)[0];
		if (!key) return;

		this._step = { 
			...this._step, 
			[key]: { 
				...(this._step as any)[key],
				text: value 
			} 
		};
	}

	get getPage() {
        const step = this.step;
        if (DbStepValues.Loading in step) {
            return DbLoading
        } else if (DbStepValues.TryLater in step) {
            return DbTryLater
        } else {
            return null
        }
    }

	reload() {
		this.step = { Loading: {text: 'Выберите функционал для работы с базой данных'}};
	}
}

export const dbStep = new DbManager;