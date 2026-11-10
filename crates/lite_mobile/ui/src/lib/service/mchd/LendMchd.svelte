<script lang='ts'>
    import {open as openFileDialog} from "@tauri-apps/plugin-dialog";
    import {invoke} from "@tauri-apps/api/core";
	import type { MchdStep } from "$lib/models/rustModels/MchdStep";
	import { currentMchdStep } from "$lib/models/Mchd/mchdManager.svelte";

    let isLoadXmlFilePushed = $state(false);
    let isLoadSigFilePushed = $state(false);
    let isLendMchdPushed = $state(false);

    let xmlFilePath = $state("");
    let sigFilePath = $state("");

    let isDataReady = $derived(xmlFilePath.length == 0 || sigFilePath.length == 0);

    async function getXmlFilePath() {
		if (isLoadXmlFilePushed) {return;}
		isLoadXmlFilePushed = true;
        try {
            const selected = await openFileDialog({
                multiple: false,
                directory: false,
                title: "Выберите XML файл доверенности",
                filters: [{name: 'документ xml', extensions: ['xml']}]
            });

            if (selected && typeof selected === 'string') {
                xmlFilePath = selected
            }
			isLoadXmlFilePushed = false;
        } catch (err) {
            const nextStep: MchdStep = {
				TryLater: {
					text: "Критическая ошибка на устройстве..."
				}
			};

			isLoadXmlFilePushed = false;
			currentMchdStep.step = nextStep;
        }
    }

    async function getSigFilePath() {
		if (isLoadSigFilePushed) { return;}
        try {
			const selected = await openFileDialog({
				multiple: false,
				directory: false,
				title: "Выберите отсоединенный файл подписи"
			});

			if (selected && typeof selected === 'string') {
				sigFilePath = selected;
			}
			isLoadSigFilePushed = false;
		} catch (err) {
			console.error("GET FILE PATH FAILED, ERROR = ", err);

			const nextStep: MchdStep = {
				TryLater: {
					text: "Критическая ошибка на устройстве..."
				}
			};

			isLoadSigFilePushed = false;
			currentMchdStep.step = nextStep;
		}
    }

    async function LendMchd() {
        if (isDataReady) { return }
        const data = {
            xmlFilePath: xmlFilePath,
            sigFilePath: sigFilePath
        }
        try {
            isLendMchdPushed = true;
            let nextStep: MchdStep = await invoke<MchdStep> ("cmd_lend_mchd", data);
            isLendMchdPushed = false;
            currentMchdStep.step = nextStep;;
        } catch (err) {
            console.error("Ошибка при отправке данных в cmd_lend_mchd, err = ", err);
            isLendMchdPushed = false;
            let nextStep: MchdStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            currentMchdStep.step = nextStep;;
        }
    }

</script>

<div class="w-100">
    <div class='w-v100'>
        <label class="label-close" for="xmlFileInput">
            Загрузите путь до XML файла доверенности
        </label>
        <input
            type="text"
            id="xmlFileInput"
            value={xmlFilePath}
            class="input-gr"
        />
        <button
            type="button"
            id="xmlFileButton"
            class="but-gr"
            onclick={getXmlFilePath}
            disabled={isLoadXmlFilePushed}
            >
            Загрузите xml файл
        </button>

    </div>
</div>

<div class="w-100">
    <div class='w-v100'>
        <label class="label-close" for="sigFileInput">
            Загрузите путь до файла ЭЦП
        </label>
  
        <input
            type="text"
            id="sigFileInput"
            value={sigFilePath}
            class="input-gr"
        />
        <button
            type="button"
            id="sigFileButton"
            class="but-gr"
            onclick={getSigFilePath}
            disabled={isLoadSigFilePushed}
            >
            Загрузите файл подписи
        </button>

    </div>
</div>

<div class="w-100">
    <div class='w-v100'>
		<button
			type="button"
			id='lendMchdButton'
			class='but-bl'
			disabled={isLendMchdPushed || isDataReady}
			onclick={LendMchd}
			>
				{#if isLendMchdPushed}
					<span class='loading-content'>
						<span class='loading-spinner'></span>
					</span>
				{:else}
					Отправить файлы на регистрацию
				{/if}
		</button>
	</div>
</div>