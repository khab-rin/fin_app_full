import { FieldValidator } from "../Auth/FieldValidator.svelte";
import type { OperationRaw } from "../rustModels/OperationRaw";

class OperationSvelte {
    data = $state({
        OperId: new FieldValidator("BoxUuid", ""),
        UserId: new FieldValidator("BoxUuid", ""),

        CompId: new FieldValidator("BoxUuid", ""),
        CtrptyId: new FieldValidator("BoxUuid", ""),
        ContractId: new FieldValidator("BoxUuid", ""),

        Debet: new FieldValidator("Account", ""),
        Credit: new FieldValidator("Account", ""),
        Amount: new FieldValidator("RubF", ""),
        OperDate: new FieldValidator("Date", ""),

        DocType: new FieldValidator("DocType", ""),
        DocNum: new FieldValidator("DocNum", ""),
        DocDate: new FieldValidator("Date", ""),

        IsStorno: false,
        IsDel: false,

        EntrDate: new FieldValidator("Date", ""),

        ExternalId: 0n
    })

    constructor(raw?: OperationRaw) {
        if (raw) {
            this.data.OperId.value = raw.oper_id;
            this.data.UserId.value = raw.user_id;

            this.data.CompId.value = raw.comp_id;
            this.data.CtrptyId.value = raw.ctrpty.comp_id;

            if (raw) {
                const activeContract = raw.contract.current ?? raw.contract.contracts[0];
                if (activeContract) {
                    this.data.ContractId.value = activeContract.contract_id;
                }
            }
            
            this.data.Debet.value = raw.debet;
            this.data.Credit.value = raw.credit;
            this.data.Amount.value = raw.amount;
            this.data.OperDate.value = raw.oper_date ?? "";

            this.data.DocType.value = raw.doc_type;
            this.data.DocNum.value = raw.doc_num;
            this.data.DocDate.value = raw.doc_date;

            this.data.IsStorno = raw.is_storno;
            this.data.IsDel = raw.is_del;

            this.data.EntrDate.value = raw.entr_date;

            this.data.ExternalId = raw.external_id
        }
    }
}