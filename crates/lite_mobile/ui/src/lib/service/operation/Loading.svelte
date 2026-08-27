<script lang='ts'>
    import {operStep} from '$lib/models/Operation/OperationManager.svelte';
    import type {OperationStep} from '$lib/models/rustModels/OperationStep';

    let isPushedStatementParser = $state(false);
    let isPushedManualInput = $state(false);
	let isPushedNewBankAcc = $state(false);

    function goToStatementParser () {
		if (isPushedStatementParser) {return;}
        isPushedStatementParser = true;
        const next_step: OperationStep = {StatementLoader: {text: "Выберите расчетный счет и загрузите банковскую выписку"}};
        isPushedStatementParser = false;
        operStep.step = next_step;
    }

    function goToManualInput() {
		if (isPushedManualInput) {return;}
        isPushedManualInput = true;
        const next_step: OperationStep = {ManualInput: {text: "Введите данные для проводок вручную"}};
        isPushedManualInput = false;
        operStep.step = next_step;
    }

	function goToNewBankAcc() {
		if (isPushedNewBankAcc) {return;}
		isPushedNewBankAcc = true;
		const next_step: OperationStep = {AccInput: {text: 'Введите БИК банка и номер расчетного счета'}};
		isPushedNewBankAcc = false;
		operStep.step = next_step;
	}
</script>

<section class='wide-button-section'>
    <div class='wide-button-group'>
        <button
            type='button'
            class='wide-button'
            onclick={goToStatementParser}
            disabled={isPushedStatementParser}
        >
            <span class='wide-button-span'>
                Загрзить проводки из банковской выписки
            </span>
        </button>
    </div>

    <div class='wide-button-group'>
        <button
            type='button'
            class='wide-button'
            onclick={goToManualInput}
            disabled={isPushedManualInput}
        >
            <span class='wide-button-span'>
                Создать проводки вручную
            </span>
        </button>
    </div>
</section>





