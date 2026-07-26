## [2.1.0] - 2026-07-26

### Added
- Four Card Omaha Hold 'em poker variant evaluation supported.

### Changed

### Fixed

### Removed

## [2.0.0] - 2026-07-26

### Added

### Changed
- BREAKING: Hand descriptions now include rank detail for every hand type (`"Two Pair"` → `"Two Pair, Kings and Queens"`, `"Straight"` → `"King High Straight"` etc).

### Fixed

### Removed

## [1.1.1] - 2026-07-25

### Added

### Changed

### Fixed

### Removed
- Redundant `variant` query parameter on `POST /evaluate`.

## [1.1.0] - 2026-07-25

### Added
- Texas Hold 'em poker variant evaluation supported.

### Changed

### Fixed

### Removed

## [1.0.0] - 2026-05-26

### Added
- First poker variant evaluation supported (Five-card draw).
- Library level validation pre evaluation (easily extendable for more poker variants).
- Secondary binary HTTP server that exposes evaluation logic.
- Dockerfile for HTTP binary.
- Testing pipeline on PRs.

### Changed

### Fixed

### Removed

