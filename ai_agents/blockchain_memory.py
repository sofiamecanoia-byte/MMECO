"""
Blockchain Memory System - Sistema de memória para erros Rust/Substrate
Mantém histórico de erros, soluções e padrões aprendidos
"""

import json
import re
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Tuple

# Caminhos para a documentação de erros
ERROR_TRACKER_DIR = Path(__file__).parent.parent / "docs" / "internal" / "error-tracker"
MEMORY_FILE = Path(__file__).parent / "blockchain_errors_memory.json"


class BlockchainMemory:
    """
    Sistema de memória especializado para erros de blockchain Rust/Substrate.
    Mantém cache local e integra com o error-tracker existente.
    """
    
    def __init__(self):
        self.errors_db: Dict[str, Dict] = {}
        self.load_memory()
    
    def load_memory(self):
        """Carrega a memória de erros do ficheiro JSON."""
        if MEMORY_FILE.exists():
            try:
                with open(MEMORY_FILE, 'r', encoding='utf-8') as f:
                    self.errors_db = json.load(f)
            except json.JSONDecodeError:
                self.errors_db = {}
    
    def save_memory(self):
        """Guarda a memória de erros no ficheiro JSON."""
        with open(MEMORY_FILE, 'w', encoding='utf-8') as f:
            json.dump(self.errors_db, f, indent=2, ensure_ascii=False)
    
    def _extract_error_code(self, error_msg: str) -> Optional[str]:
        """Extrai o código de erro Rust (E0xxx)."""
        match = re.search(r'\bE\d{4}\b', error_msg)
        return match.group(0) if match else None
    
    def _extract_file_line(self, error_msg: str) -> Optional[Tuple[str, int]]:
        """Extrai o ficheiro e linha do erro."""
        # Padrão: --> file.rs:linha
        match = re.search(r'-->\s+([^\s:]+):(\d+)', error_msg)
        if match:
            return (match.group(1), int(match.group(2)))
        return None
    
    def learn_error(self, error_msg: str, solution: str, context: str = ""):
        """
        Regista um novo erro aprendido.
        
        Args:
            error_msg: Mensagem de erro original
            solution: Solução aplicada
            context: Contexto adicional (pallet, módulo, etc.)
        """
        error_code = self._extract_error_code(error_msg)
        key = error_code or self._hash_error(error_msg)
        
        if key in self.errors_db:
            # Atualizar ocorrência
            self.errors_db[key]['count'] += 1
            self.errors_db[key]['last_seen'] = datetime.now().isoformat()
            if solution and not self.errors_db[key]['solution']:
                self.errors_db[key]['solution'] = solution
        else:
            # Novo erro
            self.errors_db[key] = {
                'error_code': error_code,
                'original_message': error_msg[:500],
                'solution': solution,
                'context': context,
                'count': 1,
                'first_seen': datetime.now().isoformat(),
                'last_seen': datetime.now().isoformat(),
                'resolved': solution != ""
            }
        
        self.save_memory()
    
    def _hash_error(self, error_msg: str) -> str:
        """Cria hash ESTÁVEL para erros sem código (SHA256 com normalização)."""
        import hashlib
        import re
        
        # Extrair código de erro se existir
        code_match = re.search(r'\bE\d{4}\b', error_msg)
        error_code = code_match.group(0) if code_match else "NONE"
        
        # Extrair localização (ficheiro:linha)
        loc_match = re.search(r'-->\s+([^\s:]+):(\d+)', error_msg)
        if loc_match:
            location = f"{loc_match.group(1)}:{loc_match.group(2)}"
        else:
            location = "unknown:0"
        
        # Normalizar mensagem (remover números variáveis, whitespace)
        normalized = error_msg.lower().strip()
        normalized = re.sub(r'\d+', 'N', normalized)  # substituir números por 'N'
        normalized = re.sub(r'\s+', ' ', normalized)   # normalizar whitespace
        
        # Criar chave estável com campos consistentes
        key = f"{error_code}|{location}|{normalized[:200]}"
        return hashlib.sha256(key.encode()).hexdigest()[:32]
    
    def search_solution(self, error_msg: str) -> Optional[Dict]:
        """
        Procura solução para um erro.
        
        Returns:
            Dict com 'solution', 'context', 'confidence' ou None
        """
        error_code = self._extract_error_code(error_msg)
        
        # Primeiro tentar por código de erro
        if error_code and error_code in self.errors_db:
            entry = self.errors_db[error_code]
            return {
                'solution': entry['solution'],
                'context': entry['context'],
                'confidence': 0.95 if entry['resolved'] else 0.5,
                'count': entry['count']
            }
        
        # Procurar por similaridade no texto
        error_lower = error_msg.lower()
        best_match = None
        best_score = 0
        
        for key, entry in self.errors_db.items():
            original = entry['original_message'].lower()
            # Calcular score de similaridade simples
            common_words = set(error_lower.split()) & set(original.split())
            score = len(common_words) / max(len(error_lower.split()), 1)
            
            if score > best_score and score > 0.3:
                best_score = score
                best_match = entry
        
        if best_match:
            return {
                'solution': best_match['solution'],
                'context': best_match['context'],
                'confidence': best_score * 0.8,
                'count': best_match['count']
            }
        
        return None
    
    def get_error_stats(self) -> Dict:
        """Retorna estatísticas dos erros conhecidos."""
        total = len(self.errors_db)
        resolved = sum(1 for e in self.errors_db.values() if e['resolved'])
        total_occurrences = sum(e['count'] for e in self.errors_db.values())
        
        # Top erros mais frequentes
        top_errors = sorted(
            self.errors_db.items(),
            key=lambda x: x[1]['count'],
            reverse=True
        )[:5]
        
        return {
            'total_errors': total,
            'resolved': resolved,
            'pending': total - resolved,
            'total_occurrences': total_occurrences,
            'top_errors': [
                {'code': k, 'count': v['count'], 'resolved': v['resolved']}
                for k, v in top_errors
            ]
        }
    
    def import_from_tracker(self):
        """Importa erros do error-tracker existente."""
        resolved_file = ERROR_TRACKER_DIR / "resolved.md"
        if not resolved_file.exists():
            return
        
        content = resolved_file.read_text()
        
        # Parse erros resolvidos do markdown
        # Formato: ## [DATA] Código Erro\n### Solução\n...
        pattern = r'##\s+\[.*?\]\s+.*?(?=##|\Z)'
        matches = re.findall(pattern, content, re.DOTALL)
        
        for match in matches:
            lines = match.strip().split('\n')
            if len(lines) > 4:
                # Extrair código E0xxx
                error_match = re.search(r'\bE\d{4}\b', match)
                if error_match:
                    code = error_match.group(0)
                    solution_lines = [l for l in lines if l.strip() and not l.startswith('#')]
                    solution = '\n'.join(solution_lines)
                    
                    if code not in self.errors_db:
                        self.errors_db[code] = {
                            'error_code': code,
                            'original_message': match[:200],
                            'solution': solution,
                            'context': 'imported_from_tracker',
                            'count': 1,
                            'first_seen': datetime.now().isoformat(),
                            'last_seen': datetime.now().isoformat(),
                            'resolved': True
                        }
        
        self.save_memory()
        return len(matches)


# Instância global
_blockchain_memory: Optional[BlockchainMemory] = None


def get_blockchain_memory() -> BlockchainMemory:
    """Retorna a instância global do BlockchainMemory."""
    global _blockchain_memory
    if _blockchain_memory is None:
        _blockchain_memory = BlockchainMemory()
    return _blockchain_memory
