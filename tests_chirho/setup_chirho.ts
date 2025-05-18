// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

import { Miniflare } from 'miniflare';
import { beforeAll, afterAll } from 'vitest';
import { readFileSync, mkdtempSync } from 'fs';
import { join } from 'path';
import os from 'os';

export interface TestContextChirho {
  mf_chirho: Miniflare;
  env_chirho: any;
  tempDir_chirho: string;
  baseUrl_chirho: string;
}

export async function setupTestEnvChirho(): Promise<TestContextChirho> {
  // Create a unique temp directory for this test context
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

  // Initialize the database with our schema
  const db_chirho = await mf_chirho.getD1Database('DB_CHIRHO');
  const schema_path_chirho = join(process.cwd(), 'migrations_chirho', '001_schema_chirho.sql');
  const schema_chirho = readFileSync(schema_path_chirho, 'utf-8');
  await db_chirho.batch(schema_chirho.split(';').filter(Boolean).map(q => db_chirho.prepare(q)));

  // Load test data
  const testdata_path_chirho = join(process.cwd(), 'migrations_chirho', '002_test_data_chirho.sql');
  const testdata_chirho = readFileSync(testdata_path_chirho, 'utf-8');
  await db_chirho.batch(testdata_chirho.split(';').filter(Boolean).map(q => db_chirho.prepare(q)));

  return {
    mf_chirho,
    env_chirho: await mf_chirho.getBindings(),
    tempDir_chirho,
    baseUrl_chirho: 'http://localhost'
  };
}

export async function teardownTestEnvChirho(context_chirho: TestContextChirho) {
  await context_chirho.mf_chirho.dispose();
  // Optionally, clean up the temp directory here if desired
} 