<script lang='ts'>
	import { pageManager } from "$lib/models/MainManager/MainManager.svelte";
	import { PageType } from "$lib/models/MainManager/PageValues";


	function goToOper() {
		pageManager.Page = PageType.Operation;
	}

	function goToMchd() {
		pageManager.Page = PageType.Mchd;
	}

	function goToReport() {
		pageManager.Page = PageType.Report;
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

