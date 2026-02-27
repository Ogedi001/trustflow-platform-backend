-- Identity Service Database Schema
-- Migration: 20240301000000_initial_schema
-- Description: Initial schema for identity service with user, profile, verification, session, and role management

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ENUMS
CREATE TYPE user_role AS ENUM ('GUEST', 'BUYER', 'SELLER', 'MODERATOR', 'ADMIN', 'SUPERADMIN');
CREATE TYPE user_status AS ENUM ('PENDING', 'ACTIVE', 'SUSPENDED', 'DELETED', 'LOCKED', 'AWAITING_APPROVAL');
CREATE TYPE verification_level AS ENUM ('LEVEL0', 'LEVEL1', 'LEVEL2', 'LEVEL3', 'LEVEL4');
CREATE TYPE verification_status AS ENUM ('PENDING', 'APPROVED', 'REJECTED', 'EXPIRED', 'MANUAL_REVIEW', 'CANCELLED');
CREATE TYPE verification_method AS ENUM ('EMAIL', 'PHONE', 'DOCUMENT', 'MANUAL', 'BIOMETRIC', 'SOCIAL', 'PROVIDER', 'BANK');
CREATE TYPE document_type AS ENUM ('NIN', 'DRIVERS_LICENSE', 'INTERNATIONAL_PASSPORT', 'CAC_CERTIFICATE', 'TIN', 'VOTERS_CARD', 'PVC', 'OTHER');
CREATE TYPE mfa_method AS ENUM ('TOTP', 'SMS', 'EMAIL', 'PUSH', 'WEBAUTHN');

-- ROLES TABLE (One Role -> Many Users)
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) UNIQUE NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    permissions JSONB NOT NULL DEFAULT '[]',
    role_level INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_system_role BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_role_level CHECK (role_level >= 0 AND role_level <= 100)
);

-- INDEXES FOR ROLES
CREATE INDEX idx_roles_name ON roles(name);
CREATE INDEX idx_roles_level ON roles(role_level DESC);

-- INSERT DEFAULT ROLES
INSERT INTO roles (name, display_name, permissions, role_level, is_system_role, is_default) VALUES
('SUPERADMIN', 'Super Administrator', '[{"resource": "*", "action": "*"}]', 100, TRUE, FALSE),
('ADMIN', 'Administrator', '[{"resource": "users", "action": "*"}, {"resource": "verifications", "action": "*"}, {"resource": "analytics", "action": "read"}]', 80, TRUE, FALSE),
('MODERATOR', 'Moderator', '[{"resource": "users", "action": "read"}, {"resource": "verifications", "action": "*"}]', 60, FALSE, FALSE),
('SELLER', 'Seller', '[{"resource": "listings", "action": "*"}, {"resource": "orders", "action": "read"}, {"resource": "profile", "action": "*"}]', 40, FALSE, FALSE),
('BUYER', 'Buyer', '[{"resource": "listings", "action": "read"}, {"resource": "orders", "action": "*"}, {"resource": "profile", "action": "*"}]', 20, FALSE, TRUE),
('GUEST', 'Guest', '[{"resource": "listings", "action": "read"}]', 0, FALSE, FALSE);

-- USERS TABLE (Many-to-One: User -> Role)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    phone VARCHAR(20) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'BUYER',
    role_id UUID REFERENCES roles(id),  -- Many-to-one relationship
    status user_status NOT NULL DEFAULT 'PENDING',
    verification_level verification_level NOT NULL DEFAULT 'LEVEL0',
    mfa_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    mfa_secret VARCHAR(255),
    last_login_at TIMESTAMP WITH TIME ZONE,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    
    CONSTRAINT valid_verification_level CHECK (verification_level IN ('LEVEL0', 'LEVEL1', 'LEVEL2', 'LEVEL3', 'LEVEL4'))
);

-- INDEXES FOR USERS
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_phone ON users(phone);
CREATE INDEX idx_users_status ON users(status);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_role_id ON users(role_id);
CREATE INDEX idx_users_created_at ON users(created_at DESC);

-- USER PROFILES TABLE (One-to-One: User -> Profile)
CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    display_name VARCHAR(100),
    avatar_url VARCHAR(500),
    date_of_birth DATE,
    gender VARCHAR(20),
    address JSONB,
    business_name VARCHAR(255),
    business_registration_number VARCHAR(50),
    tax_id VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- INDEXES FOR USER PROFILES
