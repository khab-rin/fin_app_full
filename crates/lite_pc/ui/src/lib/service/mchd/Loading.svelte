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


<section class="group-one">
    <div>
        <button
            type="button"
            class="green-button"
            onclick={goToBTBMchd}
            disabled={isBTBPushed}
        >
            <span class="green-button-span">
                Создать B2B МЧД
            </span>
        </button>
    </div>

    <div>
        <button
            type="button"
            class="green-button"
            onclick={goToFnsMchd}
            disabled={isFnsPushed}
        >
            <span class="green-button-span">
                Создать МЧД для отчетности
            </span>
        </button>
    </div>

    <div>
        <button
            type="button"
            class="green-button"
            onclick={goToHomeMchd}
            disabled={isHomePushed}
        >
            <span class="green-button-span">
                Создать МЧД для работы в системе
            </span>
        </button>
    </div>


    <div>
        <button
            type="button"
            class="green-button"
            onclick={goToLendMchd}
            disabled={isLendPushed}

            >
            <span class='green-button-span'>
                Отправить подписанный МЧД
            </span>
        </button>
    </div>

    <div>
        <button
            type="button"
            class="green-button"
            disabled={isShowPowersPushed}
            onclick={goToShowPowers}
            >
            <span class='green-button-span'>
                Посмотреть текущие полномочия
            </span>
        </button>
    </div>

</section>




    



