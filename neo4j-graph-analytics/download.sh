mkdir -p data vendor

curl -sL 'https://resources.oreilly.com/examples/0636920233145/-/archive/master/0636920233145-master.tar.bz2' |
  tar --strip-components=1 -xjC data
curl -sL 'https://github.com/neo4j-contrib/neo4j-apoc-procedures/releases/download/4.0.0.4/apoc-4.0.0.4-all.jar' \
  >vendor/apoc-4.0.0.4-all.jar

#docker run --rm -p 8888:8888 -p 4040:4040 -p 4041:4041 -v "$PWD":/home/jovyan/work jupyter/pyspark-notebook:50d1eb9ec2d8
