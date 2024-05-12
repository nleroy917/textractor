import { extract_text } from "textractor-js";

const file = new File(["Hello, World!"], "hello.txt", { type: "text/plain" })
const reader = new FileReader()

reader.onload = (e) => {
    if (!e.target) {
        console.log("No target")
        return
    }
    const arrayBuffer = e.target.result as ArrayBuffer
    const uint8Array = new Uint8Array(arrayBuffer)
    const text = extract_text(uint8Array)
    console.log(text)
}

reader.readAsArrayBuffer(file)