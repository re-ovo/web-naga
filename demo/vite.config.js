import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react-swc'
import Unocss from 'unocss/vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";


// https://vite.dev/config/
export default defineConfig({
    plugins: [
        react(),
        Unocss(),
        wasm(),
        topLevelAwait()
    ],
})
