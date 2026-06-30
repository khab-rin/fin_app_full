<script lang='ts'>
    import {open as openFileDialog} from "@tauri-apps/plugin-dialog";
    import {invoke} from "@tauri-apps/api/core";
	import type { MchdStep } from "$lib/models/rustModels/MchdStep";
	import { currentMchdStep } from "$lib/models/Mchd/mchdManager.svelte";

    let isLoadXmlFilePushed = $state(false);
    let isLoadSigFilePushed = $state(false);
    let isLendMchdButtonPushed = $state(false);

    let xmlFilePath = $state("");
    let sigFilePath = $state("");

    let isDataReady = $derived(xmlFilePath.length != 0 && sigFilePath.length != 0);

    async function getXmlFilePath() {
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
        } catch (err) {
            console.error("Ошибка при выборе файла доверенности:", err);
        }
    }

    async function getSigFilePath() {
        try {
            const selected = await openFileDialog({
                multiple: false,
                directory: false,
                title: "Выберите файл доверенности подписи",
                filters: [{name: 'документ подписи', extensions: ['sig', 'p7s']}]
            });

            if (selected && typeof selected === 'string') {
                sigFilePath = selected
            }
        } catch (err) {
            console.error("Ошибка при выборе файла доверенности:", err);
        }
    }

    async function LendMchd() {
        if (isDataReady) { return }
        const data = {
            xmlFilePath: xmlFilePath,
            sigFilePath: sigFilePath
        }
        try {
            isLendMchdButtonPushed = true;
            let next_step: MchdStep = await invoke<MchdStep> ("cmd_lend_mchd", data);
            isLendMchdButtonPushed = false;
            currentMchdStep.add(next_step);
        } catch (err) {
            console.error("Ошибка при отправке данных в cmd_lend_mchd, err = ", err);
            isLendMchdButtonPushed = false;
            let next_step: MchdStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            currentMchdStep.add(next_step);
        }
        

    }

</script>

<section class="input-section">
    <div class="input-wide-button-grid">
        <label class="input-wide-button-grid-label" for="xmlFileInput">
            Загрузите путь до XML файла доверенности
        </label>
  
        <input
            type="text"
            id="xmlFileInput"
            value={xmlFilePath}
            class="input-field"
        />
        <button
            type="button"
            id="xmlFileButton"
            class="wide-button"
            onclick={getXmlFilePath}
            disabled={isLoadXmlFilePushed}
            >
            Сохранить
        </button>

    </div>

    <div class="input-wide-button-grid">
        <label class="input-wide-button-grid-label" for="sigFileInput">
            Загрузите путь до файла ЭЦП
        </label>
  
        <input
            type="text"
            id="sigFileInput"
            value={sigFilePath}
            class="input-field"
        />
        <button
            type="button"
            id="sigFileButton"
            class="wide-button"
            onclick={getSigFilePath}
            disabled={isLoadSigFilePushed}
            >
            Сохранить
        </button>

    </div>
</section>

<div class="main-button-group">
    <button
        type="button"
        id='lendMchdButton'
        class='main-button'
        disabled={isLendMchdButtonPushed || isDataReady}
        onclick={LendMchd}

        >
        <span class='main-button-span'>Отправить файлы на регистрацию</span>
    </button>


</div>