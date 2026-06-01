// src/lib/service/validate/FieldValidator.svelte.ts
import { invoke } from "@tauri-apps/api/core";
import type { SvelteValidator } from "$lib/models/SvelteValidator";

export class FieldValidator {
    #isValid = $state(false);

    get isValid(): boolean {
        return this.#isValid;
    }

    async validate(payload: SvelteValidator) {
        const value = Object.values(payload)[0] as string;

        if (value.trim() === '') {
            this.#isValid = false;
            return;
        }

        try {

            this.#isValid = await invoke<boolean>('cmd_validate_field', {
                    typeValue: payload // В Rust превратится в аргумент type_value
                });
        } catch (err) {
            console.error(`[FieldValidator] Ошибка при вызове cmd_validate_field:`, err);
            this.#isValid = false;
        }
    }

    reset() {
        this.#isValid = false;
    }
}
