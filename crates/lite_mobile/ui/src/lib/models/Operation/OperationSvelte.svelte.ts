import { FieldValidator } from "../Auth/FieldValidator.svelte";

import type { DocType } from "../rustModels/DocType";
import type { Account } from "../rustModels/Account";
import type { Operation } from "../rustModels/Operation";
import type { OperationRaw } from "../rustModels/OperationRaw";
import { invoke } from "@tauri-apps/api/core";
import type { Company } from "../rustModels/Company";
import type { Contract } from "../rustModels/Contract";
import type { Currency } from "../rustModels/Currency";
import type { NewContrData } from "../rustModels/NewContrData";
import type { BoxUuid } from "../rustModels/BoxUuid";
import type {Date} from '$lib/models/rustModels/Date';

export class OperationSvelte {
	
	data = $state({
		operId: new FieldValidator('BoxUuid', ''),
		userId: new FieldValidator('BoxUuid', ''),
		compId: new FieldValidator('BoxUuid', ''),

		ctrptyId: new FieldValidator('BoxUuid', ''),

		debet: new FieldValidator('Account', ''),
		credit: new FieldValidator('Account', ''),
		amount: new FieldValidator('RubF', ''),

		operDate: new FieldValidator('Date', ''),

		docType: new FieldValidator('DocType', ''),
		docNum: new FieldValidator('DocNum', ''), 
		docDate: new FieldValidator('Date', ''),
		
		entrDate: new FieldValidator('Date', ''),
	});

	newContrData = {
		ctrPtyId: new FieldValidator('BoxUuid', ''),
		contractNum: new FieldValidator('DocNum', '125'),
		contractDate: new FieldValidator('Date', '18.08.2026'),
		contractTitle: new FieldValidator('String', 'договор'),
		contractStDate: new FieldValidator('Date', '18.08.2025'),
		contractEndDate: new FieldValidator('Date', '31.12.2030'),
		contractCurrency: new FieldValidator('Currency', 'руб'),
		contractTotAmnt: new FieldValidator('RubF', '1000000'),
		contractDefDays: new FieldValidator('Integ', '15'),
		contractDescr: new FieldValidator('String', 'Охуенный договор'),
	};

	private _ctrPty = $state<Company | null>(null);
	get ctrPty() {return this._ctrPty;}


	private _isNewContractValid = $derived(
		!this.newContrData.ctrPtyId.isValid ||
		!this.newContrData.contractNum.isValid ||
		!this.newContrData.contractDate.isValid ||
		!this.newContrData.contractTitle.isValid ||
		!this.newContrData.contractStDate.isValid ||
		!this.newContrData.contractEndDate.isValid ||
		!this.newContrData.contractCurrency.isValid ||
		!this.newContrData.contractTotAmnt.isValid ||
		!this.newContrData.contractDefDays.isValid ||
		!this.newContrData.contractDescr.isValid 
	)

	get isNewContractValid() {
		return this._isNewContractValid;
	}

	private _isValid = $derived(
		!this.data.operId.isValid ||
		!this.data.userId.isValid ||
		!this.data.compId.isValid ||
		!this.data.ctrptyId.isValid ||
		!this.data.debet.isValid ||
		!this.data.credit.isValid ||
		!this.data.amount.isValid ||
		!this.data.operDate.isValid ||
		!this.data.docType.isValid ||
		!this.data.docNum.isValid ||
		!this.data.docDate.isValid ||
		!this.data.entrDate.isValid
	)

	get isValid() {return this._isValid;}

	private _isDuplicate = $state(false);

	get isDuplicate() { return this._isDuplicate; }
	get isDuplicateStr() {
		if (this.isDuplicate) {
			return "Дубликат";
		} else {
			return "Новая операция";
		}
	}


	private _isConfirmed = $state(false);
	set isConfirmed(value: boolean) { this._isConfirmed = value;}
	get isConfirmed() {return this._isConfirmed;}

	private _isCompare = $state<boolean>(false);
	get isCompare() {return this._isCompare}

