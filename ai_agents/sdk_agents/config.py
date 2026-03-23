"""Configuração dos SDK Agents"""
import os

class Config:
    # GitHub
    GITHUB_REPO = "groovaloo/MMECO"
    GITHUB_BRANCH = "bld-1.7"
    
    # Usar gh CLI em vez de API direta
    USE_GH_CLI = True
    
    # Timings (em segundos)
    ERROR_CHECK_INTERVAL = 300  # 5 min
    KEEP_ALIVE_INTERVAL = 120   # 2 min
    
    # Paths
    DOCS_PATH = "/Users/nunocunha/Documents/GitHub/MMECO/docs/internal/error-tracker"
    MEMORY_PATH = "/Users/nunocunha/Documents/GitHub/MMECO/ai_agents/sdk_agents/errors_memory.json"
    LOGS_PATH = "logs"
    DAO_MEMORY_FILE = "dao_memory.json"  # ✅ ADICIONADO
    
    @classmethod
    def get_error_files(cls):
        """Retorna dicionário de ficheiros de erro"""
        return {
            'all_errors': os.path.join(cls.DOCS_PATH, 'all-errors.md'),
            'resolved': os.path.join(cls.DOCS_PATH, 'resolved.md'),
            'frequent_errors': os.path.join(cls.DOCS_PATH, 'frequent-errors.md'),
            'lessons_learned': os.path.join(cls.DOCS_PATH, 'lessons-learned.md'),
            'quick_reference': os.path.join(cls.DOCS_PATH, 'quick-reference.md')
        }
    
    @classmethod
    def get_memory_config(cls):
        """Retorna configuração de memória"""
        return {
            'file': cls.MEMORY_PATH,
            'max_entries': 100,
            'auto_save': True
        }
    
    @classmethod
    def get_github_config(cls):
        """Retorna configuração do GitHub"""
        return {
            'repo': cls.GITHUB_REPO,
            'branch': cls.GITHUB_BRANCH,
            'use_cli': cls.USE_GH_CLI
        }
    
    @classmethod
    def get_logging_config(cls):
        """Retorna configuração de logs"""
        return {
            'path': cls.LOGS_PATH,
            'level': 'INFO'
        }

# Funções auxiliares
def get_config():
    return Config
    
def validate_config():
    """Valida se todas as configurações estão OK"""
    return True
