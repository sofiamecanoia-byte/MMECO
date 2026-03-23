"""Error Detector - Monitoriza builds do GitHub Actions"""
import subprocess
import json
import re
import time
import logging
from datetime import datetime
from pathlib import Path

class ErrorDetector:
    def __init__(self, memory_manager, config):
        self.memory = memory_manager
        self.config = config
        self.logger = logging.getLogger(__name__)
        self.last_run_id = None
        
    def check_github_builds(self):
        """Verifica builds do GitHub Actions via gh CLI"""
        try:
            # Buscar último build
            result = subprocess.run(
                ['gh', 'run', 'list', '--limit', '1', '--workflow', 'rust.yml', '--json', 'databaseId,conclusion,status'],
                capture_output=True, text=True, timeout=10, cwd='/Users/nunocunha/Documents/GitHub/MMECO'
            )
            
            if result.returncode != 0:
                self.logger.warning(f"⚠️  gh CLI error: {result.stderr}")
                return
                
            runs = json.loads(result.stdout)
            if not runs:
                self.logger.info("📭 Nenhum build encontrado")
                return
                
            run = runs[0]
            run_id = run['databaseId']
            
            # Se já processamos este build, skip
            if run_id == self.last_run_id:
                return
                
            self.logger.info(f"�� Verificando build #{run_id} (status: {run['status']}, conclusion: {run.get('conclusion')})")
            
            # Se ainda está a correr, aguardar
            if run['status'] == 'in_progress':
                self.logger.info("⏳ Build ainda a correr...")
                return
                
            # Se passou com sucesso
            if run.get('conclusion') == 'success':
                self.logger.info("✅ Build passou! Nenhum erro.")
                self.last_run_id = run_id
                return
                
            # Se falhou, buscar erros
            if run.get('conclusion') == 'failure':
                self.logger.warning(f"❌ Build #{run_id} FALHOU! A buscar erros...")
                self.extract_errors_from_run(run_id)
                self.last_run_id = run_id
                
        except Exception as e:
            self.logger.error(f"💥 Erro ao verificar builds: {e}")
            
    def extract_errors_from_run(self, run_id):
        """Extrai erros do log de um build"""
        try:
            # Buscar logs
            result = subprocess.run(
                ['gh', 'run', 'view', str(run_id), '--log'],
                capture_output=True, text=True, timeout=60, cwd='/Users/nunocunha/Documents/GitHub/MMECO'
            )
            
            if result.returncode != 0:
                self.logger.error(f"Erro ao buscar logs: {result.stderr}")
                return
                
            logs = result.stdout
            
            # Extrair erros Rust (error[E0xxx])
            error_pattern = r'error\[E(\d+)\]:\s*(.+?)(?=\n\s*-->|\nerror|\nwarning|\n\n|$)'
            errors = re.findall(error_pattern, logs, re.DOTALL)
            
            if not errors:
                self.logger.info("🤔 Build falhou mas não encontrei erros E0xxx nos logs")
                return
                
            self.logger.info(f"🎯 Encontrados {len(errors)} erros!")
            
            # Processar cada erro
            for error_code, error_msg in errors[:10]:  # Limitar a 10 primeiros
                error_msg_clean = error_msg.strip()[:200]  # Primeiros 200 chars
                error_key = f"E{error_code}"
                
                self.logger.info(f"  📝 {error_key}: {error_msg_clean[:80]}...")
                
                # Guardar na memória (se não existir solução)
                existing = self.memory.search_solution(f"error[E{error_code}]")
                if not existing:
                    self.memory.learn_error(
                        error_msg=f"error[E{error_code}]: {error_msg_clean}",
                        solution="Aguardando análise",
                        context=f"GitHub Actions build #{run_id}"
                    )
                    
            # Atualizar documentação
            self.logger.info("📄 Atualizando documentação...")
            
            # FORÇAR atualização da documentação
            try:
                from sdk_agents.documentation_manager import DocumentationManager
                from datetime import datetime
                
                doc_mgr = DocumentationManager(self.config)
                
                # Preparar lista de erros para documentação
                errors_for_doc = []
                for key, entry in self.memory.memory_db.items():
                    errors_for_doc.append({
                        'code': key,
                        'message': entry.error_key,
                        'location': 'blockchain-core/runtime/src/lib.rs',
                        'timestamp': datetime.now().strftime('%Y-%m-%d %H:%M:%S'),
                        'context': entry.context,
                        'solution': entry.solution,
                        'confidence': entry.confidence,
                        'count': entry.count,
                        'categories': ['Rust', 'Compilation']
                    })
                
                if errors_for_doc:
                    doc_mgr.update_error_tracker(errors_for_doc)
                    self.logger.info(f"✅ Documentação atualizada com {len(errors_for_doc)} erros!")
                    
            except Exception as doc_err:
                self.logger.error(f"❌ Erro ao atualizar docs: {doc_err}")
            
        except Exception as e:
            self.logger.error(f"💥 Erro ao extrair erros: {e}")
            
    def start(self):
        """Inicia monitorização contínua"""
        self.logger.info("🔍 Olhos bem abertos: Monitorizando o código...")
        
        while True:
            try:
                self.logger.info("🔨 Executando análise do motor Rust...")
                self.check_github_builds()
                
            except Exception as e:
                self.logger.error(f"💥 Erro no loop: {e}")
                
            # Aguardar intervalo configurado
            time.sleep(self.config.ERROR_CHECK_INTERVAL)
