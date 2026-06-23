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


<div >
    <section class="input-section">
        <div class="file-path-group">
            <label class="input-group-label" for="xlsFile">
                Выберите папку для сохранения XLS файла МЧД доверенности
            </label>
            <div class="file-path">
                <input
                    id='xlsFile'
                    type="text"
                    value={xmlPathSaved}
                    class="file-path-input"
                />
                <button
                    type="button"
                    class="file-path-save"
                    onclick={save_xml_file}
                    disabled={xmlSavePushed}
                    >
                    Сохранить
                </button>
            </div>
        </div>

        <div class="file-path-group">
            <label class="input-group-label" for="xlsFile">
                Выберите папку для сохранения XLS файла МЧД доверенности
            </label>
            <div class="file-path">
                <input
                    id='xlsFile'
                    type="text"
                    value={docPathSaved}
                    class="file-path-input"
                />
                <button
                    type="button"
                    class="file-path-save"
                    onclick={save_doc_file}
                    disabled={docSavePushed}
                    >
                    Сохранить
                </button>
            </div>
        </div>
    </section>
</div>