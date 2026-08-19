import { FieldValidator } from "../Auth/FieldValidator.svelte";
import type { OperationRaw } from "../rustModels/OperationRaw";
import type { Operation } from "../rustModels/Operation";
import type { Account } from "../rustModels/Account";
import type { DocType } from "../rustModels/DocType";
import type {Company} from '$lib/models/rustModels/Company';
import type { Contract } from "../rustModels/Contract";

export class OperationSvelte {
    data = $state({
        operId: new FieldValidator("BoxUuid", ""),
        userId: new FieldValidator("BoxUuid", ""),
        compId: new FieldValidator("BoxUuid", ""),

        ctrptyId: new FieldValidator("BoxUuid", ""),
        ctrptyName: new FieldValidator("CompanyName", ""),

        allPossContracts: [] as Contract[],
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

        isConfirmed: false
    })

    

    isValid = $derived(
        this.data.operId.isValid &&
        this.data.userId.isValid &&
        this.data.compId.isValid &&

        this.data.ctrptyId.isValid &&
        this.data.ctrptyName.isValid &&

        this.data.contractId.isValid &&
        this.data.contractNum.isValid &&
        this.data.contractDate.isValid &&

        this.data.debet.isValid &&
        this.data.credit.isValid &&
        this.data.amount.isValid &&
        this.data.operDate.isValid &&

        this.data.docType.isValid &&
        this.data.docNum.isValid &&
        this.data.docDate.isValid &&
        this.data.isConfirmed
    );

    constructor() {}

    static async fromRaw(raw: OperationRaw): Promise<OperationSvelte> {
        const instance = new OperationSvelte();
        
        instance.data.operId.value = raw.oper_id;
        instance.data.userId.value = raw.user_id;
        instance.data.compId.value = raw.comp_id;

        instance.data.ctrptyId.value = raw.ctrpty?.comp_id ?? "";
        instance.data.ctrptyName.value = raw.ctrpty?.metadata?.comp_name?.short_egrul_name ?? "";

        instance.data.allPossContracts = raw.contract.contracts;
        instance.data.contractId.value = raw.contract.current?.contract_id ?? "";
        instance.data.contractNum.value = raw.contract.current?.contract_num ?? "";
        instance.data.contractDate.value = raw.contract.current?.contract_date ?? "";

        instance.data.debet.value = raw.debet;
        instance.data.credit.value = raw.credit;
        instance.data.amount.value = raw.amount;
        instance.data.operDate.value = raw.oper_date ?? "";

        instance.data.docType.value = raw.doc_type;
        await instance.data.docType.validate();

        instance.data.docNum.value = raw.doc_num;
        instance.data.docDate.value = raw.doc_date;

        instance.data.isStorno = raw.is_storno;
        instance.data.isDel = raw.is_del;
        instance.data.entrDate.value = raw.entr_date;
        instance.data.isConfirmed = false;

        return instance;
    }


    refreshCtrpty(ctrpty?: Company) {
        if (ctrpty) {
            this.data.ctrptyId.value = ctrpty.comp_id;
            this.data.ctrptyName.value = ctrpty.metadata.comp_name?.short_egrul_name ?? "";
        }
    }

    refreshContracts(contracts: Contract[]) {
        this.data.allPossContracts = contracts;
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