/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],
	theme: {
		extend: {
			colors: {
				primary: '#1f2937'
			}
		}
	},
	plugins: [require('@tailwindcss/forms'), require('flowbite/plugin')],
	darkMode: 'class'
};
