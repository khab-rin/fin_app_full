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

<div class='w-100'>
	<p class='x-cont-c x-self-c with-info'>
		{operStep.currentText}
	</p>
</div>


{#if operStep.getPage}
    <svelte:component this={operStep.getPage} />
{:else}
    <p>Загрузка или ошибка...</p>
{/if}


<div class='w-100'>
	<div class='w-v100'>
		<button
			type='button'
			class='but-pu'
			onclick={goToLoading}
		>
			<span>
				Меню операций
			</span>
		</button>
	</div>
</div>

<div class='w-100'>
	<div class='w-v100'>
		<button
			type="button"
			class="but-pu"
			onclick={closeOper}
			>
			<span>
				Основной экран
			</span>
		</button>
	</div>
</div>
