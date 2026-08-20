import { FieldValidator } from "../Auth/FieldValidator.svelte";
import type { OperationRaw } from "../rustModels/OperationRaw";
import type { Operation } from "../rustModels/Operation";
import type { Account } from "../rustModels/Account";
import type { DocType } from "../rustModels/DocType";
import type {Company} from '$lib/models/rustModels/Company';
import type { Contract } from "../rustModels/Contract";
import type { OperationStep } from "../rustModels/OperationStep";
import { invoke } from "@tauri-apps/api/core";
import type { NewContrData } from "../rustModels/NewContrData";

export class OperationSvelte {


    private _isAccountsCompatible = $state(false);
    _cntrPty: Company | null = $state(null);
    _allPossContracts: Contract[] = $state([]);
    _isConfirmed = $state(false);
    _isDuplicate = $state(false);


    data = $state({
        operId: new FieldValidator("BoxUuid", ""),
        userId: new FieldValidator("BoxUuid", ""),

        compId: new FieldValidator("BoxUuid", ""),
        ctrptyId: new FieldValidator("BoxUuid", ""),
        ctrptyName: new FieldValidator("CompanyName", ""),

        contractId: new FieldValidator("BoxUuid", ""),
        contractNum: new FieldValidator("DocNum", ""),
        contractDate: new FieldValidator("Date", ""),

        debet: new FieldValidator("Account", ""),
        credit: new FieldValidator("Account", ""),
        amount: new FieldValidator("RubF", ""),
        operDate: new FieldValidator("Date", ""),

        docType: new FieldValidator("DocType", ""),
        docNum: new FieldValidator("DocNum", ""),
        docDate: new FieldValidator("Date", ""),

        isStorno: false,
        isDel: false,

        entrDate: new FieldValidator("Date", ""),
    })

    constructor() {}

    static async fromRaw(raw: OperationRaw): Promise<OperationSvelte> {
        const instance = new OperationSvelte();
        instance._allPossContracts = raw.contract.contracts;
        instance._cntrPty = raw.ctrpty;
        
        instance.data.operId.value = raw.oper_id;
        instance.data.userId.value = raw.user_id;
        instance.data.compId.value = raw.comp_id;

        instance.data.ctrptyId.value = raw.ctrpty?.comp_id ?? "";
        instance.data.ctrptyName.value = raw.ctrpty?.metadata?.comp_name?.short_egrul_name ?? "";

        
        instance.data.contractId.value = raw.contract.current?.contract_id ?? "";
        instance.data.contractNum.value = raw.contract.current?.contract_num ?? "";
        instance.data.contractDate.value = raw.contract.current?.contract_date ?? "";

        instance.data.debet.value = raw.debet;
        instance.data.credit.value = raw.credit;
        instance.compateAccounts();
        instance.data.amount.value = raw.amount;
        instance.data.operDate.value = raw.oper_date ?? "";

        instance.data.docType.value = raw.doc_type;
        await instance.data.docType.validate();

        instance.data.docNum.value = raw.doc_num;
        instance.data.docDate.value = raw.doc_date;

        instance.data.isStorno = raw.is_storno;
        instance.data.isDel = raw.is_del;
        instance.data.entrDate.value = raw.entr_date;

        return instance;
    }
    
    isValid = $derived(
        this.data.operId.isValid &&
        this.data.userId.isValid &&

        this.data.compId.isValid &&
        this.data.ctrptyId.isValid &&

        this.data.contractId.isValid &&

        this.data.debet.isValid &&
        this.data.credit.isValid &&
        this.data.amount.isValid &&
        this.data.operDate.isValid &&

        this.data.docType.isValid &&
        this.data.docNum.isValid &&
        this.data.docDate.isValid &&

        this.data.entrDate.isValid
    );

    async refreshCtrpty(compInn: string, kpp: string) {
        let data = {compInn: compInn, kpp: kpp};
        const newCompany: Company | null = await invoke<Company>(
            "cmd_get_comp_by_inn_kpp", 
            data
        );
        this.data.ctrptyId.value = newCompany.comp_id;
        this.data.ctrptyName.value = newCompany.metadata.comp_name?.short_egrul_name ?? "";

        const contracts = await invoke<Contract[]>(
            "cmd_get_contracts_by_ctrpty_id",
            {ctrptyId: newCompany.comp_id}
        );
        this._allPossContracts = contracts;
        this.data.contractId.value = "";
        this.data.contractDate.value = "";
        this.data.contractNum.value = "";
        const [operId: BoxUuid, flag: Boolean] = ['1', false];
    }

    

    async compateAccounts() {
        try {
            let leftAcc = this.data.debet.value;
            let rigthAcc = this.data.credit.value;
            this._isAccountsCompatible = await invoke<boolean>(
                "cmd_is_accounts_compatible",
                {leftAcc:leftAcc, rigthAcc: rigthAcc}
            )
        } catch(err) {
            console.error("cmd_is_accounts_compatible FAILED, err = ", err);
            this._isAccountsCompatible = false;
        }
        
    }
    
    get isAccountsCompatible() {
        return this._isAccountsCompatible
    }



    

    async refreshContracts(data: NewContrData) {
        const freshContracts: Contract[] = await invoke<Contract[]>(
            "cmd_add_new_contract",
            {data: data}
        );
        this._allPossContracts = freshContracts;
        this.data.contractId.value = "";
        this.data.contractDate.value = "";
        this.data.contractNum.value = "";
    }

    refreshContract(contract: Contract) {
        this.data.contractId.value = contract.contract_id;
        this.data.contractDate.value = contract.contract_date;
        this.data.contractNum.value = contract.contract_num;
    }

    get contractStr(): string {
        const num = this.data.contractNum.value;
        const d = this.data.contractDate.value;
        const id = this.data.contractId.value;
        if (!num || !d || !id) {return "без договора"}
        return `Договор № ${num} от ${d}`;
    }



    makeRust(): Operation | null {
        if (!this.isValid) {
            return null;
        }

        return { 
            oper_id: this.data.operId.value,
            user_id: this.data.userId.value, 
            comp_id: this.data.compId.value, 
            ctrpty_id: this.data.ctrptyId.value, 
            contract_id: this.data.contractId.value, 
            debet: this.data.debet.value as Account, 
            credit: this.data.credit.value  as Account, 
            amount: this.data.amount.value, 
            oper_date: this.data.operDate.value, 
            doc_type: this.data.docType.value as DocType,
            doc_num: this.data.docNum.value, 
            doc_date: this.data.docDate.value, 
            is_storno: this.data.isStorno, 
            is_del: this.data.isDel, 
            entr_date: this.data.entrDate.value, 
        };
    }

}