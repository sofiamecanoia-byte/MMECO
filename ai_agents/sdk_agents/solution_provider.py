import os, re
def fix_e0599():
    path = 'blockchain-core/runtime/src/lib.rs'
    if os.path.exists(path):
        with open(path, 'r') as f: content = f.read()
        new_exec = 'pub type Executive = frame_executive::Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem, TxExtension>;'
        content = re.sub(r'pub type Executive = frame_executive::Executive<.*?>;', new_exec, content)
        with open(path, 'w') as f: f.write(content)
        return True
    return False

if __name__ == "__main__":
    if fix_e0599():
        print("🤖 Agente: Apliquei a correção do Executive!")
        os.system("git add blockchain-core/runtime/src/lib.rs && git commit -m '🤖 fix: auto-update executive' && git push origin bld-1.7")
