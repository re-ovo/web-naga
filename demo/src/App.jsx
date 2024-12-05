import {useState} from "react";
import {GlslBackendOptions, GlslFrontend, GlslVersion, GlslWriterFlags, ShaderStage, WgslFrontend} from "web-naga";

function App() {
    const [code, setCode] = useState('')
    const [result, setResult] = useState('')

    const glslToWgsl = () => {
        const front = GlslFrontend.new();
        const module = front.parse(code, ShaderStage.Vertex);
        setResult(module.to_wgsl())
    }

    const wgslToGlsl = () => {
        const front = WgslFrontend.new();
        const module = front.parse(code);
        const options = GlslBackendOptions.new();
        // options.stage = ShaderStage.Vertex;
        // options.version = GlslVersion.embedded(330)
        // options.flags = GlslWriterFlags.None
        setResult(module.to_glsl("main", options))
    }


    return (
        <div className="w-full container mx-auto flex flex-row gap-4">
            <textarea
                className="w-1/2 h-screen p-4 border-2 border-gray-300 rounded-md"
                value={code}
                onChange={(e) => setCode(e.target.value)}
            />
            <div className="flex flex-col gap-4">
                <button
                    className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                    onClick={glslToWgsl}
                >
                    Convert GLSL to WGSL
                </button>

                <button
                    className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                    onClick={wgslToGlsl}
                >
                    Convert WGSL to GLSL
                </button>
            </div>
            <div className="w-1/2 h-screen p-4 border-2 border-gray-300 rounded-md">
                {result}
            </div>
        </div>
    )
}

export default App
