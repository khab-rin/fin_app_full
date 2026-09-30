<script lang="ts">
  import {pageManager} from '$lib/models/MainManager/MainManager.svelte';
  import { PageType } from "$lib/models/MainManager/PageValues";


  function goToOperation() {
    pageManager.Page = PageType.Operation;
  }

  function goToMchd() {
	pageManager.Page = PageType.Mchd;
  }

  function goToReports() {
	pageManager.Page = PageType.Report;
  }

</script>


<section class='group-one'>
	<div>
		<label
			class='blue-field-label'
			for='mainOperationButton'
		>
			Раздел операций
		</label>
		<button
			type='button'
			class='purple-button'
			id='mainOperationButton'
			disabled={pageManager.totalOff}
			onclick={goToOperation}
		>
			Операции
		</button>
	</div>

	<div>
		<label
			class='blue-field-label'
			for='mainReportsButton'
		>
			Раздел отчетов
		</label>
		<button
			type='button'
			class='purple-button'
			id='mainReportsButton'
			disabled={pageManager.totalOff}
			onclick={goToReports}
		>
			Отчеты
		</button>
	</div>

	<div>
		<label
			class='blue-field-label'
			for='mainMchdButton'
		>
			Раздел доверенностей
		</label>
		<button
			type='button'
			class='purple-button'
			id='mainMchdButton'
			disabled={pageManager.totalOff}
			onclick={goToMchd}
		>
			Доверенности
		</button>
	</div>


</section>