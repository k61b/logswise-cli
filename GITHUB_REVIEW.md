# Logswise CLI

A CLI tool for note-taking, suggestions, and AI chat, powered by Supabase and LLMs.

## Project Structure
- `src/` — TypeScript source code
- `README.md` — Main documentation
- `SUPABASE_SETUP.md` — Supabase setup instructions
- `.gitignore` — Files to exclude from git
- `LICENSE` — MIT License
- `CONTRIBUTING.md` — Contribution guidelines
- `CODE_OF_CONDUCT.md` — Code of conduct

## Best Practices Checklist
- [x] Sensitive data is not committed (see `.gitignore`)
- [x] No secrets or API keys in code
- [x] MIT License included
- [x] Contribution and conduct guidelines
- [x] Usage and setup documented
- [x] No TypeScript errors
- [x] No TODOs or FIXMEs left in code
- [x] No use of `any` except where unavoidable (Supabase/LLM responses)
- [x] Console output is user-facing (CLI UX)

## Recommendations
- Consider replacing `any` types with more specific interfaces where possible.
- Add tests for core logic if you plan to grow the project.
- Keep your local `setup.json` out of version control (already handled).
- Update `author` and GitHub URLs in `package.json` before publishing.

---

You are ready to publish this project to GitHub!
