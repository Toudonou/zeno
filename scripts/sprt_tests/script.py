import os

zeno_2_0 = "./zeno-2.0"
zeno_current = "../../target/release/zeno"
develop = "https://github.com/toudonou/zeno"

# Compile the current version
os.system("cargo build --release")

# Download the version currently on GitHub
os.system("mkdir zeno_develop/")
os.system(f"git clone {develop} zeno_develop")
os.system(
  "cd zeno_develop/ && cargo build --release && mv ./target/release/zeno ./target/release/zeno_develop"
)

zeno_develop = "./zeno_develop/target/release/zeno_develop"

execute_sprt_test_current_vs_develop = f"""
./fastchess \
    -engine cmd={zeno_current} name="Zeno current" \
    -engine cmd={zeno_develop} name="Zeno Develop" \
    -pgnout file="games.pgn" \
    -openings file=UHO_Lichess_4852_v1.epd format=epd order=random \
    -each tc=8+0.08 \
    -rounds 10000 -repeat \
    -sprt elo0=-5 elo1=0.0 alpha=0.01 beta=0.01 \
    -concurrency 16 \
    -recover \
"""
#

os.system(f"{execute_sprt_test_current_vs_develop}")
os.system("rm -rf zeno_develop/")
os.system("./ordo -o ratings.txt -- games.pgn ")
os.system("cat ratings.txt")
os.system("./ordo -o ratings.txt -- games.pgn ")
os.system("rm ratings.txt")
os.system("rm games.pgn")
