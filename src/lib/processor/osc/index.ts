export async function createOscillator(
	audioCtx: AudioContext,
	frequency: number = 440
): Promise<AudioWorkletNode> {
	await audioCtx.audioWorklet.addModule('/src/lib/processor/osc/processor.js');

	const workletNode = new AudioWorkletNode(audioCtx, 'synth-processor');

    
    // wasm-bindgenで生成したwasmファイルをfetchで取得し、binaryとしてAudioWorkerProcessorのworker スレッドに投げる。
    // processorスレッドとは双方向通信でやりとりする。
	fetch('/src/wasm/synth/pkg/synth_bg.wasm')
		.then((response) => {
            console.log(response);
			return response.arrayBuffer();
		})
		.then((bytes) => {
			workletNode.port.postMessage({
				type: 'init',
				sampleRate: audioCtx.sampleRate,
				frequency,
				bytes: bytes
			});
		});
	workletNode.connect(audioCtx.destination);
	return workletNode;
}
