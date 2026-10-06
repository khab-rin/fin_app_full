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

<section class='w-100'>
	<h4 class="t-fill w-g100 x-ce">
		{operStep.currentText}
	</h4>
</section>


{#if operStep.getPage}
    <svelte:component this={operStep.getPage} />
{:else}
    <p>Загрузка или ошибка...</p>
{/if}


<div class='w-40'>
	<div class='w-v100'>
		<button
			type='button'
			class='but-bl'
			onclick={goToLoading}
		>
			<span class="but-bl-span">
				Меню операций
			</span>
		</button>
	</div>
</div>
