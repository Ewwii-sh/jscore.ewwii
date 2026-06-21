// === Command === //

const cmd = {
    async run(str) {
        await Deno.core.ops.op_run(str);
    },
    async run_read(str) {
        return await Deno.core.ops.op_run(str);
    }
};

// === File System === //
const fs = {
    async read(path) {},
    async write(path, content) {},
    async append(path, content) {},
    async remove(path) {},
    async exists(path) {},
    async mkdir(path) {},
    async readdir(path) {},
    async stat(path) {},
    async copy(src, dest) {},
    async move(src, dest) {},
}
