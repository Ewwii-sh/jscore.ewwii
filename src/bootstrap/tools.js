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
    async read(path) {
        return await Deno.core.ops.op_fs_read(path);
    },
    async write(path, content) {
        await Deno.core.ops.op_fs_write(path, content, false);
    },
    async append(path, content) {
        await Deno.core.ops.op_fs_write(path, content, true);
    },
    async remove(path) {
        await Deno.core.ops.op_fs_remove(path);
    },
    async exists(path) {
        return await Deno.core.ops.op_fs_exists(path);
    },
    async mkdir(path) {
        await Deno.core.ops.op_fs_mkdir(path);
    },
    async readdir(path) {
        const json = await Deno.core.ops.op_fs_readdir(path);
        return JSON.parse(json);
    },
    async stat(path) {
        const json = await Deno.core.ops.op_fs_stat(path);
        return JSON.parse(json);
    },
    async copy(src, dest) {
        await Deno.core.ops.op_fs_copy(src, dest);
    },
    async move(src, dest) {
        await Deno.core.ops.op_fs_move(src, dest);
    },
}

export { cmd, fs }
