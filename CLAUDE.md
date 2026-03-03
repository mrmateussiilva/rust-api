# Rust API - Projeto de Esteiras CI/CD

## Visão Geral

API para gerenciar esteiras de CI/CD com frontend React. Monorepo com backend (Rust/actix-web) e frontend (React/TypeScript).

## Estrutura

```
├── backend/                 # API Rust
│   ├── src/
│   │   ├── handlers/       # CRUD handlers
│   │   ├── models/        # Modelos e DTOs
│   │   ├── routes/        # Definição de rotas
│   │   └── main.rs       # Entry point
│   ├── migrations/        # SQL migrations
│   ├── Dockerfile        # Produção
│   └── Cargo.toml
├── frontend/              # React + Vite + TypeScript
│   ├── src/
│   │   ├── components/   # Componentes React
│   │   ├── services/     # API client
│   │   ├── types/       # TypeScript types
│   │   └── App.tsx
│   └── package.json
├── scripts/
│   └── dev.sh           # Script de desenvolvimento
├── docker-compose.yml   # PostgreSQL + API
└── Cargo.toml           # Workspace
```

## Variáveis de Ambiente

### Backend (.env)
```
DATABASE_URL=postgres://pipeline:pipeline123@localhost:5432/pipeline
HOST=0.0.0.0
PORT=8080
```

### Frontend (.env)
```
VITE_API_URL=http://localhost:8080
```

## Scripts

### Desenvolvimento
```bash
# Com tudo (backend + frontend)
./scripts/dev.sh --with-frontend

# Só backend
./scripts/dev.sh
```

### Docker
```bash
docker-compose up --build
```

## API Endpoints

| Método | Endpoint | Descrição |
|--------|----------|-----------|
| GET | /pipelines | Lista todas as esteiras |
| POST | /pipelines | Cria esteira |
| GET | /pipelines/:id | Busca esteira |
| PUT | /pipelines/:id | Atualiza esteira |
| DELETE | /pipelines/:id | Deleta esteira |
| POST | /stages | Cria estágio |
| PATCH | /stages/:id/status | Atualiza status |
| DELETE | /stages/:id | Deleta estágio |

## Modelos

### Pipeline
```json
{
  "id": "uuid",
  "name": "string",
  "description": "string",
  "stages": [],
  "createdAt": "datetime",
  "updatedAt": "datetime"
}
```

### Stage
```json
{
  "id": "uuid",
  "pipelineId": "uuid",
  "name": "string",
  "order": "number",
  "status": "pending | running | success | failed",
  "createdAt": "datetime",
  "updatedAt": "datetime"
}
```

## Stack

- **Backend**: Rust, actix-web, sqlx, PostgreSQL
- **Frontend**: React 19, TypeScript, Vite
- **Database**: PostgreSQL 15
- **Container**: Docker, Docker Compose

## Convenções

### Rust
- Handlers em `src/handlers/mod.rs`
- Models em `src/models/mod.rs`
- Rotas em `src/routes/mod.rs`
- Estado global via `AppState`

### React
- Componentes em `src/components/`
- Tipos em `src/types/`
- API client em `src/services/api.ts`
- CSS junto aos componentes

## Deploy

### Backend (VPS)
```bash
docker build -t rust-api ./backend
docker run -d -p 8080:8080 --env DATABASE_URL=postgres://... rust-api
```

### Frontend (Vercel)
- Build: `cd frontend && pnpm build`
- Output: `frontend/dist`
