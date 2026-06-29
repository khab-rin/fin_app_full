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


<section class="wide-button-section">
    <div class="wide-button-group">
        <button
            type="button"
            class="wide-button"
            onclick={makeTaxMchd}
            disabled={isMakeTaxMchdPushed}
        >
            <span class="wide-button-span">
                Создать МЧД для ФНС
            </span>
        </button>
    </div>

    <div class="wide-button-group">
        <button
            type="button"
            class="wide-button"
        >
            <span class="wide-button-span">
                Загрузить и отправить МЧД в ФНС
            </span>
        </button>
    </div>

    <div class="wide-button-group">
        <button
            type="button"
            class="wide-button"
            onclick={makeHomeMchd}
            disabled={isMakeHomeMchdPushed}
        >
            <span class="wide-button-span">
                Создать МЧД
            </span>
        
        </button>

    </div>
</section>

<section class="navi-button-section">
    <div class="navi-button-group">
        <button
            class="medium-button"
            type="button"
            onclick={handleGoBack}>
            <span class="navi-button-text">Назад</span>
        </button>
    </div>

    <div class="navi-button-group">
        <button
            class="medium-button"
            type="button"
            onclick={handleGoNext}>
            <span class="navi-button-text">Вперед</span>
        </button>
    </div>

    <div class="navi-button-group">
        <button
            class="medium-button"
            type="button"
            onclick={handleGoMain}>
            <span class="navi-button-text">Главное меню</span>
        </button>
    </div>

    
</section>


    



