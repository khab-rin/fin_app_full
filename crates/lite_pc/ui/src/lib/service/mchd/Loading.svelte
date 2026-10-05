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
        const nextStep: MchdStep = {BTBMchd: {text: "Вы на этапе создания доверенности для электронного документооборота с контрагентами, внимательно заполните поля аналогично полям в личных и учредительных документах"}};
        isBTBPushed = false;
        currentMchdStep.step = nextStep;
    }

    async function goToFnsMchd() {
        isFnsPushed = true;
        const nextStep: MchdStep = {FnsMchd: {text: "Вы на этапе создания доверенности для сдачи отчетности в ФНС, внимательно заполните поля аналогично полям в личных и учредительных документах"}};
        isFnsPushed = false;
        currentMchdStep.step = nextStep;
    }

    async function goToHomeMchd() {
        isHomePushed = true;
        const nextStep: MchdStep = {HomeMchd: {text: "Вы на этапе создания доверенности для допуска к ветвям функционала данной системы, внимательно заполните поля аналогично полям в личных и учредительных документах"}};
        isHomePushed = false;
        currentMchdStep.step = nextStep;
    }


    async function goToLendMchd() {
        isLendPushed = true;
        const nextStep: MchdStep = {LendMchd: { text: "Выберите ранее созданный xml файл доверенности, отсоединенный фалй подписи руководителя организации и отправьте доверенность для регистрации в сервисе МЧД"}}
        isLendPushed = false;
        currentMchdStep.step = nextStep;
    }

    async function goToShowPowers() {
        isShowPowersPushed = true;
        try {
            const nextStep = await invoke<MchdStep>("cmd_show_powers", {});
            isShowPowersPushed = false;
            currentMchdStep.step = nextStep;;
        } catch (err) {
            const nextStep: MchdStep = {TryLater:{text: "Критическая ошибка на устройстве..."}};
            isShowPowersPushed = false;
            console.error("cmd_show_powers FAILED, err = ", err);
            currentMchdStep.step = nextStep;
        }
    }
</script>


<section class="w-40">
    <div class='w-v100'>
        <button
            type="button"
            class="but-gr"
            onclick={goToBTBMchd}
            disabled={isBTBPushed}
        >
            <span class="span-fill">
                Создать B2B МЧД
            </span>
        </button>
    </div>
</section>

<section class="w-40">
    <div class='w-v100'>
        <button
            type="button"
            class="but-gr"
            onclick={goToFnsMchd}
            disabled={isFnsPushed}
        >
            <span class="span-fill">
                Создать МЧД для отчетности
            </span>
        </button>
    </div>
</section>

<section class="w-40">
    <div class='w-v100'>
        <button
            type="button"
            class="but-gr"
            onclick={goToHomeMchd}
            disabled={isHomePushed}
        >
            <span class="span-fill">
                Создать МЧД для работы в системе
            </span>
        </button>
    </div>
</section>

<section class="w-40">
    <div class='w-v100'>
        <button
            type="button"
            class="but-gr"
            onclick={goToLendMchd}
            disabled={isLendPushed}

            >
            <span class='span-fill'>
                Отправить подписанный МЧД
            </span>
        </button>
    </div>
</section>

<section class="w-40">
    <div class='w-v100'>
        <button
            type="button"
            class="but-gr"
            disabled={isShowPowersPushed}
            onclick={goToShowPowers}
            >
            <span class='span-fill'>
                Посмотреть текущие полномочия
            </span>
        </button>
    </div>

</section>




    



