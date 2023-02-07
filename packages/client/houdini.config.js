/// <references types="houdini-svelte">

/** @type {import('houdini').ConfigFile} */
const config = {
    "apiUrl": "http://127.0.0.1:3000/graphql",
    "plugins": {
        "houdini-svelte": {}
    },
    scalars: {
        UUID: {
            type: "string",
        }
    }
}

export default config
