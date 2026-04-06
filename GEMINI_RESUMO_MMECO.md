# 🎯 Resumo do Projeto MMECO para Gemini

## 📋 Visão Geral

**MMECO** = **Moral Money Ecosystem** - Uma blockchain baseada em Substrate/Polkadot SDK que implementa um sistema económico baseado em reputação e governança comunitária.

## 🏗️ Arquitetura Técnica

### **Tech Stack:**
- **Linguagem:** Rust
- **Framework:** Substrate/Polkadot SDK (branch stable2412)
- **Runtime:** WASM + Native
- **Consenso:** Aura (block production) + Grandpa (finalization)

### **Estrutura do Projeto:**
```
MMECO/
├── blockchain-core/
│   ├── runtime/          # mmeco-runtime (integração de todas as pallets)
│   ├── pallets/
│   │   ├── projects/     # Gestão de projetos comunitários
│   │   ├── governance/   # Sistema de propostas e votação
│   │   └── reputation/   # Algoritmo de reputação
│   ├── node/             # Nó da blockchain
│   └── Cargo.toml        # Workspace dependencies
├── ai_agents/            # Sistema de agentes IA para análise de erros
│   ├── blockchain_error_agent.py
│   ├── blockchain_memory.py
│   ├── rust_error_patterns.py
│   └── start_sdk_agents.py
└── docs/                 # Documentação do sistema
```

## 🤖 Sistema de Agentes IA (SDK Agents)

### **Objetivo:**
Analisar automaticamente erros de compilação Rust/Substrate durante o desenvolvimento e guardar soluções para referência futura.

### **Componentes:**

1. **`blockchain_error_agent.py`**
   - Executa `cargo build` e analisa erros
   - Classifica erros por categoria (E0277, E0308, E0432, E0433, etc.)
   - Procura soluções na base de conhecimento e memória local
   - **Correções recentes:**
     - Removido `"2>&1"` inválido do subprocess
     - Adicionado controlo de loops redundantes (`last_output`)
     - Implementada deduplicação com SHA256 + normalização

2. **`blockchain_memory.py`**
   - Armazena histórico de erros e soluções em JSON
   - Usa SHA256 estável para deduplicação: `hash(error_code|location|normalized_message)`
   - Normaliza mensagens (remove números variáveis, whitespace)
   - Pesquisa por similaridade para erros sem código

3. **`rust_error_patterns.py`**
   - Base de conhecimento de ~15 padrões de erros comuns
   - Inclui soluções e exemplos de correção
   - Focado em erros Substrate-specific (MaxEncodedLen, Storage, Pallet Config, etc.)

4. **`start_sdk_agents.py`**
   - Orquestrador que monitoriza builds do GitHub Actions
   - Deteta falhas e extrai erros para análise

### **Estado Atual dos Agentes:**
- ✅ Subprocess corrigido (não gera mais erro `"unexpected argument '2>&1'"`)
- ✅ Controlo de loops implementado (evita reprocessar mesmo output)
- ✅ Deduplicação estável com SHA256
- ✅ Memória limpa e pronta para erros reais

## 🔧 Pallets Personalizadas

### **1. pallet-projects**
- **Função:** Permitir criação e contribuição para projetos comunitários
- **Storage:** `Contributions<project_id>` - lista de contribuições por projeto
- **Calls:** `contribute(project_id, amount)`
- **Estado:** Funcional, com warnings de deprecated (implicit call index, constant weight)

### **2. pallet-governance**
- **Função:** Sistema de propostas e votação comunitária
- **Events:** `ProposalCreated { proposal_id }`
- **Estado:** Estrutura base criada, pronta para implementar calls de votação

### **3. pallet-reputation**
- **Função:** Algoritmo de reputação dos utilizadores
- **Estado:** Estrutura base criada, pronta para implementar lógica de reputação

## 📊 Estado Atual do Build

### **Erros Atuais no Runtime:**
- ❌ **Trait bound errors** - `UncheckedExtrinsic` não satisfaz `Checkable<ChainContext<Runtime>>`
- ❌ **Executive::offchain_worker** - não pode ser chamado devido a trait bounds
- ⚠️ **Warnings** - deprecated implicit call indices, constant weight, dead code (deposit_event)

### **Próximos Passos:**
1. Corrigir trait bounds no runtime (provavelmente relacionado com TxExtension)
2. Implementar calls faltantes nas pallets (votar propostas, etc.)
3. Testar build completo até ao fim
4. Corrigir warnings de deprecated (adicionar `#[pallet::call_index]`)

## 🎯 Objetivo Imediato

**Para o Gemini:** Precisamos de ajuda para:
1. Corrigir os trait bounds no runtime para compilar sem erros
2. Criar uma pallet de governance simplificada para testar compilação isolada (sem depender do runtime completo)
3. Implementar a lógica completa de reputação e governança

## 📁 Ficheiros Chave

- `blockchain-core/runtime/src/lib.rs` - Runtime principal (288 linhas)
- `blockchain-core/pallets/*/src/lib.rs` - Pallets personalizadas
- `ai_agents/blockchain_error_agent.py` - Agente de análise de erros
- `ai_agents/blockchain_errors_memory.json` - Memória de erros (atualmente vazia)
- `docs/internal/error-tracker/` - Histórico de erros resolvidos

---

**Última Atualização:** 2026-04-06
**Branch:** gpt-1.0
**Commits Recentes:**
- `094c11f` - Fix: Change sp_runtime import to polkadot_sdk::sp_runtime
- `dddceb1` - Fix: Add loop control and SHA256 deduplication
- `e1da68f` - Fix: Correct subprocess call (removed invalid 2>&1)