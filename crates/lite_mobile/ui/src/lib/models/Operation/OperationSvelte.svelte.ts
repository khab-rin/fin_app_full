import { FieldValidator } from "../Auth/FieldValidator.svelte";
import type { OperationRaw } from "../rustModels/OperationRaw";
import type { Operation } from "../rustModels/Operation";
import type { Account } from "../rustModels/Account";
import type { DocType } from "../rustModels/DocType";

export class OperationSvelte {
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

    constructor(raw?: OperationRaw) {
        if (raw) {
            this.data.operId.value = raw.oper_id;
            this.data.userId.value = raw.user_id;
            this.data.compId.value = raw.comp_id;

            this.data.ctrptyId.value = raw.ctrpty?.comp_id ?? "";
            this.data.ctrptyName.value = raw.ctrpty?.metadata?.comp_name?.short_egrul_name ?? "";

            this.data.contractNum.value = raw.contract.current?.contract_num ?? "";
            this.data.contractDate.value = raw.contract.current?.contract_date ?? "";
            
            
            this.data.debet.value = raw.debet;
            this.data.credit.value = raw.credit;
            this.data.amount.value = raw.amount;
            this.data.operDate.value = raw.oper_date ?? "";

            this.data.docType.value = raw.doc_type;
            this.data.docNum.value = raw.doc_num;
            this.data.docDate.value = raw.doc_date;

            this.data.isStorno = raw.is_storno;
            this.data.isDel = raw.is_del;

            this.data.entrDate.value = raw.entr_date;

            this.data.isConfirmed = false;
        }
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