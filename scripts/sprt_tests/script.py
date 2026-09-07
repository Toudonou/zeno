import os
import shutil
from random import randint

REPO = "https://github.com/toudonou/zeno"


def build_current():
    """Build the current local checkout."""
    os.system("cargo build --release")
    return "../../target/release/zeno"


def build_zeno_ref(ref):
    """
    Build a zeno version from a git ref (tag, branch, or commit hash)
    """
    name = ref[:24] + "_" + str(randint(0, 10000))
    name = name.replace("/", "_")

    dirname = f"zeno_{name}"

    if os.path.exists(dirname):
        shutil.rmtree(dirname)

    os.system(f"git clone {REPO} {dirname}")
    os.system(f"cd {dirname} && git checkout {ref} && cargo build --release")
    os.system(f"mv {dirname}/target/release/zeno {dirname}/target/release/zeno_{name}")

    return f"./{dirname}/target/release/zeno_{name}", dirname, name


def run_sprt_test(
    ref1,
    ref2,
    name1=None,
    name2=None,
    rounds=10000,
    threads=15,
    bounds=(),
    alpha=0.01,
    beta=0.01,
):
    """
    Compare two zeno versions

    ref can be:
      - commit hash
      - tag
      - branch

    If ref2 is None, compare against current local build
    """

    cleanup = []

    if ref1 is None:
        engine1 = build_current()
        engine2, dir2, temp_name = build_zeno_ref(ref2)
        cleanup.append(dir2)

        name1 = name1 or "Zeno current"
        name2 = temp_name or ref2[:24]

    else:
        engine1, dir1, temp_name1 = build_zeno_ref(ref1)
        engine2, dir2, temp_name2 = build_zeno_ref(ref2)
        cleanup.extend([dir1, dir2])

        name1 = name1 or temp_name1
        name2 = name2 or temp_name2

    sprt_bounds = ""
    if bounds:
        sprt_bounds = (
            f" -sprt elo0={bounds[0]} elo1={bounds[1]} alpha={alpha} beta={beta}"
        )

    cmd = f"""
    ./fastchess \
    -engine cmd={engine1} name="{name1}" \
    -engine cmd={engine2} name="{name2}" \
    -pgnout file="games.pgn" \
    -openings file=UHO_Lichess_4852_v1.epd format=epd order=random \
    -each tc=8+0.08 option.Threads=1 option.Hash=1 \
    -rounds {rounds} -repeat \
    {sprt_bounds} \
    -concurrency {threads} \
    -recover
  """

    os.system("cp ~/UHO_Lichess_4852_v1.epd UHO_Lichess_4852_v1.epd")

    os.system(cmd)

    os.system("./ordo -o ratings.txt -- games.pgn")
    os.system("cat ratings.txt")

    for d in cleanup:
        shutil.rmtree(d)

    for f in ["ratings.txt", "games.pgn", "UHO_Lichess_4852_v1.epd", "config.json"]:
        if os.path.exists(f):
            os.remove(f)


run_sprt_test(
  None,
    "develop",
    rounds=200,
)
