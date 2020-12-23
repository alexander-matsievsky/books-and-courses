FROM jupyter/all-spark-notebook:a0a544e6dc6e
ENV JUPYTER_ENABLE_LAB=yes
RUN conda install -c r r-lahman r-nycflights13 r-tidyverse
RUN R -e 'install.packages("gapminder", lib = "/opt/conda/lib/R/library", repos = "http://cran.us.r-project.org")'
