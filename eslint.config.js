import globals from "globals";

export default [
    {
        languageOptions: {
            ecmaVersion: 2024,
            sourceType: "module",
            globals: {
                "Deno": "readonly",
                ...globals.browser,
                ...globals.node
            },
        },
        rules: {
            "no-undef": "error",
            "no-unused-vars": "warn",
            "no-console": "off"
        }
    }
];