CREATE INDEX idx_profiles_business_name ON user_profiles(business_name);

-- VERIFICATION RECORDS TABLE (One-to-Many: User -> Verifications)
CREATE TABLE verification_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    level verification_level NOT NULL,
    status verification_status NOT NULL DEFAULT 'PENDING',
    method verification_method NOT NULL,
    document_type document_type,
    document_url VARCHAR(500),
    document_hash VARCHAR(128),
    verified_by UUID REFERENCES users(id),
    verified_at TIMESTAMP WITH TIME ZONE,
    expires_at TIMESTAMP WITH TIME ZONE,
    rejection_reason TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- INDEXES FOR VERIFICATION RECORDS
CREATE INDEX idx_verification_user ON verification_records(user_id);
CREATE INDEX idx_verification_status ON verification_records(status);
CREATE INDEX idx_verification_level ON verification_records(level);
CREATE INDEX idx_verification_expires ON verification_records(expires_at) WHERE expires_at IS NOT NULL;

-- SESSIONS TABLE (One-to-Many: User -> Sessions)
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_id VARCHAR(100) NOT NULL,
    user_agent TEXT,
    ip_address INET,
    token_hash VARCHAR(128) NOT NULL,
    refresh_token_hash VARCHAR(128) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    refresh_expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_activity_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_expiry CHECK (refresh_expires_at > expires_at)
);

-- INDEXES FOR SESSIONS
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_token_hash ON sessions(token_hash);
CREATE INDEX idx_sessions_expires ON sessions(expires_at);
CREATE INDEX idx_sessions_revoked ON sessions(revoked);

-- INVITE CODES TABLE
CREATE TABLE invite_codes (
    code VARCHAR(20) PRIMARY KEY,
    role user_role NOT NULL DEFAULT 'BUYER',
    max_uses INTEGER NOT NULL DEFAULT 1,
    used_count INTEGER NOT NULL DEFAULT 0,
    expires_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- INDEXES FOR INVITE CODES
CREATE INDEX idx_invite_codes_role ON invite_codes(role);

-- AUDIT LOG TABLE
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100),
    resource_id UUID,
    old_value JSONB,
    new_value JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- INDEXES FOR AUDIT LOGS
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_created ON audit_logs(created_at DESC);

-- FUNCTIONS

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers for updated_at
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_profiles_updated_at BEFORE UPDATE ON user_profiles FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_roles_updated_at BEFORE UPDATE ON roles FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Function to get role by name
CREATE OR REPLACE FUNCTION get_role_by_name(role_name user_role)
RETURNS TABLE (id UUID, name user_role, role_level INTEGER, permissions JSONB) AS $$
BEGIN
    RETURN QUERY
    SELECT r.id, r.name, r.role_level, r.permissions
    FROM roles r
    WHERE r.name = role_name AND r.is_active = TRUE;
END;
$$ LANGUAGE plpgsql STABLE;

-- Function to get user's effective permissions
CREATE OR REPLACE FUNCTION get_user_permissions(user_id UUID)
RETURNS JSONB AS $$
DECLARE
    perms JSONB;
BEGIN
    SELECT COALESCE(
        (SELECT r.permissions FROM roles r WHERE r.id = (
            SELECT u.role_id FROM users u WHERE u.id = user_id
        )),
        '{}'::jsonb
    ) INTO perms;
    RETURN perms;
END;
$$ LANGUAGE plpgsql STABLE;

-- VIEW: Users with roles
CREATE OR REPLACE VIEW users_with_roles AS
SELECT 
    u.id as user_id,
    u.email,
    u.phone,
    u.status,
    u.verification_level,
    u.role as role_name,
    r.display_name as role_display,
    r.role_level,
    r.permissions,
    u.created_at,
    u.last_login_at
FROM users u
LEFT JOIN roles r ON u.role_id = r.id OR u.role = r.name;

-- VIEW: Active users summary
CREATE OR REPLACE VIEW active_users_summary AS
SELECT 
    status,
    verification_level,
    role,
    COUNT(*) as count,
    DATE(created_at) as date
FROM users
WHERE deleted_at IS NULL
GROUP BY status, verification_level, role, DATE(created_at);

