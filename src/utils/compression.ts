const MAX_DICTIONARY_SIZE = 0xffff;

function createCompressionDictionary(): Map<string, number> {
  const dictionary = new Map<string, number>();
  for (let index = 0; index < 256; index += 1) {
    dictionary.set(String.fromCharCode(index), index);
  }
  return dictionary;
}

function lzwCompress(input: string): Uint16Array {
  if (input.length === 0) {
    return new Uint16Array(0);
  }

  let dictionary = createCompressionDictionary();
  let dictSize = 256;
  let phrase = input[0];
  const codes: number[] = [];

  for (let index = 1; index < input.length; index += 1) {
    const char = input[index];
    const combined = phrase + char;

    if (dictionary.has(combined)) {
      phrase = combined;
      continue;
    }

    const code = dictionary.get(phrase);
    codes.push(code ?? phrase.charCodeAt(0));

    if (dictSize < MAX_DICTIONARY_SIZE) {
      dictionary.set(combined, dictSize);
      dictSize += 1;
    } else {
      dictionary = createCompressionDictionary();
      dictSize = 256;
    }

    phrase = char;
  }

  const lastCode = dictionary.get(phrase);
  codes.push(lastCode ?? phrase.charCodeAt(0));

  return Uint16Array.from(codes);
}

function createDecompressionDictionary(): string[] {
  const dictionary: string[] = new Array(256);
  for (let index = 0; index < 256; index += 1) {
    dictionary[index] = String.fromCharCode(index);
  }
  return dictionary;
}

function lzwDecompress(codes: Uint16Array): string {
  if (codes.length === 0) {
    return "";
  }

  let dictionary = createDecompressionDictionary();
  let dictSize = 256;
  let phrase = dictionary[codes[0]] ?? String.fromCharCode(codes[0]);
  let result = phrase;

  for (let index = 1; index < codes.length; index += 1) {
    const code = codes[index];
    let entry = dictionary[code];

    if (entry === undefined) {
      if (code === dictSize) {
        entry = phrase + phrase[0];
      } else {
        entry = String.fromCharCode(code);
      }
    }

    result += entry;

    if (dictSize < MAX_DICTIONARY_SIZE) {
      dictionary[dictSize] = phrase + entry[0];
      dictSize += 1;
    } else {
      dictionary = createDecompressionDictionary();
      dictSize = 256;
    }

    phrase = entry;
  }

  return result;
}

function encodeBase64(bytes: Uint8Array): string {
  if (bytes.length === 0) {
    return "";
  }

  if (typeof globalThis.btoa === "function") {
    let binary = "";
    for (let index = 0; index < bytes.length; index += 1) {
      binary += String.fromCharCode(bytes[index]);
    }
    return globalThis.btoa(binary);
  }

  const globalBuffer = (globalThis as { Buffer?: { from(input: Uint8Array): { toString(encoding: string): string } } }).Buffer;
  if (globalBuffer) {
    return globalBuffer.from(bytes).toString("base64");
  }

  throw new Error("No base64 encoder available in this environment");
}

function decodeBase64(value: string): Uint8Array {
  if (!value) {
    return new Uint8Array(0);
  }

  if (typeof globalThis.atob === "function") {
    const binary = globalThis.atob(value);
    const bytes = new Uint8Array(binary.length);
    for (let index = 0; index < binary.length; index += 1) {
      bytes[index] = binary.charCodeAt(index);
    }
    return bytes;
  }

  const globalBuffer = (globalThis as {
    Buffer?: {
      from(input: string, encoding: string): { [key: number]: number; length: number };
    };
  }).Buffer;
  if (globalBuffer) {
    const buffer = globalBuffer.from(value, "base64");
    const bytes = new Uint8Array(buffer.length);
    for (let index = 0; index < buffer.length; index += 1) {
      bytes[index] = buffer[index];
    }
    return bytes;
  }

  throw new Error("No base64 decoder available in this environment");
}

export function compressString(value: string): string {
  if (!value) {
    return "";
  }

  const codes = lzwCompress(value);
  const bytes = new Uint8Array(codes.length * 2);
  const view = new DataView(bytes.buffer);

  for (let index = 0; index < codes.length; index += 1) {
    view.setUint16(index * 2, codes[index], true);
  }

  return encodeBase64(bytes);
}

export function decompressString(value: string): string {
  if (!value) {
    return "";
  }

  const bytes = decodeBase64(value);
  if (bytes.length === 0) {
    return "";
  }

  if (bytes.length % 2 !== 0) {
    throw new Error("Corrupted compressed payload");
  }

  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const codes = new Uint16Array(bytes.length / 2);

  for (let index = 0; index < codes.length; index += 1) {
    codes[index] = view.getUint16(index * 2, true);
  }

  return lzwDecompress(codes);
}
