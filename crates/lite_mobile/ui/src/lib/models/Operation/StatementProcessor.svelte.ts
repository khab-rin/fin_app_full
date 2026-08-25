import type { Integ } from "../rustModels/Integ";
import type { Operation } from "../rustModels/Operation";
import type { OperationRaw } from "../rustModels/OperationRaw";
import { OperationSvelte } from "./OperationSvelte.svelte";

export class StateProcessor {
	private _svelteOperations: OperationSvelte[] = [];
	private _rustOperations: (Operation | null)[] = [];
	private _curIndex: number | null = null;
	private _totalOperations: number = 0;
	private _unProcceed = $state<number>(0);
	get unProcceed() {return this._unProcceed;}
	private _curOperation = $state<OperationSvelte | null>(null);
	get curOper() {return this._curOperation;}


	constructor() {
		this._svelteOperations = [];
		this._rustOperations = [];
		this._curIndex = null;
		this._totalOperations = 0;
		this._unProcceed = 0;
		this._curOperation = null;
	}

	async init(opers: OperationRaw[]) {
		if (opers.length == 0) {
			return;
		}
		this._curIndex = 0;
		this._totalOperations = opers.length;
		this._unProcceed = opers.length;

		const validePromises = [];

		for (const operRaw of opers) {
			this._rustOperations.push(null);
			const operSvelte: OperationSvelte = new OperationSvelte;
			this._svelteOperations.push(operSvelte);
			validePromises.push(operSvelte.fromRaw(operRaw));
		}
		await Promise.all(validePromises);
		this._curOperation = this._svelteOperations[0];
	}

	makeRust() {
		if (
			this.curOper == null || 
			this._curIndex == null ||
			this.curOper.isValid ||
			this.unProcceed == 0

		) { 
			return;
		}

		const operation: Operation | null = this.curOper.makeRust();
		
		this._rustOperations[this._curIndex] = operation;
		this._unProcceed -= 1;
		this.next();
	}

	next() {
		if (
			this._unProcceed == 0 ||
			this._curIndex == null ||
			this._curOperation == null
		) {
			this._curOperation = null;
			this._curIndex = null;
			return;
		}

		this._curIndex = (this._curIndex + 1) % this._totalOperations;

		while (this._svelteOperations[this._curIndex].isConfirmed) {
			this._curIndex = (this._curIndex + 1) % this._totalOperations;
		}

		this._curOperation = this._svelteOperations[this._curIndex];

	}

	prev() {
		if (
			this._unProcceed == 0 ||
			this._curIndex == null ||
			this._curOperation == null
		) {
			this._curOperation = null;
			this._curIndex = null;
			return;
		}

		this._curIndex = (this._curIndex - 1 + this._totalOperations) % this._totalOperations;

		while (this._svelteOperations[this._curIndex].isConfirmed) {
			this._curIndex = (this._curIndex - 1 + this._totalOperations) % this._totalOperations;
		}
		this._curOperation = this._svelteOperations[this._curIndex];
	}
}

