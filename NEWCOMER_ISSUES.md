# OpenObserve: Good Issues for Newcomers 🌟

Welcome to OpenObserve! This document curates a selection of issues that are particularly suitable for newcomers who want to contribute to the project. These issues will help you understand the codebase while making meaningful contributions.

## About OpenObserve

OpenObserve is a cloud-native observability platform for logs, metrics, traces, and real user monitoring (RUM). It's built with:
- **Backend**: Rust (Actix Web framework)
- **Frontend**: Vue.js with TypeScript
- **Storage**: Embedded sled database, S3, MinIO, etc.
- **Protocols**: OTLP, gRPC, HTTP APIs

## 🎯 Excellent First Issues (Ready to Work On)

### 1. **API Consistency Fix** - Issue #7768
**Difficulty**: ⭐⭐☆☆☆ (Beginner-Intermediate)  
**Skills**: Backend (Rust), API Design  
**Description**: Resolve inconsistency where user roles endpoint uses `PUT` method in enterprise but `POST` in OSS version.

**What you'll learn**:
- REST API design principles
- Rust HTTP handlers
- API versioning and deprecation strategies

**Files to explore**:
- `src/handler/` - API route handlers
- `src/router/` - Route definitions

---

### 2. **Error Handling Improvement** - Issue #7708
**Difficulty**: ⭐⭐☆☆☆ (Beginner-Intermediate)  
**Skills**: Backend (Rust), Error Handling  
**Description**: Replace panic error with proper user-friendly error message when incorrect function is used.

**What you'll learn**:
- Rust error handling patterns
- Result type usage
- User experience considerations

**Files to explore**:
- `src/service/` - Business logic
- Error handling modules

---

### 3. **UI Consistency Fixes** - Issue #7594
**Difficulty**: ⭐☆☆☆☆ (Beginner)  
**Skills**: Frontend (Vue.js), Localization  
**Description**: Change naming convention from "Compare Against" to "Differential" in dashboards and alerts.

**What you'll learn**:
- Vue.js component structure
- Internationalization (i18n) in web apps
- UI consistency principles

**Files to explore**:
- `web/src/locales/` - Translation files
- `web/src/components/` - Vue components

---

## 🚀 Advanced Beginner Issues

### 4. **Trace Visualization Enhancement** - Issue #3764
**Difficulty**: ⭐⭐⭐☆☆ (Intermediate)  
**Skills**: Frontend (Vue.js), Data Visualization  
**Description**: Annotate events and exceptions in trace timeline with visual indicators (dots).

**What you'll learn**:
- Data visualization concepts
- OpenTelemetry trace data structure
- Vue.js reactive programming

---

### 5. **HTTP Client Unification** - Issue #3700
**Difficulty**: ⭐⭐⭐☆☆ (Intermediate)  
**Skills**: Backend (Rust), Refactoring  
**Description**: Unify usage of HTTP clients by using `reqwest` throughout and removing `awc`.

**What you'll learn**:
- Rust dependency management
- HTTP client libraries
- Large-scale refactoring techniques

---

### 6. **UI Bug Fixes** - Multiple Issues
**Difficulty**: ⭐⭐☆☆☆ (Beginner-Intermediate)  
**Skills**: Frontend (Vue.js), CSS, UX  

Several small UI issues that are perfect for newcomers:

- **#7772**: Error message consistency across pages
- **#7730**: Fix expanded log state persistence
- **#7687**: RUM page height display fix
- **#7719**: Include/exclude search term accessibility

---

## 🔍 Field-Specific Issue Categories

### Frontend Developers (Vue.js/TypeScript)
- UI consistency and localization fixes
- Component behavior improvements
- Dashboard and visualization enhancements
- RUM (Real User Monitoring) interface improvements

### Backend Developers (Rust)
- API endpoint improvements
- Error handling enhancements
- Data processing logic
- Protocol implementations (OTLP)

### Full-Stack Developers
- End-to-end feature implementations
- Data flow optimizations
- Integration improvements

## 📚 Getting Started Guide

### 1. **Set Up Development Environment**
```bash
# Clone the repository
git clone https://github.com/openobserve/openobserve
cd openobserve

# Frontend setup
cd web
npm install
npm run build
cd ..

# Backend setup (requires Rust 1.70+)
cargo build --release
```

### 2. **Understanding the Codebase**
- **Backend**: `src/` contains all Rust code
  - `src/handler/` - HTTP handlers
  - `src/service/` - Business logic
  - `src/router/` - Route definitions
- **Frontend**: `web/src/` contains Vue.js application
  - `web/src/components/` - Vue components
  - `web/src/views/` - Page views
  - `web/src/locales/` - Translations

### 3. **Development Workflow**
```bash
# Run backend (in project root)
ZO_ROOT_USER_EMAIL="root@example.com" ZO_ROOT_USER_PASSWORD="Complexpass#123" cargo run

# Run frontend (in web/ directory)
cd web
npm run dev
```

### 4. **Testing Your Changes**
```bash
# Backend tests
cargo test

# Frontend tests
cd web
npm run test

# Code coverage
./coverage.sh
```

## 🎓 Learning Path Suggestions

### For New Rust Developers
1. Start with UI consistency issues (#7594)
2. Move to error handling improvements (#7708)
3. Progress to API consistency fixes (#7768)
4. Tackle larger refactoring tasks (#3700)

### For Frontend Developers
1. Begin with localization fixes (#7594)
2. Work on UI bug fixes (#7772, #7730, #7687)
3. Advance to data visualization (#3764)
4. Contribute to dashboard improvements

### For Full-Stack Developers
1. Pick any frontend issue to understand the UI
2. Work on a backend issue to understand the API
3. Tackle cross-cutting concerns like error handling
4. Contribute to new feature development

## ⚠️ Issues to Avoid for Newcomers

- **Enterprise-related issues**: These require enterprise license access
- **Complex streaming/aggregation issues**: Need deep system understanding
- **Performance optimization issues**: Require extensive profiling knowledge
- **Database migration issues**: Need deep understanding of data structures

## 🤝 Contributing Guidelines

1. **Read the contributing guide**: Check `CONTRIBUTING.md` for detailed instructions
2. **Discuss before coding**: Comment on the issue to claim it and discuss approach
3. **Follow code style**: Use `cargo fmt` for Rust and `npm run lint` for frontend
4. **Write tests**: Ensure your changes are tested
5. **Update documentation**: Update docs if your changes affect user-facing features

## 📞 Getting Help

- **Slack Community**: [Join OpenObserve Slack](https://short.openobserve.ai/community)
- **GitHub Discussions**: Use repository discussions for questions
- **Issue Comments**: Ask questions directly on the issue you're working on

## 🏆 Benefits for Contributors

Working on these issues will help you:
- Learn modern Rust web development
- Understand observability platform architecture
- Gain experience with Vue.js and TypeScript
- Contribute to a growing open-source project
- Build portfolio-worthy contributions

---

**Ready to contribute?** Pick an issue that matches your skill level and interests, comment on it to let others know you're working on it, and start coding! 🚀

For questions or suggestions about this guide, please open a discussion in the repository.