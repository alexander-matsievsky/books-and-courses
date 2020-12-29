FROM jupyter/all-spark-notebook:a0a544e6dc6e
ENV JUPYTER_ENABLE_LAB=yes
RUN conda install -c r                    \
    r-feather                             \
    r-hexbin                              \
    r-lahman                              \
    r-maps                                \
    r-nycflights13                        \
    r-tidyverse
RUN R -e '                                \
  install.packages(                       \
    c(                                    \
      "babynames",                        \
      "fueleconomy",                      \
      "gapminder",                        \
      "ggbeeswarm",                       \
      "ggstance",                         \
      "lvplot",                           \
      "nasaweather"                       \
    ),                                    \
    lib = "/opt/conda/lib/R/library",     \
    repos = "https://cloud.r-project.org" \
  )                                       \
'
