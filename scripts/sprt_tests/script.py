import os

zeno_current = "../../target/release/zeno"
develop = "https://github.com/toudonou/zeno"
zeno_1_0 = "./zeno-1.0"

# Compile the current version
os.system("cd ../ && cargo build --release")

# Download the version currently on github
os.system("mkdir zeno_develop/")
os.system(f"git clone {develop} zeno_develop")
os.system(
    "cd zeno_develop/ && cargo build --release && mv ./target/release/zeno ./target/release/zeno_develop"
)

zeno_develop = "./zeno_develop/target/release/zeno_develop"

execute_sprt_test_current_vs_develop = f"""
./fastchess \
    -engine cmd={zeno_current} name="Zeno current" \
    -engine cmd={zeno_develop} name="Zeno develop" \
    -pgnout file="games.pgn" \
    -openings file=8moves_v3.pgn format=pgn order=random \
    -each tc=8+0.08 \
    -rounds 2000 -repeat \
    -concurrency 14 \
    -recover \
    -sprt elo0=120 elo1=125 alpha=0.05 beta=0.1
"""
os.system(f"{execute_sprt_test_current_vs_develop}")
os.system("rm -rf zeno_develop/")
os.system("./ordo -o ratings.txt -- games.pgn ")
os.system("rm games.pgn")

execute_sprt_test_current_vs_release_1_0 = f"""
./fastchess \
    -engine cmd={zeno_current} name="Zeno current" \
    -engine cmd={zeno_1_0} name="Zeno 1.0" \
    -pgnout file="games_vs_releases.pgn" \
    -openings file=8moves_v3.pgn format=pgn order=random \
    -each tc=8+0.08 \
    -rounds 2000 -repeat \
    -concurrency 14 \
    -recover \
    -sprt elo0=135 elo1=138 alpha=0.05 beta=0.1
"""
os.system(f"{execute_sprt_test_current_vs_release_1_0}")
os.system("./ordo -o ratings_vs_releases.txt -- games_vs_releases.pgn ")
os.system("rm games_vs_releases.pgn")

os.system("cat ratings.txt")
os.system("cat ratings_vs_releases.txt")
