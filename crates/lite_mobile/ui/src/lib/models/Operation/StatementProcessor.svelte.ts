import { operStep } from "./OperationManager.svelte";

import type { Operation } from "../rustModels/Operation";
import type { OperationRaw } from "../rustModels/OperationRaw";
import { OperationSvelte } from "./OperationSvelte.svelte";

export class StateProcessor {
    opersRaw = $state<OperationRaw[]>([]);
    opersSvelte = $state<OperationSvelte[]>([]);
    opersRust = $state<(Operation | null)[]>([]);
    curInd = 0;
    maxInd = 0;


    constructor(opers: OperationRaw[]) {
        for (const operRaw of opers) {
            const operSvelte = new OperationSvelte(operRaw);
            const operRust = operSvelte.makeRust();
            this.opersSvelte.push(operSvelte);
            this.opersRust.push(operRust);

        }
        this.opersRaw = opers;
        this.curInd = 0;
        this.maxInd = this.opersSvelte.length - 1;
    }

    next() {
        this.curInd = (this.curInd + 1) % this.maxInd;
    }

    prev() {
        this.curInd = (this.curInd - 1) % this.maxInd;
    }
}