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
    private _step = $state<MchdStep>({
        Loading: {text: "Выберите действие с машиночитаемыми доверенностями. Вы можете создать и зарегистрировать МЧД на любое физ. лицо от имени организации указанной при регистрации в приложении"}
    });

	get step() {return this._step}
	set step(nextStep: MchdStep) {
		this._step = nextStep;
	}

    data = $state({
        PoaNumber: new FieldValidator("String1_50", ""),
        PoaEndDate: new FieldValidator("Date", ""),
        taxOrgIdent: new FieldValidator("Digits4_4", ""),

        managerTitle: new FieldValidator("String1_255", ""),
        managerSurName: new FieldValidator("SurName", ""),
        managerFirstName: new FieldValidator("FirstName", ""),
        managerMidName: new FieldValidator("MidName", ""),
        managerBirthDay: new FieldValidator("Date", ""),
        managerSnils: new FieldValidator("Snils", ""),
        managerInn: new FieldValidator("PersInn", ""),
        managerIsCitizen: new FieldValidator("IsCitizen", ""),

        userSurName: new FieldValidator("SurName", ""),
        userFirstName: new FieldValidator("FirstName", ""),
        userMidName: new FieldValidator("MidName", ""),
        userBirthDay: new FieldValidator("Date", ""),
        userGender: new FieldValidator("Gender", ""),
        userSnils: new FieldValidator("Snils", ""),
        userInn: new FieldValidator("PersInn", ""),
        userPassportNumber: new FieldValidator("PasspRfNumber", ""),
        userPassportIssueDate: new FieldValidator("Date", ""),
        userPassportIssueer: new FieldValidator("String1_4000", ""),
        userPassportUssuerCode: new FieldValidator("String7_7", ""),
        userIsCitizen: new FieldValidator("IsCitizen", ""),
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

	reload() {
		console.error('[currentMchdStep reload] before');
		this.step = {Loading: {text: "Выберите действие с машиночитаемыми доверенностями. Вы можете создать и зарегистрировать МЧД на любое физ. лицо от имени организации указанной при регистрации в приложении"}};
		console.error('[currentMchdStep reload] after');
	}

	async loadUser1() {
		try {
			await Promise.all([
				this.data.PoaNumber.asyncSet("1"),
				this.data.PoaEndDate.asyncSet("30.10.2026"),
				this.data.taxOrgIdent.asyncSet("1655"),
				this.data.managerTitle.asyncSet("Директор"),
				this.data.managerSurName.asyncSet("Руководитель"),
				this.data.managerFirstName.asyncSet("Иван"),
				this.data.managerMidName.asyncSet("Иванович"),
				this.data.managerBirthDay.asyncSet("01.01.2000"),
				this.data.managerSnils.asyncSet("11021217665"),
				this.data.managerInn.asyncSet("161101510882"),
				this.data.managerIsCitizen.asyncSet("1"),

				this.data.userSurName.asyncSet("Сотрудник"),
				this.data.userFirstName.asyncSet("Петр"),
				this.data.userMidName.asyncSet("Петрович"),
				this.data.userBirthDay.asyncSet("01.01.2000"),
				this.data.userGender.asyncSet("1"),
				this.data.userSnils.asyncSet("12233162502"),
				this.data.userInn.asyncSet("161101511050"),
				this.data.userPassportNumber.asyncSet("1234567890"),
				this.data.userPassportIssueDate.asyncSet("01.01.2000"),
				this.data.userPassportIssueer.asyncSet("Отделением УФМС г. Казани"),
				this.data.userPassportUssuerCode.asyncSet("160-000"),
				this.data.userIsCitizen.asyncSet("1"),

			])
		} catch(err) {
			let err_step: MchdStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
			console.error("operStep.loadUser1 FAILED, err = ", err);
			currentMchdStep.step = err_step;	
		}
	}

}

export const currentMchdStep = new MchdManager;