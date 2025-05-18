// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

import { describe, it, expect, beforeAll } from 'vitest';
import { Miniflare } from 'miniflare';

interface ChildProfileChirho {
  child_id_chirho: string;
  given_name_chirho: string;
  date_of_birth_chirho: string;
  gender_chirho: string;
  education_level_chirho: string;
  sponsorship_status_chirho: string;
  created_at_chirho: string;
  updated_at_chirho: string;
}

interface ErrorResponseChirho {
  error: string;
}

describe('Child Profile View Tests', () => {
  let mf: Miniflare;
  const baseUrl = 'http://localhost:8787';

  beforeAll(async () => {
    mf = new Miniflare({
      scriptPath: 'build/worker/shim.mjs',
      modules: true,
      bindings: {
        DB_CHIRHO: 'DB_CHIRHO',
      },
      kvNamespaces: ['DB_CHIRHO'],
      modulesRules: [
        {
          type: 'ESModule',
          include: ['build/worker/shim.mjs'],
        },
        {
          type: 'CompiledWasm',
          include: ['build/worker/index.wasm'],
        },
      ],
    });
  });

  it('should successfully view a child profile', async () => {
    const childId = 'chd_1_chirho';
    const response = await mf.dispatchFetch(`${baseUrl}/api_chirho/v1_chirho/children_chirho/${childId}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    expect(response.status).toBe(200);
    const data = await response.json() as ChildProfileChirho;
    
    expect(data.child_id_chirho).toBe(childId);
    expect(data.given_name_chirho).toBe('John Doe');
    expect(data.gender_chirho).toBe('male_chirho');
    expect(data.sponsorship_status_chirho).toBe('available_chirho');
    expect(data.education_level_chirho).toBe('Primary School');
    
    // Verify date fields are properly formatted
    expect(new Date(data.date_of_birth_chirho).toISOString()).toBe('2015-05-15T00:00:00.000Z');
    expect(new Date(data.created_at_chirho).toISOString()).toBeTruthy();
    expect(new Date(data.updated_at_chirho).toISOString()).toBeTruthy();
  });

  it('should return 404 for non-existent child profile', async () => {
    const nonExistentId = 'non_existent_chirho';
    const response = await mf.dispatchFetch(`${baseUrl}/api_chirho/v1_chirho/children_chirho/${nonExistentId}`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    });

    expect(response.status).toBe(404);
    const data = await response.json() as ErrorResponseChirho;
    expect(data.error).toBe('Child profile not found');
  });
}); 