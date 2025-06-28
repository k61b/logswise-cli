# Supabase Setup for Logswise CLI

> **🚀 New: Automatic Setup Available!**
>
> As of the latest version, Logswise CLI can automatically set up your Supabase database during the initial setup process. Simply run `logswise-cli setup` and the CLI will handle table creation and schema setup for you.
>
> If you already have Supabase configured but need to set up the database, run: `logswise-cli init`
>
> The manual instructions below are provided for reference or if automatic setup is not available for your Supabase instance.

## 1. Create a Supabase Project

- Go to https://app.supabase.com/ and create a new project.
- Note your Project URL and anon/public API key (found in Project Settings → API).

## 2. Create the `notes` Table

Go to the SQL Editor in your Supabase project and run:

```sql
create table notes (
  id uuid primary key default gen_random_uuid(),
  content text not null,
  created_at timestamp with time zone default timezone('utc'::text, now())
);
```

> **Note:** You do **not** need a `suggestions` table. Suggestions are generated dynamically using your profile and recent notes, powered by your local LLM (Ollama).

## 2b. Enable pgvector and Add Embedding Column

To enable semantic search, run these SQL commands in the Supabase SQL Editor:

```sql
-- Enable pgvector extension (run once)
create extension if not exists vector;

-- Add embedding column to notes table (1536 for OpenAI, adjust if needed)
alter table notes add column if not exists embedding vector(768);
```

> **Note:** The CLI will use this column to store and search note embeddings for fast, relevant suggestions.

## 3. Use the CLI Setup

Run:

```sh
npm run setup
```

or, for the Rust CLI:

```sh
logswise-cli setup
```

and enter your Supabase Project URL and API key when prompted.

---

**Summary:**

- All credentials and profile info are stored in `setup.json` locally.
- The CLI reads from this file for all operations.
- Your Supabase DB only needs a `notes` table as shown above.
- No personal profile info is stored in the database—only notes.
- Suggestions and chat are powered by your local LLM (Ollama) using your context.

**Automatic Setup Commands:**

- `logswise-cli setup` - Complete setup including automatic database creation
- `logswise-cli init` - Initialize database schema (requires existing config)
- `logswise-cli doctor` - Verify setup and diagnose any issues
