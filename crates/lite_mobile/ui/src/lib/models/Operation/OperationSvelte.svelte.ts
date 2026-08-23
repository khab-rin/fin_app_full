import { FieldValidator } from "../Auth/FieldValidator.svelte";

import type { DocType } from "../rustModels/DocType";
import type { Account } from "../rustModels/Account";
import type { Operation } from "../rustModels/Operation";
import type { OperationRaw } from "../rustModels/OperationRaw";
import { invoke } from "@tauri-apps/api/core";
import type { Company } from "../rustModels/Company";

export class OperationSvelte {
	
	data = $state({
		ctrPtyName: new FieldValidator('CompanyName', '');
		debet: new FieldValidator('Account', ''),
		credit: new FieldValidator('Account', ''),
		amount: new FieldValidator('RubF', ''),
		operDate: new FieldValidator('Date', ''),
		entrDate: new FieldValidator('Date', ''),
	});

	private _isValid = $derived(
		this.data.debet.isValid &&
		this.data.credit.isValid &&
		this.data.amount.isValid &&
		this.data.operDate.isValid &&
		this.data.entrDate.isValid
	)
	get isValid() {return this._isValid;}


	private _isDuplicate = $state(false);
	get isDuplicate() { return this._isDuplicate; }


	private _isConfirmed = $state(false);
	set isConfirmed(value: boolean) { this._isConfirmed = value;}
	get isConfirmed() {return this._isConfirmed;}

	private _isCompare = $state<boolean>(false);
	get isCompare() {return this._isCompare}

	private _debetStr = $state<string>('');
	get debetStr() {return this._debetStr;}

	private _creditStr = $state<string>('');
	get creditStr() {return this._creditStr};


	private _ctrPty: Company | null = null;


	async fromRaw(raw: OperationRaw) {
		this._ctrPty = raw.ctrpty;

		await Promise.all([
			this.data.ctrPtyName.async_set(raw.ctrpty?.metadata.comp_name?.short_egrul_name ?? ''),
			this.data.debet.async_set(raw.debet),
			this.data.credit.async_set(raw.credit),
			this.data.amount.async_set(raw.amount),
			this.data.operDate.async_set(raw.oper_date ?? ""),
			this.data.entrDate.async_set(raw.entr_date),
		]);
	}

	constructor() {
		$effect(() => {
			const leftAcc = this.data.debet.value;
			const rigthAcc = this.data.credit.value; 
			let isCurrent = true;

			const f1 = (result: boolean) => {if (isCurrent) this._isCompare = result;};
			const f2 = (result: string) => {if (isCurrent) this._debetStr = result};
			const f3 = (result: string) => {if (isCurrent) this._creditStr = result;};

			let pr = [];

			if (this.data.debet.isValid && this.data.credit.isValid) {
				pr.push(this.cmdCompareAccounts(leftAcc, rigthAcc, f1));
			} else {
				f1(false);
			}

			if (this.data.debet.isValid) {
				pr.push(this.cmdGetAccStr(leftAcc, f2));
			} else {
				f2('');
			}

			if (this.data.credit.isValid) {
				pr.push(this.cmdGetAccStr(rigthAcc, f3));
			} else {
				f3('')
			}

			Promise.all(pr);

			return () => { isCurrent = false;};
		});

	}

	async cmdCompareAccounts(
		leftAcc: string, 
		rigthAcc: string,
		callback: (res: boolean) => void
	) {
		try {
			let res: boolean = await invoke<boolean>(
				'cmd_is_accounts_compatible',
				{leftAcc:leftAcc, rigthAcc: rigthAcc}
			);
			callback(res);
		} catch(err) {
			console.error("cmd_is_accounts_compatible FAILED, err = ", err);
			callback(false);
		}
	}

	async cmdGetAccStr(account: string, callback: (res: string)=> void) {
		try {
			let accStr = await invoke<string>(
				"cmd_get_acc_info",
				{account: account}
			);
			callback(accStr);
		} catch(err) {
			console.error("cmd_get_acc_info FAILED, err = ", err);
			callback("");
		}
	}

	async cmdChangeCtrPty(compInn: string, kpp: string) {
		let data = {compInn: compInn, kpp: kpp}
		const newCompany = await invoke<Company|null> (
			'cmd_get_comp_by_inn_kpp', data
		);
		this._ctrPty = newCompany;

	}

	

	makeRust(): Operation | null {
		if (!this.isValid || this.isDuplicate ) {
			return null;
		} else {
			let operation: Operation = {
				oper_id: "this.data.operId.value",
				user_id: 'this.data.userId.value', 
				comp_id: 'this.data.compId.value', 
				ctrpty_id: 'this.data.ctrptyId.value', 
				contract_id: 'this.data.contractId.value', 
				debet: 'this.data.debet.value' as Account, 
				credit: 'this.data.credit.value'  as Account, 
				amount: this.data.amount.value, 
				oper_date: this.data.operDate.value, 
				doc_type: 'this.data.docType.value' as DocType,
				doc_num: 'this.data.docNum.value', 
				doc_date: 'this.data.docDate.value',
				is_storno: false,
				is_del: false, 
				entr_date: this.data.entrDate.value,
			}
			return operation;
		}
	}
}














