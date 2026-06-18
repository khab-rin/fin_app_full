import { invoke } from "@tauri-apps/api/core";
import type { SvelteValidator } from "$lib/models/rustModels/SvelteValidator";

export class FieldValidator {
    _isValid = $state(true);
    
    private _value = $state(""); 
    
    private typeValue: SvelteValidator;

    constructor(type_value: SvelteValidator, start_value: string) {
        this._value = start_value;
        this.typeValue = type_value;
    }

    get value(): string {
        return this._value;
    }

    set value(newValue: string) {
        this._value = newValue;
        this.validate();
    }

    async validate() {
        try {
            this._isValid = await invoke<boolean>("cmd_validate_field", {
                typeValue: this.typeValue,
                value: this._value
            });
        } catch (err) {
            this._isValid = false;
            console.error("COMMAND cmd_validate_field FAILED, data = ", this.typeValue, this._value, err);
        }
    }

    get isValid(): boolean {
        return this._isValid;
    }

    reset() {
        this._isValid = false;
        this._value = "";
    }
}