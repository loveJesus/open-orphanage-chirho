**For God so loved the world, that He gave His only begotten Son, that whoever believes in Him should not perish but have everlasting life.** (John 3:16)
// John 3:16 (KJV)

# OpenOrphanageChirho

> _"Pure religion and undefiled before God and the Father is this, To visit the fatherless and widows in their affliction, and to keep himself unspotted from the world."_  
> **James 1:27 (KJV)** 

## The Gospel

The good news (Gospel) is this: God created you and loves you. Our sins separate us from God, but Jesus Christ, God's only Son, came to earth, lived a sinless life, died on the cross for our sins, and rose again. Whoever believes in Him and receives Him as Lord and Savior is forgiven, reconciled to God, and given eternal life. This gift is available to all who trust in Jesus Christ.

---

## Project Overview

OpenOrphanageChirho is an open-source platform designed to empower orphanages and children's programs with modern digital tools for managing child sponsorship, needs, donations, and communication. Built with a focus on transparency, security, and extensibility, the platform aims to serve both orphanage staff and sponsors, providing a robust backend API and, soon, a user-friendly frontend.

### Key Features
- **User Management:** Staff, sponsors, and platform admins with role-based access control.
- **Orphanage Profiles:** CRUD operations for orphanage data and verification workflows.
- **Child Profiles:** Secure management of child data, sponsorship status, and updates.
- **Sponsorships:** Initiate and manage sponsorship relationships and payments.
- **Donations:** Track and manage general and need-specific donations.
- **Needs Management:** Orphanages can post and update specific needs for support.
- **Communication:** Moderated messaging between sponsors and children (via staff).

### Technology Stack
- **Backend:** Rust (Cloudflare Workers)
- **Database:** Cloudflare D1 (SQL-based)
- **API:** RESTful, OpenAPI 3.0 Spec
- **Frontend:** _Coming soon!_

### Development Principles
- All identifiers are suffixed with `Chirho` for clarity and consistency.
- Security, privacy, and PII protection are core priorities.
- Modular, testable, and well-documented codebase.

---

## Getting Started

1. **Clone the repository:**
   ```sh
   git clone <repo-url>
   cd open-orphanage-chirho
   ```
2. **Build and run the backend:**
   ```sh
   npm install
   npx wrangler dev
   ```
3. **Configure Cloudflare Workers:**
   - Edit `wrangler.toml` for your environment and D1 database bindings.

4. **Run tests:**
   ```sh
   cargo test
   ```

---

## Contributing

Contributions are welcome! Please open issues or pull requests for improvements, bug fixes, or new features. All contributions should adhere to the identifier suffixing and file header comment rules.

---

## License

This project is open-source and available under the MIT License.

---

## Contact

For questions, support, or partnership inquiries, please contact: [support@openorphanage.org](mailto:support@openorphanage.org)

---

