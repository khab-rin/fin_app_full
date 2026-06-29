<script lang='ts'>
    import { save } from "@tauri-apps/plugin-dialog";
    import { writeFile } from "@tauri-apps/plugin-fs";

    import type { MchdStep } from "$lib/models/rustModels/MchdStep";
    import {MchdStepType} from "$lib/models/Mchd/MchdValues";
    import {currentMchdStep} from "$lib/models/Mchd/mchdManager.svelte";

    let xmlName: string = MchdStepType.TaxMchdSuccess in currentMchdStep.step ? 
        currentMchdStep.step.TaxMchdSuccess.xml_name : "";

    let xmlFile: Array<number> = MchdStepType.TaxMchdSuccess in currentMchdStep.step ? 
        currentMchdStep.step.TaxMchdSuccess.xml_file : [];

    const xmlFileBytes: Uint8Array = new Uint8Array(xmlFile);

    let doc_name: string = MchdStepType.TaxMchdSuccess in currentMchdStep.step ? 
        currentMchdStep.step.TaxMchdSuccess.doc_name : "";

    let doc_file: Array<number> = MchdStepType.TaxMchdSuccess in currentMchdStep.step ? 
        currentMchdStep.step.TaxMchdSuccess.doc_file : [];

    const docFileBytes: Uint8Array = new Uint8Array(doc_file);


    let xmlPathSaved = $state("");
    let docPathSaved = $state("");

    let xmlSavePushed = $state(false)
    let docSavePushed = $state(false)

    async function save_xml_file() {
        try {
            xmlSavePushed = true;
            const docPath = await save({
                title: "Сохранить файл",
                defaultPath: xmlName,
                filters: [{name: "Документы xml", extensions: ["xml"]}]
            });

            if (docPath) {
                await writeFile(docPath, xmlFileBytes);

                console.log("Документ успешно сохранен по пути:", docPath);

                xmlPathSaved = docPath;
                xmlSavePushed = false;

            } else {
                xmlSavePushed = false;
            }
        } catch(err) {
            console.error("XML FILE SAVING ERROR: ", err);
            xmlSavePushed = false;
            const next_step: MchdStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            currentMchdStep.add(next_step);
        }
    }

    async function save_doc_file() {
        try {
            docSavePushed = true;
            const docPath = await save({
                title: "Сохранить файл",
                defaultPath: doc_name,
                filters: [{name: "Документы doc", extensions: ["doc"]}]
            });

            if (docPath) {
                await writeFile(docPath, docFileBytes);

                console.log("Документ успешно сохранен по пути:", docPath);

                docPathSaved = docPath;
                docSavePushed = false;

            } else {
                docSavePushed = false;
            }
        } catch(err) {
            console.error("DOC FILE SAVING ERROR: ", err);
            docSavePushed = false;
            const next_step: MchdStep = {TryLater: {text: "Критическая ошибка на устройстве..."}};
            currentMchdStep.add(next_step);
        }
    }
</script>



<section class="input-section">
    <div class="input-wide-button-grid">
        <label class="input-wide-button-grid-label" for="xlsFile">
            Сохранить XML файл
        </label>
  
        <input
            id="xlsFile"
            type="text"
            value={xmlPathSaved}
            class="input-field"
        />
        <button
            type="button"
            class="wide-button"
            onclick={save_xml_file}
            disabled={xmlSavePushed}
            >
            Сохранить
        </button>

    </div>

    <div class="input-wide-button-grid">
        <label class="input-wide-button-grid-label" for="xlsFile">
            Сохранить DOC файл
        </label>
  
        <input
            id="xlsFile"
            type="text"
            value={docPathSaved}
            class="input-field"
        />
        <button
            type="button"
            class="wide-button"
            onclick={save_doc_file}
            disabled={docSavePushed}
            >
            Сохранить
        </button>
    </div>
</section>

<div class="main-button-group">
    <button
        type="button"
        class="main-button"
        id="SendMchd"
        disabled={xmlPathSaved.length == 0}
        >
        <span class="main-button-span">
            Отправить МЧД на регистрацию
        </span>
    </button>

</div>
