const wasm = import("./node_modules/invoker-game-wasm/invoker_game_wasm");

wasm.then(wasm => {
    window.addEventListener("keyup", (event) => {
        wasm.update(event.keyCode);
        console.log(wasm.fetch_status());
    });
});
