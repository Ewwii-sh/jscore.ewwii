export default [
    {
        languageOptions: {
            ecmaVersion: 2024,
            sourceType: "module",
            globals: {
                "Deno": "readonly",
            },
        },
        rules: {
            "no-undef": "error",
            "no-unused-vars": "warn",
            "no-console": "off"
        }
    }
];