	private _debetStr = $state<string>('');
	get debetStr() {return this._debetStr;}

	private _creditStr = $state<string>('');
	get creditStr() {return this._creditStr};


	private _allPossContracts = $state<Contract []>([]);
	get allPossContracts() {
		return this._allPossContracts;
	}
	private _currContract = $state<Contract | null>(null);

	private _contrStr = $derived.by(() => {
		if (this._currContract == null) {
			return '';
		} else {
			return `Договор № ${this._currContract.contract_num} от ${this._currContract.contract_date}`
		}
	});
	get contrStr() {return this._contrStr;}

	anyContractStr(contract: Contract) {
		return `Договор № ${contract.contract_num} от ${contract.contract_date}`
	}

	async changeContract(contract: Contract) {
		this._currContract = contract;
	}

	async cmdAddNewContract() {
		if (this.isNewContractValid) {
			return;
		}
		const data: NewContrData = {
            ctrpty_id: this.newContrData.ctrPtyId.value,
            contract_num: this.newContrData.contractNum.value,
            contract_date: this.newContrData.contractDate.value,
            contract_title: this.newContrData.contractTitle.value,
            contract_st_date: this.newContrData.contractStDate.value,
            contract_end_date: this.newContrData.contractEndDate.value,
            contract_currency: this.newContrData.contractCurrency.value as Currency,
            contract_tot_amnt: this.newContrData.contractTotAmnt.value,
            contract_def_days: this.newContrData.contractDefDays.value,
            contract_descr: this.newContrData.contractDescr.value
        }

		const contracts = await invoke<Contract[]>(
			'cmd_add_new_contract',
			{data: data}
		);
		this._allPossContracts = contracts;
		this._currContract = null;
	}

	private _allCtrPtys = $state<Company[]>([]);
	get allCtrPtys() {return this._allCtrPtys};
	async cmdGetAllCompanys() {
		let companys: Company[] = await invoke(
			'cmd_get_all_companys',
			{}
		);
		this._allCtrPtys = companys;
	}

	async fromRaw(raw: OperationRaw) {
		this._ctrPty = raw.ctrpty;
		this._allPossContracts = raw.contract.contracts ?? [];
		this._currContract = raw.contract.current;

		await Promise.all([
			this.data.operId.asyncSet(raw.oper_id),
			this.data.userId.asyncSet(raw.user_id),
			this.data.compId.asyncSet(raw.comp_id),

			this.data.ctrptyId.asyncSet(raw.ctrpty?.comp_id ?? ''),

			this.data.debet.asyncSet(raw.debet),
			this.data.credit.asyncSet(raw.credit),
			this.data.amount.asyncSet(raw.amount),

			this.data.operDate.asyncSet(raw.oper_date ?? ''),

			this.data.docType.asyncSet(raw.doc_type),
			this.data.docNum.asyncSet(raw.doc_num),
			this.data.docDate.asyncSet(raw.doc_date),

			this.data.entrDate.asyncSet(raw.entr_date),
			
			this.newContrData.ctrPtyId.asyncSet(raw.ctrpty?.comp_id ?? '')
		]);
	}

	async cmdCheckDuplicate(
		docNum:string,
		operDate: string,
		amount: string,
		ctrptyId: string,
		callback: (res: [string, boolean]) => void
	 ) {
		try {
			let res:[string, boolean] = await invoke<[string, boolean]>(
				'cmd_is_operation_exist',
				{docNum, operDate, amount, ctrptyId}
			);
			callback(res);
		} catch(err) {
			console.error("cmdNewOperId FAILED, err = ", err);
			callback(["", false]);
		}
	 }


