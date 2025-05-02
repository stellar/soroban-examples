// Collect stdin.
export async function stdin(): Promise<string> {
  let stdin = "";
  const decoder = new TextDecoder();
  const reader = Deno.stdin.readable.getReader();
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      stdin += decoder.decode(value);
    }
  } finally {
    reader.releaseLock();
  }
  return stdin;
}

// Write to stderr, for human logs.
export function stderr(...args: string[]) {
  const s = args.join(" ") + "\n";
  const encoder = new TextEncoder();
  const data = encoder.encode(s);
  Deno.stderr.writeSync(data);
}

// Write to stdout, for outputting the transaction envelope.
export function stdout(...args: string[]) {
  const s = args.join(" ");
  const encoder = new TextEncoder();
  const data = encoder.encode(s);
  Deno.stdout.writeSync(data);
}
