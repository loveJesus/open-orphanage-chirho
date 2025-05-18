// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { setupTestEnvChirho, teardownTestEnvChirho, TestContextChirho } from './setup_chirho';
import { ChildProfileChirho } from './types_chirho';

describe('Child Profile API', () => {
  let context_chirho: TestContextChirho;

  beforeAll(async () => {
    context_chirho = await setupTestEnvChirho();
  });

  afterAll(async () => {
    await teardownTestEnvChirho(context_chirho);
  });

  it('should create a new child profile', async () => {
    const child_profile_chirho: ChildProfileChirho = {
      child_id_chirho: crypto.randomUUID(),
      orphanage_id_chirho: crypto.randomUUID(),
      given_name_chirho: 'Test Child',
      date_of_birth_chirho: '2010-01-01T00:00:00Z',
      gender_chirho: 'male_chirho',
      medical_history_chirho: 'No known conditions',
      education_level_chirho: 'Primary',
      sponsorship_status_chirho: 'available_chirho',
      created_at_chirho: new Date().toISOString(),
      updated_at_chirho: new Date().toISOString()
    };

    const response_chirho = await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer test_token_chirho'
      },
      body: JSON.stringify(child_profile_chirho)
    });

    expect(response_chirho.status).toBe(201);
    const result_chirho = await response_chirho.json() as ChildProfileChirho;
    expect(result_chirho.child_id_chirho).toBe(child_profile_chirho.child_id_chirho);
  });

  it('should retrieve a child profile', async () => {
    const child_id_chirho = crypto.randomUUID();
    const child_profile_chirho: ChildProfileChirho = {
      child_id_chirho,
      orphanage_id_chirho: crypto.randomUUID(),
      given_name_chirho: 'Test Child',
      date_of_birth_chirho: '2010-01-01T00:00:00Z',
      gender_chirho: 'male_chirho',
      medical_history_chirho: 'No known conditions',
      education_level_chirho: 'Primary',
      sponsorship_status_chirho: 'available_chirho',
      created_at_chirho: new Date().toISOString(),
      updated_at_chirho: new Date().toISOString()
    };

    // First create the child profile
    await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer test_token_chirho'
      },
      body: JSON.stringify(child_profile_chirho)
    });

    // Then retrieve it
    const response_chirho = await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}`, {
      headers: {
        'Authorization': 'Bearer test_token_chirho'
      }
    });

    expect(response_chirho.status).toBe(200);
    const result_chirho = await response_chirho.json() as ChildProfileChirho;
    expect(result_chirho.child_id_chirho).toBe(child_id_chirho);
    expect(result_chirho.given_name_chirho).toBe(child_profile_chirho.given_name_chirho);
  });

  it('should update a child profile', async () => {
    const child_id_chirho = crypto.randomUUID();
    const child_profile_chirho: ChildProfileChirho = {
      child_id_chirho,
      orphanage_id_chirho: crypto.randomUUID(),
      given_name_chirho: 'Test Child',
      date_of_birth_chirho: '2010-01-01T00:00:00Z',
      gender_chirho: 'male_chirho',
      medical_history_chirho: 'No known conditions',
      education_level_chirho: 'Primary',
      sponsorship_status_chirho: 'available_chirho',
      created_at_chirho: new Date().toISOString(),
      updated_at_chirho: new Date().toISOString()
    };

    // First create the child profile
    await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer test_token_chirho'
      },
      body: JSON.stringify(child_profile_chirho)
    });

    // Then update it
    const update_chirho: ChildProfileChirho = {
      ...child_profile_chirho,
      given_name_chirho: 'Updated Child Name',
      updated_at_chirho: new Date().toISOString()
    };

    const response_chirho = await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer test_token_chirho'
      },
      body: JSON.stringify(update_chirho)
    });

    expect(response_chirho.status).toBe(200);
    const result_chirho = await response_chirho.json() as ChildProfileChirho;
    expect(result_chirho.given_name_chirho).toBe('Updated Child Name');
  });

  it('should delete a child profile', async () => {
    const child_id_chirho = crypto.randomUUID();
    const child_profile_chirho: ChildProfileChirho = {
      child_id_chirho,
      orphanage_id_chirho: crypto.randomUUID(),
      given_name_chirho: 'Test Child',
      date_of_birth_chirho: '2010-01-01T00:00:00Z',
      gender_chirho: 'male_chirho',
      medical_history_chirho: 'No known conditions',
      education_level_chirho: 'Primary',
      sponsorship_status_chirho: 'available_chirho',
      created_at_chirho: new Date().toISOString(),
      updated_at_chirho: new Date().toISOString()
    };

    // First create the child profile
    await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer test_token_chirho'
      },
      body: JSON.stringify(child_profile_chirho)
    });

    // Then delete it
    const response_chirho = await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}`, {
      method: 'DELETE',
      headers: {
        'Authorization': 'Bearer test_token_chirho'
      }
    });

    expect(response_chirho.status).toBe(200);

    // Verify it's deleted
    const get_response_chirho = await context_chirho.mf_chirho.dispatchFetch(`${context_chirho.baseUrl_chirho}/api_chirho/v1_chirho/children_chirho/${child_id_chirho}`, {
      headers: {
        'Authorization': 'Bearer test_token_chirho'
      }
    });

    expect(get_response_chirho.status).toBe(404);
  });
}); 