<script lang='ts'>
    import {} from "$lib/models/Mchd/mchdManager.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import type {MchdStep} from "$lib/models/rustModels/MchdStep";
    import {currentMchdStep} from "$lib/models/Mchd/mchdManager.svelte";
    import {pageManager} from "$lib/models/MainManager/MainManager.svelte";

    let isMakeTaxMchdPushed = $state(false);
    let isMakeHomeMchdPushed = $state(false);

    async function makeTaxMchd() {
        try {
            isMakeTaxMchdPushed = true;
            const next_step = await invoke<MchdStep>("cmd_check_user_mchd_tax");
            currentMchdStep.step = next_step;
            isMakeTaxMchdPushed = false;
            currentMchdStep.add(next_step)
        } catch(err) {
            isMakeTaxMchdPushed = false;
            console.log("FUN makeTaxMchd FAILED BY cmd_check_user_mchd_tax, err = ", err);
            const next_step: MchdStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            currentMchdStep.step = next_step;
        }
    }

    async function makeHomeMchd() {
        try {
            isMakeHomeMchdPushed = true;
            const next_step = await invoke<MchdStep>("cmd_check_user_mchd_home");
            isMakeHomeMchdPushed = false;
            currentMchdStep.add(next_step)

        } catch(err) {
            isMakeHomeMchdPushed = false;
            console.log("FUN makeTaxMchd FAILED BY cmd_check_user_mchd_tax, err = ", err);
            const next_step: MchdStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            currentMchdStep.step = next_step;
        }
    }

    function handleGoBack() {
        currentMchdStep.back(); 
    }

    function handleGoNext() {
        currentMchdStep.next(); 
    }

    function handleGoMain() {
        pageManager.Page = null; 
    }

    
</script>

<div class="mchd-main">
    <section class="medium-button-grid">
        <button
            type="button" 
            class="medium-button"
            onclick={makeTaxMchd}
            disabled={isMakeTaxMchdPushed}>
                Создать МЧД для ФНС
        </button>

        <button
            type="button" 
            class="medium-button"
            onclick={makeHomeMchd}
            disabled={isMakeHomeMchdPushed}>
                Создать МЧД 
        </button>

        <div class="navi-buttons-row">
            <button
                class="nav-btn"
                type="button"
                onclick={handleGoBack}>
                <span class="nav-btn-text">Назад</span>
            </button>

            <button
                class="nav-btn"
                type="button"
                onclick={handleGoMain}>
                <span class="nav-btn-text">Главная страница</span>
            </button>

            <button
                class="nav-btn"
                type="button"
                onclick={handleGoNext}>
                <span class="nav-btn-text">Вперед</span>
            </button>

        </div>
    </section>
</div>


