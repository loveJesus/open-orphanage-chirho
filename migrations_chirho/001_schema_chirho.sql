-- For God so loved the world, that he gave his only begotten Son,
-- that all who believe in Him should not perish but have everlasting life.
-- John 3:16 (KJV)

-- User Management Tables
CREATE TABLE IF NOT EXISTS staff_users_chirho (
    user_id_chirho TEXT PRIMARY KEY,
    orphanage_id_chirho TEXT NOT NULL,
    email_chirho TEXT UNIQUE NOT NULL,
    hashed_password_chirho TEXT NOT NULL,
    first_name_chirho TEXT NOT NULL,
    last_name_chirho TEXT NOT NULL,
    role_chirho TEXT NOT NULL CHECK (role_chirho IN ('admin_chirho', 'staff_chirho')),
    created_at_chirho TEXT NOT NULL,
    updated_at_chirho TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sponsor_users_chirho (
    user_id_chirho TEXT PRIMARY KEY,
    email_chirho TEXT UNIQUE NOT NULL,
    hashed_password_chirho TEXT NOT NULL,
    first_name_chirho TEXT NOT NULL,
    last_name_chirho TEXT NOT NULL,
    created_at_chirho TEXT NOT NULL,
    updated_at_chirho TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS platform_admins_chirho (
    user_id_chirho TEXT PRIMARY KEY,
    email_chirho TEXT UNIQUE NOT NULL,
    hashed_password_chirho TEXT NOT NULL,
    first_name_chirho TEXT NOT NULL,
    last_name_chirho TEXT NOT NULL,
    created_at_chirho TEXT NOT NULL,
    updated_at_chirho TEXT NOT NULL
);

-- Orphanage Profile Tables
CREATE TABLE IF NOT EXISTS orphanage_profiles_chirho (
    orphanage_id_chirho TEXT PRIMARY KEY,
    name_chirho TEXT NOT NULL,
    description_chirho TEXT NOT NULL,
    address_chirho TEXT NOT NULL,
    contact_email_chirho TEXT NOT NULL,
    contact_phone_chirho TEXT,
    website_url_chirho TEXT,
    capacity_chirho INTEGER NOT NULL,
    current_children_count_chirho INTEGER NOT NULL DEFAULT 0,
    founded_date_chirho TEXT NOT NULL,
    registration_number_chirho TEXT NOT NULL,
    is_verified_chirho BOOLEAN NOT NULL DEFAULT FALSE,
    verification_date_chirho TEXT,
    verified_by_chirho TEXT,
    created_at_chirho TEXT NOT NULL,
    updated_at_chirho TEXT NOT NULL,
    FOREIGN KEY (verified_by_chirho) REFERENCES platform_admins_chirho(user_id_chirho)
);

-- Child Profile Tables
CREATE TABLE IF NOT EXISTS child_profiles_chirho (
    child_id_chirho TEXT PRIMARY KEY,
    orphanage_id_chirho TEXT NOT NULL,
    given_name_chirho TEXT NOT NULL,
    date_of_birth_chirho TEXT NOT NULL,
    gender_chirho TEXT NOT NULL CHECK (gender_chirho IN ('male_chirho', 'female_chirho', 'other_chirho')),
    medical_history_chirho TEXT,
    education_level_chirho TEXT NOT NULL,
    sponsorship_status_chirho TEXT NOT NULL CHECK (sponsorship_status_chirho IN ('available_chirho', 'partially_sponsored_chirho', 'fully_sponsored_chirho')),
    created_at_chirho TEXT NOT NULL,
    updated_at_chirho TEXT NOT NULL,
    FOREIGN KEY (orphanage_id_chirho) REFERENCES orphanage_profiles_chirho(orphanage_id_chirho)
);

CREATE TABLE IF NOT EXISTS child_updates_chirho (
    update_id_chirho TEXT PRIMARY KEY,
    child_id_chirho TEXT NOT NULL,
    staff_user_id_chirho TEXT NOT NULL,
    update_text_chirho TEXT NOT NULL,
    date_posted_chirho TEXT NOT NULL,
    visibility_chirho TEXT NOT NULL CHECK (visibility_chirho IN ('sponsors_only_chirho', 'internal_chirho')),
    created_at_chirho TEXT NOT NULL,
    updated_at_chirho TEXT NOT NULL,
    FOREIGN KEY (child_id_chirho) REFERENCES child_profiles_chirho(child_id_chirho),
    FOREIGN KEY (staff_user_id_chirho) REFERENCES staff_users_chirho(user_id_chirho)
);

-- Sponsorship Tables
CREATE TABLE IF NOT EXISTS sponsorships_chirho (
    sponsorship_id_chirho TEXT PRIMARY KEY,
    sponsor_id_chirho TEXT NOT NULL,
    child_id_chirho TEXT NOT NULL,
    monthly_amount_chirho REAL NOT NULL,
    start_date_chirho TEXT NOT NULL,
    end_date_chirho TEXT,
    status_chirho TEXT NOT NULL CHECK (status_chirho IN ('active_chirho', 'paused_chirho', 'ended_chirho')),
    created_at_chirho TEXT NOT NULL,
    updated_at_chirho TEXT NOT NULL,
    FOREIGN KEY (sponsor_id_chirho) REFERENCES sponsor_users_chirho(user_id_chirho),
    FOREIGN KEY (child_id_chirho) REFERENCES child_profiles_chirho(child_id_chirho)
);

-- Donation Tables
CREATE TABLE IF NOT EXISTS donations_chirho (
    donation_id_chirho TEXT PRIMARY KEY,
    donor_id_chirho TEXT,
    orphanage_id_chirho TEXT NOT NULL,
    amount_chirho REAL NOT NULL,
    status_chirho TEXT NOT NULL CHECK (status_chirho IN ('pending_chirho', 'completed_chirho', 'failed_chirho', 'refunded_chirho')),
    payment_method_chirho TEXT NOT NULL,
    transaction_id_chirho TEXT,
    created_at_chirho TEXT NOT NULL,
    updated_at_chirho TEXT NOT NULL,
    FOREIGN KEY (donor_id_chirho) REFERENCES sponsor_users_chirho(user_id_chirho),
    FOREIGN KEY (orphanage_id_chirho) REFERENCES orphanage_profiles_chirho(orphanage_id_chirho)
);

-- Needs Management Tables
CREATE TABLE IF NOT EXISTS needs_chirho (
    need_id_chirho TEXT PRIMARY KEY,
    orphanage_id_chirho TEXT NOT NULL,
    title_chirho TEXT NOT NULL,
    description_chirho TEXT NOT NULL,
    amount_needed_chirho REAL NOT NULL,
    amount_raised_chirho REAL NOT NULL DEFAULT 0,
    status_chirho TEXT NOT NULL CHECK (status_chirho IN ('active_chirho', 'fulfilled_chirho', 'cancelled_chirho')),
    created_at_chirho TEXT NOT NULL,
    updated_at_chirho TEXT NOT NULL,
    FOREIGN KEY (orphanage_id_chirho) REFERENCES orphanage_profiles_chirho(orphanage_id_chirho)
);

-- Communication Tables
CREATE TABLE IF NOT EXISTS messages_chirho (
    message_id_chirho TEXT PRIMARY KEY,
    sponsorship_id_chirho TEXT NOT NULL,
    sender_user_id_chirho TEXT NOT NULL,
    receiver_child_id_chirho TEXT NOT NULL,
    message_text_chirho TEXT NOT NULL,
    sent_at_chirho TEXT NOT NULL,
    read_status_chirho BOOLEAN NOT NULL DEFAULT FALSE,
    moderation_status_chirho TEXT NOT NULL CHECK (moderation_status_chirho IN ('pending_chirho', 'approved_chirho', 'rejected_chirho')),
    moderator_staff_user_id_chirho TEXT,
    created_at_chirho TEXT NOT NULL,
    updated_at_chirho TEXT NOT NULL,
    FOREIGN KEY (sponsorship_id_chirho) REFERENCES sponsorships_chirho(sponsorship_id_chirho),
    FOREIGN KEY (sender_user_id_chirho) REFERENCES sponsor_users_chirho(user_id_chirho),
    FOREIGN KEY (receiver_child_id_chirho) REFERENCES child_profiles_chirho(child_id_chirho),
    FOREIGN KEY (moderator_staff_user_id_chirho) REFERENCES staff_users_chirho(user_id_chirho)
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_staff_users_orphanage_chirho ON staff_users_chirho(orphanage_id_chirho);
CREATE INDEX IF NOT EXISTS idx_child_profiles_orphanage_chirho ON child_profiles_chirho(orphanage_id_chirho);
CREATE INDEX IF NOT EXISTS idx_child_updates_child_chirho ON child_updates_chirho(child_id_chirho);
CREATE INDEX IF NOT EXISTS idx_sponsorships_sponsor_chirho ON sponsorships_chirho(sponsor_id_chirho);
CREATE INDEX IF NOT EXISTS idx_sponsorships_child_chirho ON sponsorships_chirho(child_id_chirho);
CREATE INDEX IF NOT EXISTS idx_donations_donor_chirho ON donations_chirho(donor_id_chirho);
CREATE INDEX IF NOT EXISTS idx_donations_orphanage_chirho ON donations_chirho(orphanage_id_chirho);
CREATE INDEX IF NOT EXISTS idx_needs_orphanage_chirho ON needs_chirho(orphanage_id_chirho);
CREATE INDEX IF NOT EXISTS idx_messages_sponsorship_chirho ON messages_chirho(sponsorship_id_chirho);
CREATE INDEX IF NOT EXISTS idx_messages_sender_chirho ON messages_chirho(sender_user_id_chirho);
CREATE INDEX IF NOT EXISTS idx_messages_receiver_chirho ON messages_chirho(receiver_child_id_chirho); 