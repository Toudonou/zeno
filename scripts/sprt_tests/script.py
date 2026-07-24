import os
import shutil

REPO = "https://github.com/toudonou/zeno"

def build_current():
    """Build the current local checkout."""
    os.system("cargo build --release")
    return "../../target/release/zeno"

def build_zeno_ref(ref):
  """
  Build a zeno version from a git ref (tag, branch, or commit hash)
  """
  dirname = f"zeno_{ref[:8]}"

  if os.path.exists(dirname):
    shutil.rmtree(dirname)

  os.system(f"git clone {REPO} {dirname}")
  os.system(f"cd {dirname} && git checkout {ref} && cargo build --release")
  os.system(
    f"mv {dirname}/target/release/zeno {dirname}/target/release/zeno_{ref[:8]}"
  )

  return f"./{dirname}/target/release/zeno_{ref[:8]}", dirname


def run_sprt_test(ref1, ref2, name1=None, name2=None, rounds=10000, threads=16, bounds=(), alpha=0.01, beta=0.01):
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
    engine2, dir2 = build_zeno_ref(ref2)
    cleanup.append(dir2)

    name1 = name1 or "Zeno current"
    name2 = name2 or ref2[:8]

  else:
    engine1, dir1 = build_zeno_ref(ref1)
    engine2, dir2 = build_zeno_ref(ref2)
    cleanup.extend([dir1, dir2])

    name1 = name1 or ref1[:8]
    name2 = name2 or ref2[:8]


  sprt_bounds = ""
  if bounds:
    sprt_bounds = f" -sprt elo0={bounds[0]} elo1={bounds[1]} alpha={alpha} beta={beta}"

  cmd = f"""
    ./fastchess \
    -engine cmd={engine1} name="{name1}" \
    -engine cmd={engine2} name="{name2}" \
    -pgnout file="games.pgn" \
    -openings file=UHO_Lichess_4852_v1.epd format=epd order=random \
    -each tc=8+0.08 \
    -rounds {rounds} -repeat \
    {sprt_bounds} \
    -concurrency {threads} \
    -recover
  """

  os.system(cmd)

  os.system("./ordo -o ratings.txt -- games.pgn")
  os.system("cat ratings.txt")

  for d in cleanup:
    shutil.rmtree(d)

  for f in ["ratings.txt", "games.pgn"]:
    if os.path.exists(f):
      os.remove(f)


run_sprt_test(
    "little-refactoring",
    "develop",
    name1="little-refactoring",
    name2="develop",
    rounds=30000,
    bounds=(-11, -10),
    alpha=0.01,
    beta=0.01
)
