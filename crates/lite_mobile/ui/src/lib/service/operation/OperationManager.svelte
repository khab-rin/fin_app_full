<script lang='ts'>
    import {operStep} from '$lib/models/Operation/OperationManager.svelte';
    import {pageManager} from '$lib/models/MainManager/MainManager.svelte';
    import type {OperationStep} from '$lib/models/rustModels/OperationStep';
	import { currAuthStep } from '$lib/models/Auth/AuthStep.svelte';

    function closeOper() {
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


<div class="main-button-group">
	<button
		type='button'
		class='main-button'
		onclick={goToLoading}
	>
		Меню операций
	</button>

    <button
        type="button"
        class="main-button"
        onclick={closeOper}
        >
        <span class="main-button-span">
            Основной экран
        </span>
    </button>
</div>