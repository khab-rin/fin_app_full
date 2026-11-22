import {invoke} from '@tauri-apps/api/core';
import type {NullRegistry} from '$lib/models/rustModels/NullRegistry';

class NullStore {
	private _value = $state<NullRegistry | null>(null);

	async init() {
		if (!this._value) {
			this._value = await invoke<NullRegistry>(
				"cmd_get_nulls"
			)
		}
	}
	get value() {
		return this._value;
	}
}

export const nulls = new NullStore();