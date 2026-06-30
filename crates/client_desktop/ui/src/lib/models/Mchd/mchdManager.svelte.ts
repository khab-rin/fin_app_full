import { invoke } from "@tauri-apps/api/core";

import { FieldValidator } from "../Auth/FieldValidator.svelte";
import { MchdStepType } from "./MchdValues";

import type {MchdStep} from "$lib/models/rustModels/MchdStep";
import type {MchdPowerInfo} from "$lib/models/rustModels/MchdPowerInfo";
import type { MchdTaxFields } from "../rustModels/MchdTaxFields";

import HomeMchd from "$lib/service/mchd/HomeMchd.svelte";
import LendMchd from "$lib/service/mchd/LendMchd.svelte";
import Loading from "$lib/service/mchd/Loading.svelte";
import ShowPowers from "$lib/service/mchd/ShowPowers.svelte";
import Success from "$lib/service/mchd/Success.svelte";
import TaxMchd from "$lib/service/mchd/TaxMchd.svelte";
import WrongData from "$lib/service/mchd/WrongData.svelte";


import TryLater from "$lib/service/mchd/TryLater.svelte";



class MchdManager {
    step = $state<MchdStep>({
        Loading: {text: "Выберите тип доверенности, который нужно создать"}
    });
    private steps: MchdStep[] = $state([]);
    private index = $state(0);

    constructor() {
        this.steps.push(this.step);
    }

    next() {
        if (this.index < this.steps.length - 1) {
            this.index++;
            this.step = this.steps[this.index];
        }
    }

    back() {
        if (this.index > 0) {
            this.index--;
            this.step = this.steps[this.index];
        }
    }

    add(next_step: MchdStep) {
        this.steps.length = this.index + 1;
        this.steps.push(next_step);
        this.index++;
        this.step = next_step;
    }

    data = $state({
        PoaNumber: new FieldValidator("String1_50", "1"),
        PoaEndDate: new FieldValidator("Date", "18.06.2028"),

        managerTitle: new FieldValidator("String1_255", "Директор"),
        managerSurName: new FieldValidator("SurName", "Хабипов"),
        managerFirstName: new FieldValidator("FirstName", "Ринат"),
        managerMidName: new FieldValidator("MidName", "Ришатович"),
        managerBirthDay: new FieldValidator("Date", "06.01.1985"),
        managerSnils: new FieldValidator("Snils", "16293848705"),
        managerInn: new FieldValidator("PersInn", "166021488126"),
        managerIsCitizen: new FieldValidator("IsCitizen", "1"),

        userSurName: new FieldValidator("SurName", "Хабипов"),
        userFirstName: new FieldValidator("FirstName", "Ринат"),
        userMidName: new FieldValidator("MidName", "Ришатович"),
        userBirthDay: new FieldValidator("Date", "06.01.1985"),
        userGender: new FieldValidator("Gender", "1"),
        userSnils: new FieldValidator("Snils", "11021217665"),
        userInn: new FieldValidator("PersInn", "161101510882"),
        userPassportNumber: new FieldValidator("PasspRfNumber", "9207 360338"),
        userPassportIssueDate: new FieldValidator("Date", "15-05-2008"),
        userPassportIssueer: new FieldValidator("String1_4000", "Отделением в Советском районе отдела УФМС России по Республике Татарстан в городе Казани"),
        userPassportUssuerCode: new FieldValidator("String7_7", "160-009"),
        userIsCitizen: new FieldValidator("IsCitizen", "1"),
    })

    get currentText(): string {
        if (!this.step || typeof this.step !== 'object') {
            return '';
        }

        const currentStepObj = Object.values(this.step)[0];

        if (
            currentStepObj && 
            typeof currentStepObj === 'object' && 
            'text' in currentStepObj && 
            typeof (currentStepObj as { text: unknown }).text === 'string'
        ) {
            return (currentStepObj as { text: string }).text;
        }
        
        return '';
    }

    get getPage() {
        const step = this.step;
        if (MchdStepType.HomeMchd in step) {
            return HomeMchd
        } else if (MchdStepType.LendMchd in step) {
            return LendMchd
        } else if (MchdStepType.Loading in step) {
            return Loading
        } else if (MchdStepType.ShowPowers in step) {
            return ShowPowers
        } else if (MchdStepType.Success in step) {
            return Success
        } else if (MchdStepType.TaxMchd in step) {
            return TaxMchd
        } else if (MchdStepType.TryLater in step) {
            return TryLater
        } else if (MchdStepType.WrongData in step) {
            return WrongData
        } else {
            return null
        }
    }

    async get_power_info(power: MchdTaxFields) {
        try {
            return await invoke<MchdPowerInfo>("cmd_get_power_info", { power: power })
        } catch(err) {
            console.log("function get_power_info FAILED BY cmd_get_power_info, err = ", err);
        }
    }

}

export const currentMchdStep = new MchdManager;