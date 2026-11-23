<script lang='ts'>
	import { pageManager } from "$lib/models/MainManager/MainManager.svelte";
	import { PageType } from "$lib/models/MainManager/PageValues";

	import {dbStep} from '$lib/models/DataBase/DbManager.svelte';
	import {operStep} from '$lib/models/Operation/OperationManager.svelte';
	import {reportManager} from '$lib/models/Reports/ReportsManager.svelte'
	import {currentMchdStep} from '$lib/models/Mchd/mchdManager.svelte';



	function goToOper() {
		operStep.reload();
		pageManager.Page = PageType.Operation;
	}

	function goToMchd() {
		currentMchdStep.reload();
		pageManager.Page = PageType.Mchd;
	}

	function goToReport() {
		reportManager.reload();
		pageManager.Page = PageType.Report;
	}

	function goToDb() {
		dbStep.reload();
		pageManager.Page = PageType.DataBase;
	}


</script>

<div class='w-100'>
	<div class='w-v100'>
		<label class='label-close' for='mainGoToOper'>
			Раздел проводок и операций
		</label>
		<button 
			class='but-bl'
			id='mainGoToOper'
			onclick={goToOper}
		>
			Оперции
		</button>
	</div>
</div>


<div class='w-100'>
	<div class='w-v100'>
		<label class='label-close' for='mainGoToReport'>
			Раздел отчетов
		</label>
		<button 
			class='but-bl'
			id='mainGoToReport'
			onclick={goToReport}
		
		>
			Отчеты
		</button>
	</div>
</div>

<div class='w-100'>
	<div class='w-v100'>
		<label class='label-close' for='maingoToMchd'>
			Раздел электронных доверенностей
		</label>
		<button 
			class='but-bl'
			id='maingoToMchd'
			onclick={goToMchd}
		>
			Доверенности
		</button>
	</div>
</div>

<div class='w-100'>
	<div class='w-v100'>
		<label class='label-close' for='maingoToMchd'>
			Действия с базой данных
		</label>
		<button 
			class='but-bl'
			id='maingoToMchd'
			onclick={goToDb}
		>
			База данных
		</button>
	</div>
</div>

