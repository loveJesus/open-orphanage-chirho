import { describe, it, expect } from 'vitest';
import { Miniflare } from 'miniflare';
import {mkdtempSync} from "fs";
import {join} from "path";
import os from "os";

describe('Simple Test Endpoint', () => {
  it('should return success message', async () => {
    const tempDir_chirho = mkdtempSync(join(os.tmpdir(), 'd1-test-chirho-'));

    const mf_chirho = new Miniflare({
      modules: true,
      modulesRules: [
        { type: "CompiledWasm", include: ["**/*.wasm"], fallthrough: true }
      ],
      scriptPath: 'build/worker/shim.mjs',
      bindings: {
        ENVIRONMENT_CHIRHO: 'test',
      },
      d1Databases: ['DB_CHIRHO'],
      d1Persist: tempDir_chirho,
    });

    const response_chirho = await mf_chirho.dispatchFetch('http://localhost/api_chirho/v1_chirho/test_chirho');
    const data_chirho = await response_chirho.json();

    expect(response_chirho.status).toBe(200);
    expect(data_chirho).toEqual({
      message: 'Hallelujah, Simple test endpoint working',
      status: 'success'
    });
  });
}); 