import { operStep } from "./OperationManager.svelte";
import type { Operation } from "../rustModels/Operation";
import type { OperationRaw } from "../rustModels/OperationRaw";
import type { Contract } from '$lib/models/rustModels/Contract';
import { OperationSvelte } from "./OperationSvelte.svelte";

export class StateProcessor {
	private _opersSvelte: OperationSvelte[] = [];
	private _opersRust: (Operation | null)[] = [];
    private _curInd = 0; 
    private _maxInd = 0;
    private _unProcceed = $state(0);
	private _curOper = $state<OperationSvelte | null>(null);


    constructor() {
        this._opersSvelte = [];
        this._opersRust = [];
        this._curInd = 0;
        this._maxInd = 0;
        this._unProcceed = 0;
		this._curOper = null;
    }

    async init(opers: OperationRaw[]) {
        this._curInd = 0;
        this._maxInd = opers.length - 1;
        this._unProcceed = opers.length;

        for (const operRaw of opers) {
			let operSvelte = new OperationSvelte;
            operSvelte.fromRaw(operRaw);
            this._opersSvelte.push(operSvelte);
            this._opersRust.push(null);
        }
		this._curOper = this._opersSvelte[0];

    }

	get curOper() {
		return this._curOper;
	}

	get unProceed() {
        return this._unProcceed
    }

	getContractInfo(contract: Contract) {
        const num = contract.contract_num;
        const d = contract.contract_date;
        return `Договор № ${num} от ${d}`;
    }



	makeRust() {
		if (!this._curOper?.isValid) {
			return;
		}
		if (!this._curOper?.isDuplicate) {
			const operRust = this._curOper?.makeRust();
			if (operRust) {
				this._opersRust[this._curInd] = operRust;
				this._unProcceed -= 1;
			} 
		} else {
			this._unProcceed -= 1;
		}
		this.next();
	}

    next() {
        if (this._unProcceed == 0) {
			this._curOper = null;
			return
		}
		while (this._opersRust[this._curInd] != null || 
			(
				this._opersSvelte[this._curInd].isDuplicate &&
				this._opersSvelte[this._curInd].isConfirmed
			)
		) {
			this._curInd = (this._curInd + 1) % this._maxInd;
		}
		this._curOper = this._opersSvelte[this._curInd];
    }

	prev() {
        if (this._unProcceed == 0) {
			this._curOper = null;
			return
		}
		while (this._opersRust != null || 
			(
				this._opersSvelte[this._curInd].isDuplicate &&
				this._opersSvelte[this._curInd].isConfirmed
			)
		) {
			this._curInd = (this._curInd - 1) % this._maxInd;
		}
		this._curOper = this._opersSvelte[this._curInd];
    }

}
