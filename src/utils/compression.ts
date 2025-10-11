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

  const globalBuffer = (globalThis as {
    Buffer?: { from(input: Uint8Array): { toString(encoding: string): string } };
  }).Buffer;
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
    Buffer?: { from(input: string, encoding: string): { [key: number]: number; length: number } };
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

function supportsCompressionStream(): boolean {
  return (
    typeof CompressionStream !== "undefined" &&
    typeof Blob !== "undefined" &&
    typeof Response !== "undefined"
  );
}

function supportsDecompressionStream(): boolean {
  return (
    typeof DecompressionStream !== "undefined" &&
    typeof Blob !== "undefined" &&
    typeof Response !== "undefined"
  );
}

export interface CompressionResult {
  data: string;
  compressed: boolean;
}

export async function compressString(value: string): Promise<CompressionResult> {
  if (!supportsCompressionStream()) {
    return { data: value, compressed: false };
  }

  const compressor = new CompressionStream("gzip");
  const sourceStream = new Blob([value]).stream();
  const compressedStream = sourceStream.pipeThrough(compressor);
  const compressedBuffer = await new Response(compressedStream).arrayBuffer();
  const encoded = encodeBase64(new Uint8Array(compressedBuffer));

  return { data: encoded, compressed: true };
}

export async function decompressString(value: string): Promise<string> {
  if (!value) {
    return "";
  }

  if (!supportsDecompressionStream()) {
    throw new Error("DecompressionStream is not supported in this environment");
  }

  const bytes = decodeBase64(value);
  const decompressor = new DecompressionStream("gzip");
  const sourceStream = new Blob([bytes]).stream();
  const decompressedStream = sourceStream.pipeThrough(decompressor);
  return new Response(decompressedStream).text();
}
