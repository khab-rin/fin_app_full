<script lang='ts'>
    import {operStep} from '$lib/models/Operation/OperationManager.svelte';
    import {pageManager} from '$lib/models/MainManager/MainManager.svelte';
    import type {OperationStep} from '$lib/models/rustModels/OperationStep';

    function closeOper() {
		const next_step: OperationStep = {Loading:{text: 'Выберите функционал работы с проводками'}};
		operStep.step = next_step;
        pageManager.Page = null;
    }

	function goToLoading() {
		const next_step: OperationStep = {Loading:{text: 'Выберите функционал работы с проводками'}};
		operStep.step = next_step;
	}
</script>

<p class="text-small">{operStep.currentText}</p>

{#if operStep.getPage}
    <svelte:component this={operStep.getPage} />
{:else}
    <p>Загрузка или ошибка...</p>
{/if}


<div class='group-one'>
	<div>
		<button
			type='button'
			class='blue-button'
			onclick={goToLoading}
		>
			<span class="blue-button-span">
				Меню операций
			</span>
		</button>
	</div>

	<div>
		<button
			type="button"
			class="blue-button"
			onclick={closeOper}
			>
			<span class="blue-button-span">
				Основной экран
			</span>
		</button>
	</div>


</div>