	constructor() {

		$effect(() => {
			let isCurrent = true;
			let f = ((result:[BoxUuid, boolean]) => {
				if (isCurrent) {
					this.data.operId.value = result[0];
					this._isDuplicate = result[1];
				}
			});

			if (!this.data.docNum.isValid ||
				!this.data.operDate.isValid ||
				!this.data.amount.isValid ||
				this._ctrPty == null
			) {
				f(['', false]);
				return;
			}

			
			let docNum = this.data.docNum.value;
			let operDate = this.data.operDate.value;
			let amount = this.data.amount.value;
			let ctrptyId = this._ctrPty.comp_id;

			(async () => {
				await this.cmdCheckDuplicate(docNum, operDate, amount, ctrptyId, f);
			})();

			return () => {isCurrent = false;};
		})


		$effect(() => {
			const leftAcc = this.data.debet.value;
			const rigthAcc = this.data.credit.value; 
			let isCurrent = true;
			
			(async() => {
				await Promise.all([
					this.data.debet.validate(),
					this.data.credit.validate()
				]);
				if (!isCurrent) {
				return;
			}

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
			})();

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
		
		await this.selectCtrPty(newCompany);
	}

	async selectCtrPty(company: Company | null) {
		this._ctrPty = company;
		this.data.ctrptyId.asyncSet(company?.comp_id ?? "");
		if (company) {
			const contracts = await invoke<Contract[]>(
				'cmd_get_contracts_by_ctrpty_id',
				{ctrptyId: company.comp_id}
			);
			if (!this._allCtrPtys.some(c => c.comp_id === company.comp_id)) {
				this._allCtrPtys.push(company)
			}
			this._allPossContracts = contracts;
		} else {
			this._allPossContracts = [];
		}
		await this.data.ctrptyId.asyncSet(company?.comp_id ?? '');
		await this.newContrData.ctrPtyId.asyncSet(company?.comp_id ?? '');
		this._currContract = null;
	}

	async cmdGetUserCompId() {
		let [userId, compId] = await invoke<[BoxUuid, BoxUuid]>(
		'cmd_get_user_comp_ids',
			{}
		);
		this.data.userId.value = userId;
		this.data.compId.value = compId;
	}

	async cmdGetToday() {
		let today = await invoke<Date>(
			'cmd_get_today',
			{}
		);
		this.data.entrDate.value = today;
	}

	
	makeRust(): Operation | null {
		if (this.isValid) {
			return null
		}
		if (this.isDuplicate) {
			this._isConfirmed = true;
			return null;
		} else {
			let operation: Operation = {
				oper_id: this.data.operId.value,
				user_id: this.data.userId.value,
				comp_id: this.data.compId.value, 

				ctrpty_id: this.data.ctrptyId.value, 

				contract_id: this._currContract?.contract_id ?? "",

				debet: this.data.debet.value as Account, 
				credit: this.data.credit.value  as Account, 
				amount: this.data.amount.value, 

				oper_date: this.data.operDate.value, 

				doc_type: this.data.docType.value as DocType,
				doc_num: this.data.docNum.value, 
				doc_date: this.data.docDate.value,

				is_storno: false,
				is_del: false, 

				entr_date: this.data.entrDate.value,
			}
			this._isConfirmed = true;
			return operation;
		}
	}

	async reset() {
		this._ctrPty = null;
		this._allPossContracts = [];
		this._currContract = null;

		await Promise.all([
			this.data.operId.asyncSet(''),

			this.data.ctrptyId.asyncSet(''),

			this.data.debet.asyncSet(''),
			this.data.credit.asyncSet(''),
			this.data.amount.asyncSet(''),

			this.data.operDate.asyncSet(''),

			this.data.docType.asyncSet(''),
			this.data.docNum.asyncSet(''),
			this.data.docDate.asyncSet(''),
			
			this.newContrData.ctrPtyId.asyncSet(''),
			this.newContrData.ctrPtyId.asyncSet(''),
			this.newContrData.contractNum.asyncSet(''),
			this.newContrData.contractDate.asyncSet(''),
			this.newContrData.contractTitle.asyncSet(''),
			this.newContrData.contractStDate.asyncSet(''),
			this.newContrData.contractEndDate.asyncSet(''),
			this.newContrData.contractCurrency.asyncSet(''),
			this.newContrData.contractTotAmnt.asyncSet(''),
			this.newContrData.contractDefDays.asyncSet(''),
			this.newContrData.contractDescr.asyncSet(''),
		]);
	}
}
