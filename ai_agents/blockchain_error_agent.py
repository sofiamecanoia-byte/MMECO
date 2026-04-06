#!/usr/bin/env python3
"""
Blockchain Error Agent - Agente especializado em erros de desenvolvimento blockchain
Trabalha com erros Rust/Cargo do projeto MMECO

Uso:
    python blockchain_error_agent.py --error "error[E0277]: trait bound..."
    python blockchain_error_agent.py --build  # Analisa build atual
    python blockchain_error_agent.py --stats  # Mostra estatísticas
"""

import argparse
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Optional

# Adicionar diretório atual ao path
sys.path.insert(0, str(Path(__file__).parent))

from blockchain_memory import BlockchainMemory, get_blockchain_memory
from rust_error_patterns import (
    RUST_ERROR_PATTERNS,
    get_solution_for_error,
    format_solution_message,
    identify_error_category,
    parse_cargo_error,
    get_common_substrate_errors,
)


class BlockchainErrorAgent:
    """
    Agente especializado em resolver erros de desenvolvimento blockchain.
    Focado em Rust/Substrate/Polkadot SDK.
    """
    
    def __init__(self, project_path: str = "blockchain-core"):
        self.project_path = Path(project_path)
        self.memory = get_blockchain_memory()
        self.last_output = None  # Controlo de loops redundantes
        self.emoji = {
            "error": "🔴",
            "warning": "🟡",
            "success": "🟢",
            "info": "🔵",
            "fix": "🔧",
            "learn": "📚",
        }
    
    def analyze_error(self, error_msg: str, verbose: bool = False) -> Dict:
        """
        Analisa um erro e retorna informação sobre ele.
        
        Args:
            error_msg: Mensagem de erro completa
            verbose: Se True, mostra mais detalhes
            
        Returns:
            Dict com análise completa
        """
        result = {
            'error': error_msg,
            'code': None,
            'categories': [],
            'solution': None,
            'from_memory': False,
            'confidence': 0,
            'suggestions': [],
        }
        
        # 1. Extrair código de erro
        import re
        code_match = re.search(r'\bE\d{4}\b', error_msg)
        if code_match:
            result['code'] = code_match.group(0)
        
        # 2. Identificar categorias
        result['categories'] = identify_error_category(error_msg)
        
        # 3. Procurar na memória local
        memory_solution = self.memory.search_solution(error_msg)
        if memory_solution:
            result['solution'] = memory_solution['solution']
            result['from_memory'] = True
            result['confidence'] = memory_solution.get('confidence', 0.8)
        
        # 4. Se não encontrou, procurar na base de conhecimento
        if not result['solution']:
            pattern = get_solution_for_error(error_msg)
            if pattern:
                result['solution'] = format_solution_message(pattern)
                result['confidence'] = 0.6
                result['suggestions'] = pattern.patterns
        
        # 5. Adicionar sugestões baseadas em categorias
        if result['categories']:
            for cat in result['categories']:
                if cat == "Substrate Pallet":
                    result['suggestions'].append("Verificar Config trait e derives")
                elif cat == "Storage":
                    result['suggestions'].append("Verificar tipos de Storage")
                elif cat == "Tipos e Traits":
                    result['suggestions'].append("Verificar traits implementadas")
        
        return result
    
    def analyze_build(self, show_all: bool = False) -> Dict:
        """
        Executa cargo build e analisa todos os erros.
        
        Args:
            show_all: Se True, mostra todos os erros. Se False, apenas novos.
            
        Returns:
            Dict com resumo da análise
        """
        print(f"{self.emoji['info']} A executar cargo build...")
        
        try:
            # Executar build
            result = subprocess.run(
                ["cargo", "build"],
                cwd=self.project_path,
                capture_output=True,
                text=True,
                timeout=300  # 5 minutos timeout
            )
            
            output = result.stdout + result.stderr
            
        except subprocess.TimeoutExpired:
            return {
                'success': False,
                'errors': [],
                'summary': 'Build timeout (>5 min)'
            }
        except FileNotFoundError:
            return {
                'success': False,
                'errors': [],
                'summary': 'Cargo não encontrado. Instalar Rust?'
            }
        
        # Controlo de loops redundantes - evitar reprocessar mesmo output
        if self.last_output == output:
            return {
                'success': False,
                'errors': [],
                'summary': 'Output duplicado - skipping'
            }
        self.last_output = output
        
        # Guarda: só processar se houver erro no build
        if result.returncode == 0:
            return {
                'success': True,
                'errors': [],
                'summary': 'Build bem sucedido!'
            }
        
        # Parse erros
        errors = parse_cargo_error(output)
        
        if not errors:
            return {
                'success': True,
                'errors': [],
                'summary': 'Build bem sucedido!'
            }
        
        # Analisar cada erro
        analyzed_errors = []
        new_errors = 0
        known_errors = 0
        
        for error in errors:
            analysis = self.analyze_error(error.get('raw', ''), verbose=False)
            analysis['location'] = {
                'file': error.get('file'),
                'line': error.get('line')
            }
            analysis['message'] = error.get('message')
            analysis['hints'] = error.get('hints', [])
            
            analyzed_errors.append(analysis)
            
            if analysis['confidence'] > 0.5:
                known_errors += 1
            else:
                new_errors += 1
            
            # Adicionar à memória
            self.memory.learn_error(
                error.get('raw', ''),
                analysis['solution'] or '',
                f"{error.get('file')}:{error.get('line')}"
            )
        
        return {
            'success': result.returncode == 0,
            'errors': analyzed_errors,
            'summary': f"{len(errors)} erros: {known_errors} conhecidos, {new_errors} novos",
            'build_output': output[-2000:] if len(output) > 2000 else output
        }
    
    def report_error(self, error_msg: str, verbose: bool = False):
        """
        Reporta e formata informação sobre um erro.
        """
        print("\n" + "="*60)
        print(f"{self.emoji['error']} ANÁLISE DE ERRO")
        print("="*60)
        
        analysis = self.analyze_error(error_msg, verbose)
        
        # Código do erro
        if analysis['code']:
            print(f"\n{self.emoji['fix']} Código: {analysis['code']}")
        
        # Categorias
        if analysis['categories']:
            print(f"\n📂 Categorias: {', '.join(analysis['categories'])}")
        
        # Confiança
        conf = analysis['confidence']
        conf_emoji = "🟢" if conf > 0.7 else "🟡" if conf > 0.4 else "🔴"
        print(f"\n{conf_emoji} Confiança: {conf:.0%}")
        
        # Solução
        if analysis['solution']:
            print(f"\n{self.emoji['success']} SOLUÇÃO ENCONTRADA:")
            if analysis['from_memory']:
                print("   (Da memória local)")
            print(analysis['solution'])
        else:
            print(f"\n{self.emoji['warning']} Nenhuma solução conhecida.")
            print("   O erro foi registado para análise futura.")
            
            # Mostrar hints do compilador
            if analysis.get('hints'):
                print("\n💡 Hints do compilador:")
                for hint in analysis['hints']:
                    print(f"   • {hint}")
        
        # Sugestões
        if analysis['suggestions']:
            print("\n📋 Sugestões:")
            for s in analysis['suggestions']:
                print(f"   • {s}")
        
        print("\n" + "="*60)
    
    def report_build(self, show_all: bool = False):
        """
        Reporta resultado do build.
        """
        result = self.analyze_build(show_all)
        
        print("\n" + "="*60)
        print(f"{self.emoji['error']} RELATÓRIO DE BUILD")
        print("="*60)
        
        if result['success']:
            print(f"\n{self.emoji['success']} BUILD BEM SUCEDIDO!")
            return
        
        print(f"\n{self.emoji['error']} {result['summary']}")
        
        errors_by_category: Dict[str, List] = {}
        
        for error in result['errors']:
            cats = error.get('categories', ['Desconhecido'])
            cat = cats[0] if cats else 'Desconhecido'
            if cat not in errors_by_category:
                errors_by_category[cat] = []
            errors_by_category[cat].append(error)
        
        print("\n📊 ERROS POR CATEGORIA:")
        for cat, errors in errors_by_category.items():
            print(f"\n  {cat}: {len(errors)} erro(s)")
        
        print("\n" + "-"*60)
        print("DETALHES DOS ERROS:")
        print("-"*60)
        
        for i, error in enumerate(result['errors'][:10], 1):  # Limitar a 10
            loc = error.get('location', {})
            file_info = f"{loc.get('file', '?')}:{loc.get('line', '?')}" if loc else "?"
            
            print(f"\n{i}. {error.get('code', 'SEM_CÓDIGO')} @ {file_info}")
            if error.get('message'):
                msg = error.get('message')[:100]
                print(f"   {msg}{'...' if len(error.get('message', '')) > 100 else ''}")
            
            if error.get('confidence', 0) > 0.5:
                print(f"   ✅ {self.emoji['success']} Solução disponível ({error['confidence']:.0%})")
            else:
                print(f"   ❓ Sem solução conhecida")
        
        if len(result['errors']) > 10:
            print(f"\n... e mais {len(result['errors']) - 10} erros")
        
        print("\n" + "="*60)
    
    def report_stats(self):
        """Mostra estatísticas dos erros conhecidos."""
        stats = self.memory.get_error_stats()
        
        print("\n" + "="*60)
        print(f"{self.emoji['info']} ESTATÍSTICAS DE ERROS")
        print("="*60)
        
        print(f"\n📈 Total de erros registados: {stats['total_errors']}")
        print(f"✅ Resolvidos: {stats['resolved']}")
        print(f"⏳ Pendentes: {stats['pending']}")
        print(f"📊 Total de ocorrências: {stats['total_occurrences']}")
        
        if stats['top_errors']:
            print("\n🔥 TOP ERROS MAIS FREQUENTES:")
            for i, err in enumerate(stats['top_errors'], 1):
                status = "✅" if err['resolved'] else "❌"
                print(f"   {i}. {err['code']} ({err['count']}x) {status}")
        
        # Erros comuns do Substrate
        print("\n📚 ERROS COMUNS DO SUBSTRATE:")
        substrate_errors = get_common_substrate_errors()
        for pattern in substrate_errors[:5]:
            print(f"   • {pattern.code}: {pattern.category}")
    
    def interactive_mode(self):
        """Modo interativo para análise de erros."""
        print("\n" + "="*60)
        print(f"{self.emoji['learn']} MODO INTERATIVO")
        print("="*60)
        print("\nComandos:")
        print("  build  - Executar cargo build e analisar")
        print("  stats  - Ver estatísticas")
        print("  help   - Mostrar erros comuns do Substrate")
        print("  quit   - Sair")
        
        while True:
            try:
                cmd = input("\n> ").strip().lower()
                
                if cmd == 'quit':
                    break
                elif cmd == 'build':
                    self.report_build(show_all=True)
                elif cmd == 'stats':
                    self.report_stats()
                elif cmd == 'help':
                    print("\n📚 ERROS COMUNS DO SUBSTRATE:")
                    for pattern in get_common_substrate_errors():
                        print(f"\n{pattern.code}: {pattern.category}")
                        print(f"   {pattern.explanation}")
                        if pattern.example_fix:
                            print(f"   Fix: {pattern.example_fix[:100]}...")
                elif cmd:
                    # Tratar como mensagem de erro
                    self.report_error(cmd, verbose=True)
            except KeyboardInterrupt:
                print("\n\nAdeus!")
                break


