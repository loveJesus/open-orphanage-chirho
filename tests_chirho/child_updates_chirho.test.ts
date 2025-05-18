// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { setupTestEnvChirho, teardownTestEnvChirho, TestContextChirho } from './setup_chirho';
import { ChildUpdateChirho } from './types_chirho';

describe('Child Updates API', () => {
  let context_chirho: TestContextChirho;

  beforeAll(async () => {
    context_chirho = await setupTestEnvChirho();
  });

  afterAll(async () => {
    await teardownTestEnvChirho(context_chirho);
  });

  it('should create a new child update', async () => {
    const child_id_chirho = crypto.randomUUID();
    const staff_user_id_chirho = crypto.randomUUID();
    const update_chirho: ChildUpdateChirho = {
      update_id_chirho: crypto.randomUUID(),
      child_id_chirho,
      staff_user_id_chirho,
      update_text_chirho: 'Completed primary school with excellent grades',
      date_posted_chirho: new Date().toISOString(),
      visibility_chirho: 'sponsors_only_chirho',
      created_at_chirho: new Date().toISOString(),
      updated_at_chirho: new Date().toISOString()
    };

    const response_chirho = await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}/updates_chirho`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer test_token_chirho'
      },
      body: JSON.stringify(update_chirho)
    });

    expect(response_chirho.status).toBe(201);
    const result_chirho = await response_chirho.json() as ChildUpdateChirho;
    expect(result_chirho.update_id_chirho).toBe(update_chirho.update_id_chirho);
  });

  it('should retrieve child updates', async () => {
    const child_id_chirho = crypto.randomUUID();
    const staff_user_id_chirho = crypto.randomUUID();
    const update_chirho: ChildUpdateChirho = {
      update_id_chirho: crypto.randomUUID(),
      child_id_chirho,
      staff_user_id_chirho,
      update_text_chirho: 'Completed primary school with excellent grades',
      date_posted_chirho: new Date().toISOString(),
      visibility_chirho: 'sponsors_only_chirho',
      created_at_chirho: new Date().toISOString(),
      updated_at_chirho: new Date().toISOString()
    };

    // First create an update
    await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}/updates_chirho`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer test_token_chirho'
      },
      body: JSON.stringify(update_chirho)
    });

    // Then retrieve updates
    const response_chirho = await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}/updates_chirho`, {
      headers: {
        'Authorization': 'Bearer test_token_chirho'
      }
    });

    expect(response_chirho.status).toBe(200);
    const result_chirho = await response_chirho.json() as ChildUpdateChirho[];
    expect(result_chirho.length).toBeGreaterThan(0);
    expect(result_chirho[0].child_id_chirho).toBe(child_id_chirho);
  });

  it('should update a child update', async () => {
    const child_id_chirho = crypto.randomUUID();
    const staff_user_id_chirho = crypto.randomUUID();
    const update_id_chirho = crypto.randomUUID();
    const update_chirho: ChildUpdateChirho = {
      update_id_chirho,
      child_id_chirho,
      staff_user_id_chirho,
      update_text_chirho: 'Completed primary school with excellent grades',
      date_posted_chirho: new Date().toISOString(),
      visibility_chirho: 'sponsors_only_chirho',
      created_at_chirho: new Date().toISOString(),
      updated_at_chirho: new Date().toISOString()
    };

    // First create an update
    await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}/updates_chirho`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer test_token_chirho'
      },
      body: JSON.stringify(update_chirho)
    });

    // Then update it
    const updated_text_chirho = 'Updated: Completed primary school with outstanding grades';
    const response_chirho = await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}/updates_chirho/${update_id_chirho}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer test_token_chirho'
      },
      body: JSON.stringify({
        ...update_chirho,
        update_text_chirho: updated_text_chirho,
        updated_at_chirho: new Date().toISOString()
      })
    });

    expect(response_chirho.status).toBe(200);
    const result_chirho = await response_chirho.json() as ChildUpdateChirho;
    expect(result_chirho.update_text_chirho).toBe(updated_text_chirho);
  });

  it('should delete a child update', async () => {
    const child_id_chirho = crypto.randomUUID();
    const staff_user_id_chirho = crypto.randomUUID();
    const update_id_chirho = crypto.randomUUID();
    const update_chirho: ChildUpdateChirho = {
      update_id_chirho,
      child_id_chirho,
      staff_user_id_chirho,
      update_text_chirho: 'Completed primary school with excellent grades',
      date_posted_chirho: new Date().toISOString(),
      visibility_chirho: 'sponsors_only_chirho',
      created_at_chirho: new Date().toISOString(),
      updated_at_chirho: new Date().toISOString()
    };

    // First create an update
    await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}/updates_chirho`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer test_token_chirho'
      },
      body: JSON.stringify(update_chirho)
    });

    // Then delete it
    const response_chirho = await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}/updates_chirho/${update_id_chirho}`, {
      method: 'DELETE',
      headers: {
        'Authorization': 'Bearer test_token_chirho'
      }
    });

    expect(response_chirho.status).toBe(200);

    // Verify it's deleted
    const get_response_chirho = await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}/updates_chirho/${update_id_chirho}`, {
      headers: {
        'Authorization': 'Bearer test_token_chirho'
      }
    });

    expect(get_response_chirho.status).toBe(404);
  });
}); 