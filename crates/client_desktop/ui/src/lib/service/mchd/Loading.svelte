<script lang='ts'>
    import {} from "$lib/models/Mchd/mchdManager.svelte";
    import type {MchdStep} from "$lib/models/rustModels/MchdStep";
    import {currentMchdStep} from "$lib/models/Mchd/mchdManager.svelte";

    let isTaxPushed = $state(false);
    let isHomePushed = $state(false);
    let isLendPushed = $state(false);
    let isShowPowersPushed = $state(false);

    async function goToTaxMchd() {
        isTaxPushed = true;
        const next_step: MchdStep = {TaxMchd: {text: "Заполните поэтапно поля для доверенности в ФНС в том же формате, в каком они указаны в документах. Предупреждение: Сервис ФНС РФ МЧД проверяет данные на соответствие, проверяйте корректность всех полей"}};
        isTaxPushed = false;
        currentMchdStep.add(next_step)
    }

    async function goToHomeMchd() {
        isHomePushed = true;
        const next_step: MchdStep = {HomeMchd: {text: "Заполните поэтапно поля для доверенности в системе. Предупреждение: Сервис ФНС РФ МЧД проверяет данные на соответствие, проверяйте корректность всех полей"}};
        isHomePushed = false;
        currentMchdStep.add(next_step)
    }

    async function goToLendMchd() {
        isLendPushed = true;
        const next_step: MchdStep = {LendMchd: { text: "Загрузите сформированный XML файл доверенности и файл подписи"}}
        isLendPushed = false;
        currentMchdStep.add(next_step)
    }
</script>


<section class="wide-button-section">
    <div class="wide-button-group">
        <button
            type="button"
            class="wide-button"
            onclick={goToTaxMchd}
            disabled={isTaxPushed}
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
            onclick={goToHomeMchd}
            disabled={isHomePushed}
        >
            <span class="wide-button-span">
                Создать МЧД
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
            >
            <span class='wide-button-span'>
                Посмотреть текущие полномочия
            </span>
        </button>
    </div>

</section>




    



