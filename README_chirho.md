# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life.
# OpenOrphanageChirho

For God so loved the world, that he gave his only begotten Son,
that whosoever believeth in him should not perish, but have everlasting life.
John 3:16 (KJV)

## Overview

OpenOrphanageChirho is a platform designed to help orphanages and children's programs manage their operations, with a focus on child sponsorship and needs management. The system is built with Rust on Cloudflare Workers and uses Cloudflare D1 for data storage.

## Features

- User Management (Staff, Sponsors, Platform Admins)
- Orphanage Profile Management
- Child Profile Management
- Sponsorship Management
- Donation Management
- Communication System
- Needs Management

## Technology Stack

- Backend: Rust
- Deployment: Cloudflare Workers
- Database: Cloudflare D1 (SQL-based)
- Authentication: JWT-based

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Wrangler CLI
- Cloudflare account

### Installation

1. Clone the repository:
```bash
git clone https://github.com/your-org/open-orphanage-chirho.git
cd open-orphanage-chirho
```

2. Install dependencies:
```bash
cargo build
```

3. Configure Wrangler:
```bash
wrangler login
```

4. Update the `wrangler.toml` file with your Cloudflare account details and database ID.

### Development

1. Start the development server:
```bash
wrangler dev
```

2. Run tests:
```bash
cargo test
```

### Deployment

1. Deploy to staging:
```bash
wrangler deploy --env staging
```

2. Deploy to production:
```bash
wrangler deploy --env production
```

## API Documentation

### Authentication Endpoints

- `POST /api_chirho/v1_chirho/users_chirho/register/staff_chirho` - Register a new staff user
- `POST /api_chirho/v1_chirho/users_chirho/register/sponsor_chirho` - Register a new sponsor
- `POST /api_chirho/v1_chirho/users_chirho/login_chirho` - Login user

### Orphanage Endpoints

- `GET /api_chirho/v1_chirho/orphanages_chirho` - List verified orphanages
- `POST /api_chirho/v1_chirho/orphanages_chirho` - Create new orphanage
- `GET /api_chirho/v1_chirho/orphanages_chirho/:id_chirho` - Get orphanage details
- `PUT /api_chirho/v1_chirho/orphanages_chirho/:id_chirho` - Update orphanage details

### Child Profile Endpoints

- `GET /api_chirho/v1_chirho/children_chirho` - List children available for sponsorship
- `POST /api_chirho/v1_chirho/children_chirho` - Add new child profile
- `GET /api_chirho/v1_chirho/children_chirho/:id_chirho` - Get child details
- `PUT /api_chirho/v1_chirho/children_chirho/:id_chirho` - Update child details

### Sponsorship Endpoints

- `POST /api_chirho/v1_chirho/sponsorships_chirho` - Create new sponsorship
- `GET /api_chirho/v1_chirho/sponsorships_chirho` - List user's sponsorships
- `PUT /api_chirho/v1_chirho/sponsorships_chirho/:id_chirho` - Update sponsorship

### Donation Endpoints

- `POST /api_chirho/v1_chirho/donations_chirho` - Create new donation
- `GET /api_chirho/v1_chirho/donations_chirho` - List donations

### Communication Endpoints

- `POST /api_chirho/v1_chirho/communications_chirho` - Send message
- `GET /api_chirho/v1_chirho/communications_chirho` - List messages

### Needs Management Endpoints

- `POST /api_chirho/v1_chirho/needs_chirho` - Post new need
- `GET /api_chirho/v1_chirho/needs_chirho` - List needs
- `PUT /api_chirho/v1_chirho/needs_chirho/:id_chirho` - Update need status

## Security

- All sensitive endpoints require JWT authentication
- Passwords are hashed using Argon2
- Input validation is performed on all endpoints
- Rate limiting is implemented for public endpoints

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature-chirho`)
3. Commit your changes (`git commit -m 'Add some amazing feature chirho'`)
4. Push to the branch (`git push origin feature/amazing-feature-chirho`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Thanks to all contributors and supporters of this project
- Special thanks to Cloudflare for providing the infrastructure 