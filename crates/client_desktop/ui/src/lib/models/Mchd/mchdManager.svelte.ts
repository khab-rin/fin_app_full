import { invoke } from "@tauri-apps/api/core";

import { FieldValidator } from "../Auth/FieldValidator.svelte";
import { MchdStepType } from "./MchdValues";

import type {MchdStep} from "$lib/models/rustModels/MchdStep";
import type {HomePowerInfo} from "$lib/models/rustModels/HomePowerInfo";
import type { HomeMchdPower } from "../rustModels/HomeMchdPower";

import BTBMchd from "$lib/service/mchd/BTBMchd.svelte"; 
import FnsMchd from "$lib/service/mchd/FnsMchd.svelte";
import HomeMchd from "$lib/service/mchd/HomeMchd.svelte";
import LendMchd from "$lib/service/mchd/LendMchd.svelte";
import Loading from "$lib/service/mchd/Loading.svelte";
import SaveXmlDocFiles from "$lib/service/mchd/SaveXmlDocFiles.svelte";
import ShowPowers from "$lib/service/mchd/ShowPowers.svelte";
import SuccessRegisterMchd from "$lib/service/mchd/SuccessRegisterMchd.svelte";
import TryLater from "$lib/service/mchd/TryLater.svelte";
import WrongData from "$lib/service/mchd/WrongData.svelte";


class MchdManager {
    step = $state<MchdStep>({
        Loading: {text: "Выберите действие с машиночитаемыми доверенностями. Вы можете создать и зарегистрировать МЧД на любое физ. лицо от имени организации указанной при регистрации в приложении"}
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
        taxOrgIdent: new FieldValidator("Digits4_4", "1655"),


        managerTitle: new FieldValidator("String1_255", "Директор"),
        managerSurName: new FieldValidator("SurName", "Хабипов"),
        managerFirstName: new FieldValidator("FirstName", "Ринат"),
        managerMidName: new FieldValidator("MidName", "Ришатович"),
        managerBirthDay: new FieldValidator("Date", "06.01.1985"),
        managerSnils: new FieldValidator("Snils", "11021217665"),
        managerInn: new FieldValidator("PersInn", "161101510882"),
        managerIsCitizen: new FieldValidator("IsCitizen", "1"),

        userSurName: new FieldValidator("SurName", "Хабипов"),
        userFirstName: new FieldValidator("FirstName", "Ильдар"),
        userMidName: new FieldValidator("MidName", "Ринатович"),
        userBirthDay: new FieldValidator("Date", "31.05.2009"),
        userGender: new FieldValidator("Gender", "1"),
        userSnils: new FieldValidator("Snils", "16293848705"),
        userInn: new FieldValidator("PersInn", "166021488126"),
        userPassportNumber: new FieldValidator("PasspRfNumber", "9223 381140"),
        userPassportIssueDate: new FieldValidator("Date", "27.09.2023"),
        userPassportIssueer: new FieldValidator("String1_4000", "МВД ПО РЕСПУБЛИКЕ ТАТАРСТАН"),
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
        if (MchdStepType.BTBMchd in step) {
            return BTBMchd
        } else if (MchdStepType.FnsMchd in step) {
            return FnsMchd
        } else if (MchdStepType.HomeMchd in step) {
            return HomeMchd
        } else if (MchdStepType.LendMchd in step) {
            return LendMchd
        } else if (MchdStepType.Loading in step) {
            return Loading
        } else if (MchdStepType.SaveXmlDocFiles in step) {
            return SaveXmlDocFiles
        } else if (MchdStepType.ShowPowers in step) {
            return ShowPowers
        } else if (MchdStepType.SuccessRegisterMchd in step) {
            return SuccessRegisterMchd
        } else if (MchdStepType.TryLater in step) {
            return TryLater
        } else if (MchdStepType.WrongData in step) {
            return WrongData
        } else {
            return null
        }
    }

    async get_power_info(power: HomeMchdPower) {
        try {
            return await invoke<HomePowerInfo>("cmd_get_power_info", { power: power })
        } catch(err) {
            console.log("function get_power_info FAILED BY cmd_get_power_info, err = ", err);
        }
    }

}

export const currentMchdStep = new MchdManager;