// import { FieldValidator } from "../Auth/FieldValidator.svelte";
// import type { OperationRaw } from "../rustModels/OperationRaw";
// import type { Operation } from "../rustModels/Operation";
// import type { Account } from "../rustModels/Account";
// import type { DocType } from "../rustModels/DocType";
// import type {Company} from '$lib/models/rustModels/Company';
// import type { Contract } from "../rustModels/Contract";
// import type { OperationStep } from "../rustModels/OperationStep";
// import { invoke } from "@tauri-apps/api/core";
// import type { NewContrData } from "../rustModels/NewContrData";
// import type { BoxUuid } from "../rustModels/BoxUuid";

// export class OperationSvelte {

// 	private _isDuplicate = $state(false);
//     private _isAccountsCompatible = $state(false);
// 	private _isConfirmed = $state(false);
// 	private _cntrPty = $state<Company | null>(null);
//     private _allPossContracts: Contract[] = [];

// 	get isDuplicate() {
// 		return this._isDuplicate;
// 	}
    
// 	get isDuplicateStr() {
// 		if (this._isDuplicate) {
// 			return "Операция дубликат"
// 		} else {
// 			return "Новая операция"
// 		}
// 	}

// 	get isAccountsCompatible() {
// 		return this._isAccountsCompatible;
// 	}

// 	get allPossContracts() {
// 		return this._allPossContracts;
// 	}

// 	get isConfirmed() {
// 		return this._isConfirmed
// 	}

// 	get cntrPty() {
// 		return this._cntrPty;
// 	}


//     data = $state({
//         operId: new FieldValidator("BoxUuid", ""),
//         userId: new FieldValidator("BoxUuid", ""),

//         compId: new FieldValidator("BoxUuid", ""),
//         ctrptyId: new FieldValidator("BoxUuid", ""),
//         ctrptyName: new FieldValidator("CompanyName", ""),

//         contractId: new FieldValidator("BoxUuid", ""),
//         contractNum: new FieldValidator("DocNum", ""),
//         contractDate: new FieldValidator("Date", ""),

//         debet: new FieldValidator("Account", ""),
//         credit: new FieldValidator("Account", ""),
//         amount: new FieldValidator("RubF", ""),
//         operDate: new FieldValidator("Date", ""),

//         docType: new FieldValidator("DocType", ""),
//         docNum: new FieldValidator("DocNum", ""),
//         docDate: new FieldValidator("Date", ""),

//         isStorno: false,
//         isDel: false,

//         entrDate: new FieldValidator("Date", ""),
//     })

// 	isValid = $derived(
//         this.data.operId.isValid &&
//         this.data.userId.isValid &&

//         this.data.compId.isValid &&
//         this.data.ctrptyId.isValid &&

//         this.data.contractId.isValid &&

//         this.data.debet.isValid &&
//         this.data.credit.isValid &&
//         this.data.amount.isValid &&
//         this.data.operDate.isValid &&

//         this.data.docType.isValid &&
//         this.data.docNum.isValid &&
//         this.data.docDate.isValid &&

//         this.data.entrDate.isValid &&
// 		this._cntrPty != null
//     );

//     constructor() {
// 		$effect(() => {
// 			const left = this.data.debet.value;
// 			const rigth = this.data.credit.value;
// 			this.compateAccounts(left, rigth);
// 		})

// 		$effect(() => {
// 			[
// 				this.data.docNum.value, 
// 				this.data.operDate.value, 
// 				this.data.amount.value, 
// 				this._cntrPty?.comp_id
// 			];
// 			this.cmdIsOperationExist()
// 		});
// 	}

// 	fromRaw(raw: OperationRaw) {

//         this._allPossContracts = raw.contract.contracts;
//         this._cntrPty = raw.ctrpty;
        
//         this.data.operId.value = raw.oper_id;
//         this.data.userId.value = raw.user_id;
//         this.data.compId.value = raw.comp_id;

//         this.data.ctrptyId.value = raw.ctrpty?.comp_id ?? "";
//         this.data.ctrptyName.value = raw.ctrpty?.metadata?.comp_name?.short_egrul_name ?? "";

        
//         this.data.contractId.value = raw.contract.current?.contract_id ?? "";
//         this.data.contractNum.value = raw.contract.current?.contract_num ?? "";
//         this.data.contractDate.value = raw.contract.current?.contract_date ?? "";

//         this.data.debet.value = raw.debet;
//         this.data.credit.value = raw.credit;
//         this.data.amount.value = raw.amount;
//         this.data.operDate.value = raw.oper_date ?? "";

