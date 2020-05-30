# https://stackoverflow.com/a/246128
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

# https://jupyter-docker-stacks.readthedocs.io/en/latest/#quick-start
docker run --rm --network host -p 8888:8888 -e JUPYTER_ENABLE_LAB=yes -v "$DIR":/home/jovyan/work jupyter/r-notebook:76402a27fd13
