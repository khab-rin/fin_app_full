<script lang='ts'>
    import {invoke} from "@tauri-apps/api/core"


    import type {MchdStep} from "$lib/models/rustModels/MchdStep";
    import {currentMchdStep} from "$lib/models/Mchd/mchdManager.svelte";

    let isBTBPushed = $state(false);
    let isFnsPushed = $state(false);
    let isHomePushed = $state(false);
    let isLendPushed = $state(false);
    let isShowPowersPushed = $state(false);

    async function goToBTBMchd() {
        isBTBPushed = true;
        const next_step: MchdStep = {BTBMchd: {text: "Вы на этапе создания доверенности для электронного документооборота с контрагентами, внимательно заполните поля аналогично полям в личных и учредительных документах"}};
        isBTBPushed = false;
        currentMchdStep.add(next_step)
    }

    async function goToFnsMchd() {
        isFnsPushed = true;
        const next_step: MchdStep = {FnsMchd: {text: "Вы на этапе создания доверенности для сдачи отчетности в ФНС, внимательно заполните поля аналогично полям в личных и учредительных документах"}};
        isFnsPushed = false;
        currentMchdStep.add(next_step)
    }

    async function goToHomeMchd() {
        isHomePushed = true;
        const next_step: MchdStep = {HomeMchd: {text: "Вы на этапе создания доверенности для допуска к ветвям функционала данной системы, внимательно заполните поля аналогично полям в личных и учредительных документах"}};
        isHomePushed = false;
        currentMchdStep.add(next_step)
    }


    async function goToLendMchd() {
        isLendPushed = true;
        const next_step: MchdStep = {LendMchd: { text: "Выберите ранее созданный xml файл доверенности, отсоединенный фалй подписи руководителя организации и отправьте доверенность для регистрации в сервисе МЧД"}}
        isLendPushed = false;
        currentMchdStep.add(next_step)
    }

    async function goToShowPowers() {
        isShowPowersPushed = true;
        try {
            const next_step = await invoke<MchdStep>("cmd_show_powers", {});
            isShowPowersPushed = false;
            currentMchdStep.add(next_step);
        } catch (err) {
            const next_step: MchdStep = {TryLater:{text: "Критическая ошибка на устройстве..."}};
            isShowPowersPushed = false;
            console.error("cmd_show_powers FAILED, err = ", err);
            currentMchdStep.add(next_step);
        }
    }
</script>


<section class="wide-button-section">
    <div class="wide-button-group">
        <button
            type="button"
            class="wide-button"
            onclick={goToBTBMchd}
            disabled={isBTBPushed}
        >
            <span class="wide-button-span">
                Создать B2B МЧД
            </span>
        </button>
    </div>

    <div class="wide-button-group">
        <button
            type="button"
            class="wide-button"
            onclick={goToFnsMchd}
            disabled={isFnsPushed}
        >
            <span class="wide-button-span">
                Создать МЧД для отчетности
            </span>
        </button>
    </div>

    <div class="wide-button-group">
        <button
            type="button"
            class="wide-button"
            onclick={goToHomeMchd}
            disabled={isHomePushed}
        >
            <span class="wide-button-span">
                Создать МЧД для работы в системе
            </span>
        </button>
    </div>


    <div class="wide-button-group">
        <button
            type="button"
            class="wide-button"
            onclick={goToLendMchd}
            disabled={isLendPushed}

            >
            <span class='wide-button-span'>
                Отправить подписанный МЧД
            </span>
        </button>
    </div>

    <div class="wide-button-group">
        <button
            type="button"
            class="wide-button"
            disabled={isShowPowersPushed}
            onclick={goToShowPowers}
            >
            <span class='wide-button-span'>
                Посмотреть текущие полномочия
            </span>
        </button>
    </div>

</section>




    



