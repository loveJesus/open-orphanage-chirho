// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

export interface ChildProfileChirho {
  child_id_chirho: string;
  orphanage_id_chirho: string;
  given_name_chirho: string;
  date_of_birth_chirho: string;
  gender_chirho: string;
  medical_history_chirho: string;
  education_level_chirho: string;
  sponsorship_status_chirho: string;
  created_at_chirho: string;
  updated_at_chirho: string;
}

export interface ChildUpdateChirho {
  update_id_chirho: string;
  child_id_chirho: string;
  staff_user_id_chirho: string;
  update_text_chirho: string;
  date_posted_chirho: string;
  visibility_chirho: 'sponsors_only_chirho' | 'internal_chirho';
  created_at_chirho: string;
  updated_at_chirho: string;
} 