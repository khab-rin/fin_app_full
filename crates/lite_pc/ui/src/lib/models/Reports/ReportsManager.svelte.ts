import type { ReportStep } from "../rustModels/ReportStep";
import { ReportType } from "./ReportValues";


import FnsReports from "$lib/service/Reports/FnsReports.svelte";
import HomeReports from "$lib/service/Reports/HomeReports.svelte";
import Loading from "$lib/service/Reports/Loading.svelte";
import SaveFiles from "$lib/service/Reports/SaveFiles.svelte";
import Trylater from "$lib/service/Reports/Trylater.svelte";

class ReportManager {
	private _step = $state<ReportStep>({
		Loading: {text: 'Выберите функционал для работы с отчетами'}
	});

	get step() {return this._step;}
	set step(nextStep: ReportStep) {
		this._step = nextStep;
	}
	
	get currentText(): string {
		if (!this._step || typeof this._step != 'object') {
			return '';
		}

		const currentStepObj = Object.values(this._step)[0];

		if (
			currentStepObj &&
			typeof currentStepObj === 'object' &&
			'text' in currentStepObj &&
			typeof (currentStepObj as {text: unknown}).text === 'string'
		) {
			return (currentStepObj as {text:string}).text;
		}
		return '';
	}

	get getPage() {
		if (ReportType.FnsReports in this._step) {
			return FnsReports;
		} else if (ReportType.HomeReports in this._step) {
			return HomeReports;
		} else if (ReportType.Loading in this._step) {
			return Loading;
		} else if (ReportType.SaveFiles in this._step){
			return SaveFiles;
		} else {
			return Trylater;
		}
	}
}

export const reportManager = new ReportManager;