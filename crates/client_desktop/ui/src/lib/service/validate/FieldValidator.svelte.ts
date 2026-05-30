// src/lib/service/validate/FieldValidator.svelte.ts
import { invoke } from "@tauri-apps/api/core";
import type { SvelteValidator } from "$lib/models/Validator";

export class FieldValidator {
    #isValid = $state(true);

    get isValid(): boolean {
        return this.#isValid;
    }

    async validate(payload: SvelteValidator) {
        const value = Object.values(payload)[0] as string;

        // Если пользователь полностью очистил поле, мгновенно снимаем красную подсветку
        if (value.trim() === '') {
            this.#isValid = true;
            return;
        }

        try {
            const result = await invoke<boolean>('cmd_validate_field', {
                typeValue: payload // В Rust превратится в аргумент type_value
            });

            this.#isValid = result;
        } catch (err) {
            console.error(`[FieldValidator] Ошибка при вызове cmd_validate_field:`, err);
            this.#isValid = false;
        }
    }

    reset() {
        this.#isValid = true;
    }
}
