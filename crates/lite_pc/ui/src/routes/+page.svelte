<script lang="ts">
	import '$lib/style/global.css'

	import {pageManager} from '$lib/models/MainManager/MainManager.svelte'
	import { PageType } from "$lib/models/MainManager/PageValues";
	import { operStep } from '$lib/models/Operation/OperationManager.svelte';
	import {currentMchdStep} from '$lib/models/Mchd/mchdManager.svelte';
	import {reportManager} from '$lib/models/Reports/ReportsManager.svelte';

	function goToOperation() {
		pageManager.Page = PageType.Operation;
		operStep.reload();
	}

	function goToMchd() {
		pageManager.Page = PageType.Mchd;
		currentMchdStep.reload()
	}

	function goToReports() {
		pageManager.Page = PageType.Report;
		reportManager.reload()
	}

	function goToDb() {
		pageManager.Page = PageType.DataBase;
	}

	async function logOut() {
		await pageManager.logOut();
	}

</script>


{#if !pageManager.totalOff}
	<div class='w-100'>
		<div class='w-v100'>
			<label
				class='label-close x-self-l'
				for='mainOperationButton'
			>
				Раздел операций
			</label>
			<button
				type='button'
				class='but-bl'
				id='mainOperationButton'
				disabled={pageManager.totalOff}
				onclick={goToOperation}
			>
				<span class='t-base x-self-c  t-bold'>
					Операции
				</span>
			</button>
		</div>
	</div>

	<div class='w-100'>

		<div class='w-v100'>
			<label
				class='label-close x-self-l'
				for='mainReportsButton'
			>
				Раздел отчетов
			</label>
			<button
				type='button'
				class='but-bl'
				id='mainReportsButton'
				disabled={pageManager.totalOff}
				onclick={goToReports}
			>
				<span class='t-base x-self-c t-bold'>
					Отчеты
				</span>
			</button>
		</div>
	</div>

	<div class='w-100'>
		<div class='w-v100'>
			<label
				class='label-close x-self-l'
				for='mainMchdButton'
			>
				Раздел доверенностей
			</label>
			<button
				type='button'
				class='but-bl'
				id='mainMchdButton'
				disabled={pageManager.totalOff}
				onclick={goToMchd}
			>
				<span class='t-base x-self-c t-bold'>
					Доверенности
				</span>
			</button>
		</div>
	</div>

	<div class='w-100'>
		<div class='w-v100'>
			<label
				class='label-close x-self-l'
				for='mainDataBase'
			>
				Действия с базой данных
			</label>
			<button
				type='button'
				class='but-bl'
				id='mainMchdButton'
				disabled={pageManager.totalOff}
				onclick={goToDb}
			>
				<span class='t-base x-self-c t-bold'>
					База данных
				</span>
			</button>
		</div>
	</div>
{/if}