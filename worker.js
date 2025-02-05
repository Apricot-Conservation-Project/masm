import init, { render_map } from "./masm.js";

onmessage = () => {
    init().then(() => postMessage("ready"));
    onmessage = async function (e) {
        if (e.data == "init") {
            return;
        }
        console.log("working!");
        postMessage(render_map(e.data));
    };
};
