import { byte_repr } from '../pkg/spt';


export function byteRepr(byte, reverse = false) {
	return byte_repr(byte, reverse);
}

window.onload = function() {
	let input_field = document.getElementById("byte-input");
	let output = document.getElementById("output");

	let update = () => {
		output.innerText = byteRepr(Number(input_field.value));
	};

	let btn = document.getElementById("convert-button");

	btn.addEventListener("mouseup", update);
}