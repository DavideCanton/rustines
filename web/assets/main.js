import init, { start_wasm } from "./generated/rustines_wasm.js";
import { VERSION } from "./generated/version.js";

const romInput = document.getElementById("rom");
const btn = document.getElementById("start");
const loaded = document.getElementById("loaded");
const gById = document.getElementById.bind(document);

let romBytes;
let romName;

function show(element) {
    element.classList.remove("hidden");
}

function hide(element) {
    element.classList.add("hidden");
}

romInput.addEventListener("change", async () => {
    const file = romInput.files[0];
    romBytes = undefined;

    if (!file) {
        return;
    }

    romName = file.name;

    try {
        romBytes = new Uint8Array(await file.arrayBuffer());
        show(btn);
        loaded.textContent = `Loaded ${file.name}`;
        show(gById("zapper_container"));
    } catch (error) {
        loaded.textContent = `Could not read file: ${error}`;
    }
});

btn.addEventListener("click", async () => {
    let zapper = gById("zapper").checked;

    try {
        await init({
            module_or_path: "assets/generated/rustines_wasm_bg.wasm.gz",
        });
        start_wasm(romBytes, romName, zapper);
        hide(gById("header"));
        hide(gById("zapper_container"));
        hide(btn);
        show(gById("run_status"));
        gById("game_canvas").focus();
    } catch (error) {
        loaded.textContent = `Could not start ROM: ${error}`;
    }
});

gById("version").textContent = VERSION;
