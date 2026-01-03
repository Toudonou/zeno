import os

zeno_current = "../target/release/zeno"
develop = "https://github.com/toudonou/zeno"

# Compile the current version
os.system("cd ../ && cargo build --release")

# Download the version currently on github
os.system("mkdir zeno_develop/")
os.system(f"git clone {develop} zeno_develop")
os.system(
    "cd zeno_develop/ && cargo build --release && mv ./target/release/zeno ./target/release/zeno_develop"
)

zeno_develop = "./zeno_develop/target/release/zeno_develop"

execute_sprt_test = f"""
./fastchess \
    -engine cmd={zeno_current} name="Zeno current" \
    -engine cmd={zeno_develop} name="Zeno develop" \
    -pgnout file="games.pgn" \
    -openings file=8moves_v3.pgn format=pgn order=random \
    -each tc=8+0.08 \
    -rounds 1000 -repeat \
    -concurrency 14 \
    -recover \
    -sprt elo0=30 elo1=50 alpha=0.05 beta=0.1
"""
os.system(f"{execute_sprt_test}")

os.system("rm -rf zeno_develop/")

os.system("./ordo -o ratings.txt -- games.pgn ")
os.system("cat ratings.txt")
os.system("rm games.pgn")
