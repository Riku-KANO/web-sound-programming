import { Oscillator, initSync } from "../../../wasm/synth/pkg/synth";

class SynthProcessor extends AudioWorkletProcessor {
	static waveform = {
		SINE: 'sine',
		SQUARE: 'square',
		SAWTOOTH: 'sawtooth',
		TRIANGLE: 'triangle'
	};

	static MESSAGE = {
		INIT: 'init'
	};

	constructor() {
		super();
		this.port.onmessage = async (event) => this.onmessage(event);
	}


	/**
	 * @param {MessageEvent} event
	 */
	async onmessage(event) {
		switch (event.data.type) {
			case SynthProcessor.MESSAGE.INIT:
				console.log(event.data.bytes);
				const module = await WebAssembly.compile(event.data.bytes);
				console.log(module);
				const output = initSync({ module });
				console.log(output);
				this.osc = new Oscillator(event.data.sampleRate, event.data.frequency);
				console.log(this.osc);
				break;
			default:
				break;
		}
	}

	/**
	 * @param {Float32Array[][]} inputs FlozenArrayでもある
	 * @param {Float32Array[][]} outputs
	 * @param {Record<string, any>} params
	 * @returns {boolean}
	 */
	process(inputs, outputs, params) {
    if (!this.osc) {
      return true;
    }
		const output = outputs[0];
		for (let channel = 0; channel < output.length; channel++) {
			const outputChannel = output[channel];
			for (let i = 0; i < outputChannel.length; i++) {
				outputChannel[i] = this.osc.generate_samples();
			}
		}
		return true;
	}
}

registerProcessor("synth-processor", SynthProcessor);