//         this.data.docType.value = raw.doc_type;
//         this.data.docNum.value = raw.doc_num;
//         this.data.docDate.value = raw.doc_date;

//         this.data.isStorno = raw.is_storno;
//         this.data.isDel = raw.is_del;
//         this.data.entrDate.value = raw.entr_date;
//     }

// 	async cmdIsOperationExist() {
// 		if (
// 			!this.data.docNum.isValid ||
// 			!this.data.operDate.isValid ||
// 			!this.data.amount.isValid ||
// 			this._cntrPty == null
// 		) {
// 			this._isDuplicate = false;
// 			this.data.operId.value = "";
// 			return;
// 		}

// 		const docNum = this.data.docNum.isValid ? this.data.docNum.value : null;
// 		const operDate = this.data.operDate.isValid ? this.data.operDate.value : null;
// 		const amount = this.data.amount.isValid ? this.data.amount.value : null;
// 		const ctrptyId = this._cntrPty?.comp_id ?? null;

// 		let data = {
// 			docNum: docNum,
// 			operDate: operDate,
// 			amount: amount,
// 			ctrptyId: ctrptyId
// 		};

// 		try {
// 			let [operId, flag] = await invoke<[BoxUuid, boolean]>(
// 			"cmd_is_operation_exist",
// 			data
// 			); 
// 			this.data.operId.value = operId;
// 			this._isDuplicate = flag;
// 		} catch(err) {
// 			this.data.operId.value = "";
// 			this._isDuplicate = false;
// 		}
		
// 	}

// 	async compateAccounts(leftAcc: string, rigthAcc: string) {
// 		if (!this.data.debet.isValid || !this.data.credit.isValid) {
// 			this._isAccountsCompatible = false;
// 			return;
// 		}

//         try {
//             this._isAccountsCompatible = await invoke<boolean>(
//                 "cmd_is_accounts_compatible",
//                 {leftAcc:leftAcc, rigthAcc: rigthAcc}
//             )
//         } catch(err) {
//             console.error("cmd_is_accounts_compatible FAILED, err = ", err);
//             this._isAccountsCompatible = false;
//         }
//     }


//     async refreshCtrpty(compInn: string, kpp: string) {
//         let data = {compInn: compInn, kpp: kpp};
//         const newCompany: Company | null = await invoke<Company>(
//             "cmd_get_comp_by_inn_kpp", 
//             data
//         );
// 		this._cntrPty = newCompany;
//         this.data.ctrptyId.value = newCompany?.comp_id ?? "";
//         this.data.ctrptyName.value = newCompany?.metadata.comp_name?.short_egrul_name ?? "";
		
// 		if (newCompany) {
// 			const contracts = await invoke<Contract[]>(
// 				"cmd_get_contracts_by_ctrpty_id",
// 				{ctrptyId: newCompany?.comp_id ?? ""}
// 			);
// 			this._allPossContracts = contracts;
// 		}
        
//         this.data.contractId.value = "";
//         this.data.contractDate.value = "";
//         this.data.contractNum.value = "";

//     }

//     async refreshContracts(data: NewContrData) {
// 		if (data.ctrpty_id == "") {
// 			return
// 		}
//         const freshContracts: Contract[] = await invoke<Contract[]>(
//             "cmd_add_new_contract",
//             {data: data}
//         );
//         this._allPossContracts = freshContracts;
//         this.data.contractId.value = "";
//         this.data.contractDate.value = "";
//         this.data.contractNum.value = "";
//     }

//     refreshContract(contract: Contract) {
//         this.data.contractId.value = contract.contract_id;
//         this.data.contractDate.value = contract.contract_date;
//         this.data.contractNum.value = contract.contract_num;
//     }

//     get contractStr(): string {
//         const num = this.data.contractNum.value;
//         const d = this.data.contractDate.value;
//         const id = this.data.contractId.value;
//         if (!num || !d || !id) {return "без договора"}
//         return `Договор № ${num} от ${d}`;
//     }

//     makeRust(): Operation | null {
//         if (!this.isValid) {
//             return null;
//         }
//         return { 
//             oper_id: this.data.operId.value,
//             user_id: this.data.userId.value, 
//             comp_id: this.data.compId.value, 
//             ctrpty_id: this.data.ctrptyId.value, 
//             contract_id: this.data.contractId.value, 
//             debet: this.data.debet.value as Account, 
//             credit: this.data.credit.value  as Account, 
//             amount: this.data.amount.value, 
//             oper_date: this.data.operDate.value, 
//             doc_type: this.data.docType.value as DocType,
//             doc_num: this.data.docNum.value, 
//             doc_date: this.data.docDate.value, 
//             is_storno: this.data.isStorno, 
//             is_del: this.data.isDel, 
//             entr_date: this.data.entrDate.value, 
//         };
//     }
// }