"use strict";
window.loading = true;
window.rendering = false;


function loaded(buf) {
    let pic = document.getElementById("picture");
    pic.src = buf.data;
    console.log("render done");
    window.rendering = false;
    document.getElementById("spin").setAttribute("repeatCount", "0");
    setTimeout(() => {
        document.getElementById("load").style.fill = "#dadee1";
    }, 1000);
    if (window.zooming) return;
    window.zooming = true;
    let panzoom = Panzoom(pic, {
        maxScale: 7,
        cursor: "grab",
        noBind: true,
    });
    panzoom.pan(pic.width / 2, pic.height / 2, { animate: true });
    panzoom.zoom(1, { animate: true });
    pic.addEventListener("pointerdown", (event) => {
        pic.style.cursor = "grabbing";
        panzoom.handleDown(event);
    });
    pic.addEventListener("pointerup", (event) => {
        pic.style.cursor = "grab";
        panzoom.handleUp(event);
    });
    pic.addEventListener("pointermove", panzoom.handleMove);
    pic.addEventListener("wheel", panzoom.zoomWithWheel);
}

document.querySelector("#input").addEventListener("change", (e) => {
    window.rendering = true;
    document.getElementById("spin").setAttribute("repeatCount", "indefinite")
    document.getElementById("spin").beginElement()
    document.getElementById("load").style.fill = "#bf92f9";

    var reader = new FileReader();
    reader.onload = () => load(reader.result);
    reader.readAsArrayBuffer(e.target.files[0]);
});
const worker = new Worker("./worker.js", { type: "module" });
worker.onmessage = (x) => {
    if (x.data == "ready") {
        window.loading = false;
        document.querySelector("input").disabled = false;
        document.getElementById("load").style.fill = "#dadee1";
        worker.onmessage = loaded;
    }
};
worker.postMessage("start");

window.load = (buf) => {
    console.log("sending...");
    worker.onerror = console.log;
    console.debug("sent to worker");
    worker.postMessage(buf);
};
