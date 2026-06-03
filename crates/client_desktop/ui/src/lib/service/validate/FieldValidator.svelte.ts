import { invoke } from "@tauri-apps/api/core";
import type { SvelteValidator } from "$lib/models/SvelteValidator";

export class FieldValidator {
    #isValid = $state(false);
    
    value = $state(""); 
    
    private type_value: SvelteValidator;

    constructor(type_value: SvelteValidator) {
        this.type_value = type_value;
    }

    async validate() { // Добавлена фигурная скобка
        try {
            // Добавлена запятая после имени команды
            this.#isValid = await invoke<boolean>("cmd_validate_field", {
                type_value: this.type_value,
                value: this.value
            });
        } catch (err) {
            this.#isValid = false;
            console.error("COMMAND cmd_validate_field FAILED, data = ", this.type_value, this.value, err);
        }
    }

    get isValid(): boolean {
        return this.#isValid;
    }

    reset() {
        this.#isValid = false;
        this.value = "";
    }
}