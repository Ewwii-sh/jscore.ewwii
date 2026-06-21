class TextEncoder {
    encode(string) {
        const bytes = new Uint8Array(string.length);

        for (let i = 0; i < string.length; i++) {
            const code = string.charCodeAt(i);
            bytes[i] = code;
        }

        return bytes;
    }

    encodeInto(string, buffer) {
        let read = 0;
        let written = 0;

        while (read < string.length && written < buffer.length) {
            const code = string.charCodeAt(read);
            buffer[written] = code;

            read++;
            written++;
        }

        return {
            read: read,
            written: written
        };
    }
}

class TextDecoder {
    decode(buffer) {
        let string = '';

        for (let i = 0; i < buffer.length; i++) {
            const byte = buffer[i];
            const char = String.fromCharCode(byte);

            string += char;
        }

        return string;
    }
}

console.log(Object.getOwnPropertyNames(TextEncoder.prototype));

const encoder = new TextEncoder();
const decoder = new TextDecoder();

const encoding = encoder.encode("Hello, World!");
const decoding = decoder.decode(encoding);

console.log(encoding);
console.log(decoding);