def main():
    parser = argparse.ArgumentParser(
        description="Blockchain Error Agent - Analisador de erros Rust/Substrate"
    )
    
    parser.add_argument(
        '--error', '-e',
        help='Mensagem de erro para analisar'
    )
    
    parser.add_argument(
        '--build', '-b',
        action='store_true',
        help='Executar cargo build e analisar erros'
    )
    
    parser.add_argument(
        '--stats', '-s',
        action='store_true',
        help='Mostrar estatísticas'
    )
    
    parser.add_argument(
        '--project', '-p',
        default='blockchain-core',
        help='Caminho para o projeto (default: blockchain-core)'
    )
    
    parser.add_argument(
        '--verbose', '-v',
        action='store_true',
        help='Modo verboso'
    )
    
    parser.add_argument(
        '--interactive', '-i',
        action='store_true',
        help='Modo interativo'
    )
    
    parser.add_argument(
        '--import-tracker',
        action='store_true',
        help='Importar erros do error-tracker'
    )
    
    args = parser.parse_args()
    
    # Criar agente
    agent = BlockchainErrorAgent(project_path=args.project)
    
    # Importar do tracker se solicitado
    if args.import_tracker:
        count = agent.memory.import_from_tracker()
        print(f"✅ Importados {count} erros do error-tracker")
        return
    
    # Interativo
    if args.interactive:
        agent.interactive_mode()
        return
    
    # Build
    if args.build:
        agent.report_build()
        return
    
    # Stats
    if args.stats:
        agent.report_stats()
        return
    
    # Erro específico
    if args.error:
        agent.report_error(args.error, verbose=args.verbose)
        return
    
    # Sem argumentos = ajuda
    parser.print_help()
    print("\n💡 Exemplo: python blockchain_error_agent.py --error 'error[E0277]: the trait bound...'")


if __name__ == "__main__":
    main()
