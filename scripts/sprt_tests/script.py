import os
from datetime import datetime, timezone

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
    -each option.Threads=4 option.Hash=64 \
    -pgnout file="games.pgn" \
    -openings file=UHO_Lichess_4852_v1.epd format=epd order=random \
    -each tc=8+0.08 \
    -rounds 2000 -repeat \
    -concurrency 2 \
    -recover \
    -sprt elo0=85 elo1=90 alpha=0.01 beta=0.01
"""

os.system(f"{execute_sprt_test_current_vs_develop}")
# os.system("rm -rf zeno_develop/")
os.system("./ordo -o ratings.txt -- games.pgn ")
os.system("cat ratings.txt")
os.system("./ordo -o ratings.txt -- games.pgn ")
os.system("rm ratings.txt")
os.system("rm games.pgn")

# execute_sprt_current_vs_release_2_0 = f"""
# ./fastchess \
#     -engine cmd={zeno_current} name="Zeno Current" \
#     -engine cmd={zeno_2_0} name="Zeno 2.0" \
#     -pgnout file="games_vs_releases.pgn" \
#     -openings file=UHO_Lichess_4852_v1.epd format=epd order=random \
#     -each tc=8+0.08 \
#     -rounds 10000 -repeat \
#     -concurrency 14 \
#     -recover \
#     -sprt elo0=20 elo1=25 alpha=0.01 beta=0.01
# """
#
#
# os.system(f"{execute_sprt_current_vs_release_2_0}")
# os.system("./ordo -o ratings_vs_releases.txt -- games_vs_releases.pgn ")
# os.system("cat ratings_vs_releases.txt")
# os.system("rm games_vs_releases.pgn")
# os.system("rm ratings_vs_releases.txt")


def play_games_for_tuning(number_of_games):
    number_of_games = max(number_of_games, 2)

    now = datetime.now(timezone.utc)
    timestamp = now.strftime("%Y-%m-%d-%H-%M-%S")
    output_file = f"games-for-tuner-{number_of_games}-{timestamp}.pgn"

    execute_sprt_for_tuner = f"""
    ./fastchess \
        -engine cmd={zeno_current} name="Zeno current 1" \
        -engine cmd={zeno_current} name="Zeno current 2" \
        -pgnout file={output_file} \
        -openings file=UHO_Lichess_4852_v1.epd format=epd order=random \
        -each tc=1+0.08 \
        -rounds {number_of_games // 2} -repeat \
        -concurrency 16 \
        -recover \
    """
    os.system(f"{execute_sprt_for_tuner}")


# play_games_for_tuning(100000)
