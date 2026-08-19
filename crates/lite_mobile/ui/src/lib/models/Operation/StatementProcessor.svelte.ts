import { operStep } from "./OperationManager.svelte";
import type { Operation } from "../rustModels/Operation";
import type { OperationRaw } from "../rustModels/OperationRaw";
import type { Contract } from '$lib/models/rustModels/Contract';
import { OperationSvelte } from "./OperationSvelte.svelte";

export class StateProcessor {
    opersRaw = $state<OperationRaw[]>([]);
    opersSvelte = $state<OperationSvelte[]>([]);
    opersRust = $state<(Operation | null)[]>([]);
    curInd = $state(0); // делаем индексы реактивными, чтобы Svelte обновлял экран при переключении
    maxInd = $state(0);


    constructor() {
        this.opersRaw = [];
        this.opersSvelte = [];
        this.opersRust = [];
        this.curInd = 0;
        this.maxInd = 0;
    }


    async init(opers: OperationRaw[]) {
        
        this.opersRaw = opers;
        this.curInd = 0;
        this.maxInd = opers.length - 1;

        for (const operRaw of opers) {
            const operSvelte = await OperationSvelte.fromRaw(operRaw);
            this.opersSvelte.push(operSvelte);
        }

    }

    next() {
        const total = this.opersSvelte.length;
        if (total === 0) return;
        this.curInd = (this.curInd + 1) % total;
    }

    prev() {
        const total = this.opersSvelte.length;
        if (total === 0) return;
        // Безопасный переход назад для циклического массива в JS
        this.curInd = (this.curInd - 1 + total) % total;
    }

    getContractInfo(contract: Contract) {
        const num = contract.contract_num;
        const d = contract.contract_date;
        return `Договор № ${num} от ${d}`;
    }
}
