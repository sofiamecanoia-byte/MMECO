import re
import sys
import json

def analyze_log(file_path):
    print(f"🔍 Analisando log: {file_path}")
    try:
        with open(file_path, 'r') as f:
            log = f.read()
        
        # Procura erros específicos do SDK 2412
        errors = {
            "E0560 (Genesis)": len(re.findall(r"E0560", log)),
            "E0277 (Traits)": len(re.findall(r"E0277", log)),
            "E0308 (Types)": len(re.findall(r"E0308", log)),
            "E0412 (Missing Type)": len(re.findall(r"E0412", log))
        }
        
        print("\n=== Relatório de Erros SDK 2412 ===")
        for err, count in errors.items():
            status = "❌" if count > 0 else "✅"
            print(f"{status} {err}: {count} ocorrências")
            
    except Exception as e:
        print(f"Erro ao ler log: {e}")

if __name__ == "__main__":
    log_file = sys.argv[1] if len(sys.argv) > 1 else "../blockchain-core/cargo_output.log"
    analyze_log(log_file)
