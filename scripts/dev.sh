#!/bin/bash

set -e

# Cores para output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Diretório do projeto
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "🚀 Iniciando ambiente de desenvolvimento..."

# Função para verificar se PostgreSQL está rodando
check_postgres() {
    if docker ps --format '{{.Names}}' | grep -q "^pipeline-db$"; then
        echo -e "${GREEN}✓${NC} PostgreSQL já está rodando"
        return 0
    fi
    return 1
}

# Função para iniciar PostgreSQL
start_postgres() {
    echo -e "${YELLOW}📦${NC} Iniciando PostgreSQL..."
    
    # Verifica se o container existe (parado)
    if docker ps -a --format '{{.Names}}' | grep -q "^pipeline-db$"; then
        docker start pipeline-db
    else
        docker run -d \
            --name pipeline-db \
            -e POSTGRES_USER=pipeline \
            -e POSTGRES_PASSWORD=pipeline123 \
            -e POSTGRES_DB=pipeline \
            -p 5432:5432 \
            postgres:15-alpine
    fi
    
    # Aguarda PostgreSQL estar pronto
    echo -e "${YELLOW}⏳${NC} Aguardando PostgreSQL..."
    sleep 3
    
    echo -e "${GREEN}✓${NC} PostgreSQL rodando na porta 5432"
}

# Função para configurar banco
setup_database() {
    echo -e "${YELLOW}🗄️${NC} Configurando banco de dados..."
    
    cd "$PROJECT_DIR/backend"
    
    # Cria arquivo .env se não existir
    if [ ! -f .env ]; then
        echo "DATABASE_URL=postgres://pipeline:pipeline123@localhost:5432/pipeline" > .env
        echo -e "${GREEN}✓${NC} Arquivo .env criado"
    fi
    
    cd "$PROJECT_DIR"
    
    # Executa migrações se necessário
    echo -e "${YELLOW}🔄${NC} Verificando migrações..."
    if docker exec -i pipeline-db psql -U pipeline -d pipeline -c "\dt" 2>/dev/null | grep -q "pipelines"; then
        echo -e "${GREEN}✓${NC} Tabelas já existem"
    else
        echo -e "${YELLOW}🔄${NC} Criando tabelas..."
        docker exec -i pipeline-db psql -U pipeline -d pipeline < "$PROJECT_DIR/backend/migrations/001_initial.sql"
        echo -e "${GREEN}✓${NC} Tabelas criadas"
    fi
    
    echo -e "${GREEN}✓${NC} Banco configurado"
}

# Função para verificar e matar processo na porta
kill_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️${NC} Porta $port já está em uso, encerrando processo..."
        lsof -Pi :$port -sTCP:LISTEN -t | xargs kill -9 2>/dev/null || true
        sleep 1
    fi
}

# Função para iniciar backend
start_backend() {
    echo -e "${YELLOW}⚙️${NC} Iniciando Backend..."
    kill_port 8080
    cd "$PROJECT_DIR/backend"
    cargo run &
    cd "$PROJECT_DIR"
    echo -e "${GREEN}✓${NC} Backend rodando em http://localhost:8080"
}

# Função para iniciar frontend
start_frontend() {
    echo -e "${YELLOW}⚛️${NC} Iniciando Frontend..."
    kill_port 5173
    cd "$PROJECT_DIR/frontend"
    pnpm dev &
    cd "$PROJECT_DIR"
    echo -e "${GREEN}✓${NC} Frontend rodando em http://localhost:5173"
}

# Parse argumentos
FRONTEND=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --with-frontend)
            FRONTEND=true
            shift
            ;;
        *)
            echo "Uso: $0 [--with-frontend]"
            exit 1
            ;;
    esac
done

# Executa setup
if ! check_postgres; then
    start_postgres
fi

setup_database

# Inicia serviços em background
start_backend

if [ "$FRONTEND" = true ]; then
    start_frontend
fi

echo ""
echo -e "${GREEN}========================================"
echo "🎉 Ambiente de desenvolvimento pronto!"
echo "========================================"
echo "Backend: http://localhost:8080"
if [ "$FRONTEND" = true ]; then
    echo "Frontend: http://localhost:5173"
fi
echo ""
echo "APIs disponíveis:"
echo "  GET    /pipelines"
echo "  POST   /pipelines"
echo "  GET    /pipelines/:id"
echo "  PUT    /pipelines/:id"
echo "  DELETE /pipelines/:id"
echo "  POST   /stages"
echo "  PATCH  /stages/:id/status"
echo "  DELETE /stages/:id"
echo ""
echo "Para parar: docker stop pipeline-db && pkill -f 'cargo run'"
