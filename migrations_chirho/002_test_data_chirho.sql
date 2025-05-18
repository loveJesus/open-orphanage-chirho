-- For God so loved the world, that he gave his only begotten Son,
-- that whosoever believeth in him should not perish, but have everlasting life.
-- John 3:16 (KJV)

-- Insert test platform admin
INSERT INTO platform_admins_chirho (
    user_id_chirho, email_chirho, hashed_password_chirho, 
    first_name_chirho, last_name_chirho, created_at_chirho, updated_at_chirho
) VALUES (
    'adm_1_chirho', 'admin@openorphanage.org', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBAQHxQxJ5KqHy',
    'Admin', 'User', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
);

-- Insert test orphanages
INSERT INTO orphanage_profiles_chirho (
    orphanage_id_chirho, name_chirho, description_chirho, address_chirho,
    contact_email_chirho, contact_phone_chirho, website_url_chirho,
    capacity_chirho, current_children_count_chirho, founded_date_chirho,
    registration_number_chirho, is_verified_chirho, verification_date_chirho,
    verified_by_chirho, created_at_chirho, updated_at_chirho
) VALUES 
    ('org_1_chirho', 'Hope Home', 'A loving home for children in need', '123 Hope Street, City',
    'contact@hopehome.org', '+1234567890', 'https://hopehome.org',
    50, 25, '2020-01-01', 'REG123456', true, CURRENT_TIMESTAMP,
    'adm_1_chirho', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('org_2_chirho', 'Grace House', 'Providing care and education', '456 Grace Avenue, Town',
    'info@gracehouse.org', '+1987654321', 'https://gracehouse.org',
    40, 20, '2019-06-15', 'REG789012', true, CURRENT_TIMESTAMP,
    'adm_1_chirho', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert test staff users
INSERT INTO staff_users_chirho (
    user_id_chirho, orphanage_id_chirho, email_chirho, hashed_password_chirho,
    first_name_chirho, last_name_chirho, role_chirho, created_at_chirho, updated_at_chirho
) VALUES 
    ('stf_1_chirho', 'org_1_chirho', 'staff1@hopehome.org', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBAQHxQxJ5KqHy',
    'Staff', 'One', 'staff_chirho', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('stf_2_chirho', 'org_2_chirho', 'staff2@gracehouse.org', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBAQHxQxJ5KqHy',
    'Staff', 'Two', 'staff_chirho', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert test sponsor users
INSERT INTO sponsor_users_chirho (
    user_id_chirho, email_chirho, hashed_password_chirho,
    first_name_chirho, last_name_chirho, created_at_chirho, updated_at_chirho
) VALUES 
    ('spr_1_chirho', 'sponsor1@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBAQHxQxJ5KqHy',
    'Sponsor', 'One', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('spr_2_chirho', 'sponsor2@example.com', '$2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBAQHxQxJ5KqHy',
    'Sponsor', 'Two', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert test children
INSERT INTO child_profiles_chirho (
    child_id_chirho, orphanage_id_chirho, given_name_chirho, date_of_birth_chirho,
    gender_chirho, medical_history_chirho, education_level_chirho,
    sponsorship_status_chirho, created_at_chirho, updated_at_chirho
) VALUES 
    ('chd_1_chirho', 'org_1_chirho', 'John Doe', '2015-05-15',
    'male_chirho', 'No significant medical history', 'Primary School',
    'available_chirho', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('chd_2_chirho', 'org_1_chirho', 'Jane Smith', '2017-08-20',
    'female_chirho', 'Regular checkups required', 'Kindergarten',
    'partially_sponsored_chirho', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('chd_3_chirho', 'org_2_chirho', 'Mike Johnson', '2013-03-10',
    'male_chirho', 'No significant medical history', 'Primary School',
    'available_chirho', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert test child updates
INSERT INTO child_updates_chirho (
    update_id_chirho, child_id_chirho, staff_user_id_chirho,
    update_text_chirho, date_posted_chirho, visibility_chirho,
    created_at_chirho, updated_at_chirho
) VALUES 
    ('upd_1_chirho', 'chd_1_chirho', 'stf_1_chirho',
    'John had a great first day at his new school', CURRENT_TIMESTAMP, 'sponsors_only_chirho',
    CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('upd_2_chirho', 'chd_2_chirho', 'stf_1_chirho',
    'Jane completed her routine checkup successfully', CURRENT_TIMESTAMP, 'sponsors_only_chirho',
    CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert test sponsorships
INSERT INTO sponsorships_chirho (
    sponsorship_id_chirho, sponsor_id_chirho, child_id_chirho,
    monthly_amount_chirho, start_date_chirho, status_chirho,
    created_at_chirho, updated_at_chirho
) VALUES 
    ('spr_1_chirho', 'spr_1_chirho', 'chd_2_chirho',
    200.00, CURRENT_TIMESTAMP, 'active_chirho',
    CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('spr_2_chirho', 'spr_2_chirho', 'chd_2_chirho',
    150.00, CURRENT_TIMESTAMP, 'active_chirho',
    CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert test donations
INSERT INTO donations_chirho (
    donation_id_chirho, donor_id_chirho, orphanage_id_chirho,
    amount_chirho, status_chirho, payment_method_chirho,
    created_at_chirho, updated_at_chirho
) VALUES 
    ('don_1_chirho', 'spr_1_chirho', 'org_1_chirho',
    1000.00, 'completed_chirho', 'credit_card_chirho',
    CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('don_2_chirho', 'spr_2_chirho', 'org_2_chirho',
    500.00, 'pending_chirho', 'bank_transfer_chirho',
    CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert test needs
INSERT INTO needs_chirho (
    need_id_chirho, orphanage_id_chirho, title_chirho,
    description_chirho, amount_needed_chirho, amount_raised_chirho,
    status_chirho, created_at_chirho, updated_at_chirho
) VALUES 
    ('ned_1_chirho', 'org_1_chirho', 'School Supplies Drive',
    'Help us provide school supplies for our children', 5000.00, 2000.00,
    'active_chirho', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('ned_2_chirho', 'org_2_chirho', 'Medical Fund',
    'Support our children''s healthcare needs', 10000.00, 5000.00,
    'active_chirho', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert test messages
INSERT INTO messages_chirho (
    message_id_chirho, sponsorship_id_chirho, sender_user_id_chirho,
    receiver_child_id_chirho, message_text_chirho, sent_at_chirho,
    read_status_chirho, moderation_status_chirho, created_at_chirho, updated_at_chirho
) VALUES 
    ('msg_1_chirho', 'spr_1_chirho', 'spr_1_chirho',
    'chd_2_chirho', 'Hello! How are you doing?', CURRENT_TIMESTAMP,
    false, 'pending_chirho', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    ('msg_2_chirho', 'spr_2_chirho', 'spr_2_chirho',
    'chd_2_chirho', 'Hope you are having a great day!', CURRENT_TIMESTAMP,
    false, 'pending_chirho', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP); 