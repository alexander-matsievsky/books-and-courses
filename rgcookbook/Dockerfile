FROM jupyter/all-spark-notebook:703d8b2dcb88
ENV JUPYTER_ENABLE_LAB=yes
RUN conda install -c r r-devtools r-tidyverse
RUN R -e 'devtools::install_github("wch/gcookbook", lib = "/opt/conda/lib/R/library")'
