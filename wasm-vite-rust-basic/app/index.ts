// imports the freshly built wasm-pack dependency package
import { init, greet, alert_greet } from "../pkg";

const loadWASM = () => {

    alert_greet("Joel")
    // create dom initialization
    init()

    // say hello!
    greet()
}

loadWASM()
