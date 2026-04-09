import os
from openai import OpenAI

# Garante que a chave está no ambiente
api_key = os.getenv("OPENAI_API_KEY")
if not api_key:
    print("ERRO: OPENAI_API_KEY não encontrada. Corre: export OPENAI_API_KEY='tua_chave'")
    exit()

client = OpenAI(api_key=api_key)

def pedir_ajuda(task, model="gpt-4o-mini"):
    # Preços por 1M tokens (ajustado para gpt-4o-mini)
    in_price = 0.15
    out_price = 0.60
    
    # Se usares o1-mini, os preços mudam
    if model == "o1-mini":
        in_price = 1.10
        out_price = 4.40

    response = client.chat.completions.create(
        model=model,
        messages=[{"role": "system", "content": "És um especialista em Substrate SDK 2412. Responde de forma técnica, curta e sem conversa fiada."},
                  {"role": "user", "content": task}]
    )

    prompt_t = response.usage.prompt_tokens
    comp_t = response.usage.completion_tokens
    custo = (prompt_t * in_price / 1000000) + (comp_t * out_price / 1000000)

    print(f"\n" + "="*30)
    print(f"MODELO: {model}")
    print(f"TOKENS: {response.usage.total_tokens} (In: {prompt_t} | Out: {comp_t})")
    print(f"CUSTO ESTIMADO: ${custo:.6f}")
    print("="*30)
    print(f"\nRESPOSTA:\n{response.choices[0].message.content}\n")

if __name__ == "__main__":
    import sys
    query = " ".join(sys.argv[1:]) if len(sys.argv) > 1 else "Como declarar um evento no Substrate 2412?"
    pedir_ajuda(query)
