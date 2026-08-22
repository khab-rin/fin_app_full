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
			!this.curOper.isValid ||
			this.unProcceed == 0

		) { 
			console.error("curOper = ", this.curOper);
			console.error("_curIndex = ", this._curIndex);
			console.error("unProcceed = ", this.unProcceed);
			console.error("isValid ", this.curOper?.isValid);
			return;
		}


		const operation: Operation | null = this.curOper.makeRust();
		console.error("oper = ", operation);
		this._rustOperations[this._curIndex] = operation;
		this._unProcceed -= 1;
		this.curOper.isConfirmed = true;
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


		while (this._svelteOperations[this._curIndex].isConfirmed == true) {
			this._curIndex = (this._curIndex + 1) % this._totalOperations;
		}

		this._curOperation = this._svelteOperations[this._curIndex];

	}
}




// import { operStep } from "./OperationManager.svelte";
// import type { Operation } from "../rustModels/Operation";
// import type { OperationRaw } from "../rustModels/OperationRaw";
// import type { Contract } from '$lib/models/rustModels/Contract';
// import { OperationSvelte } from "./OperationSvelte.svelte";

// export class StateProcessor {
// 	private _opersSvelte: OperationSvelte[] = [];
// 	private _opersRust: (Operation | null)[] = [];
//     private _curInd = 0; 
//     private _maxInd = 0;
//     private _unProcceed = $state(0);
// 	private _curOper = $state<OperationSvelte | null>(null);


//     constructor() {
//         this._opersSvelte = [];
//         this._opersRust = [];
//         this._curInd = 0;
//         this._maxInd = 0;
//         this._unProcceed = 0;
// 		this._curOper = null;
//     }

//     async init(opers: OperationRaw[]) {
//         this._curInd = 0;
//         this._maxInd = opers.length - 1;
//         this._unProcceed = opers.length;

//         for (const operRaw of opers) {
// 			let operSvelte = new OperationSvelte;
//             operSvelte.fromRaw(operRaw);
//             this._opersSvelte.push(operSvelte);
//             this._opersRust.push(null);
//         }
// 		this._curOper = this._opersSvelte[0];

//     }

// 	get curOper() {
// 		return this._curOper;
// 	}

// 	get unProceed() {
//         return this._unProcceed
//     }

// 	getContractInfo(contract: Contract) {
//         const num = contract.contract_num;
//         const d = contract.contract_date;
//         return `Договор № ${num} от ${d}`;
//     }



// 	makeRust() {
// 		if (!this._curOper?.isValid) {
// 			console.error("isValid = false");
// 			return;
// 		}
// 		if (!this._curOper?.isDuplicate) {
// 			const operRust = this._curOper?.makeRust();
// 			console.error("oper = ", operRust);
// 			if (operRust) {
// 				this._opersRust[this._curInd] = operRust;
// 				this._unProcceed -= 1;
// 			} 
// 		} else {
// 			console.error("oper = duplicate",);
// 			this._unProcceed -= 1;
// 		}
// 		this.next();
// 	}

//     next() {
//         if (this._unProcceed == 0) {
// 			this._curOper = null;
// 			return
// 		}
// 		while (this._opersRust[this._curInd] != null || 
// 			(
// 				this._opersSvelte[this._curInd].isDuplicate &&
// 				this._opersSvelte[this._curInd].isConfirmed
// 			)
// 		) {
// 			this._curInd = (this._curInd + 1) % this._maxInd;
// 		}
// 		this._curOper = this._opersSvelte[this._curInd];
//     }

// 	prev() {
//         if (this._unProcceed == 0) {
// 			this._curOper = null;
// 			return
// 		}
// 		while (this._opersRust[this._curInd] != null || 
// 			(
// 				this._opersSvelte[this._curInd].isDuplicate &&
// 				this._opersSvelte[this._curInd].isConfirmed
// 			)
// 		) {
// 			this._curInd = (this._curInd - 1) % this._maxInd;
// 		}
// 		this._curOper = this._opersSvelte[this._curInd];
//     }

// }
