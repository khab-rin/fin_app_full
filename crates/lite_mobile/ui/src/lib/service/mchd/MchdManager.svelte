<script lang='ts'>
    import {currentMchdStep} from "$lib/models/Mchd/mchdManager.svelte";
    import {pageManager} from "$lib/models/MainManager/MainManager.svelte";

    import type {MchdStep} from "$lib/models/rustModels/MchdStep";

	function goToLoading() {
		const nextStep: MchdStep = {Loading: {text: ""}};
        currentMchdStep.step = nextStep;
	}

    function closeMchd() {
        const nextStep: MchdStep = {Loading: {text: ""}};
        currentMchdStep.step = nextStep;
        pageManager.Page = null;
    }
</script>

{#if currentMchdStep.currentText}
	<div class='w-100'>
		<p class="x-self-c x-cont-c with-info">{currentMchdStep.currentText}</p>
	</div>
{/if}


{#key currentMchdStep.step}
    {#if currentMchdStep.getPage}
        {@const CurrentPage = currentMchdStep.getPage}
        <CurrentPage />
    {:else}
        <p>Загрузка или ошибка...</p>
    {/if}
{/key}


<div class="w-100">
	<div class='w-v50'>
		<button
			type="button"
			class="but-pu"
			onclick={goToLoading}
			>
			<span>
				Меню доверенностей
			</span>
		</button>
	</div>

	<div class='w-v50'>
		<button
			type="button"
			class="but-pu"
			onclick={closeMchd}
			>
			<span>
				Основной экран
			</span>
		</button>
	</div>
</div>

