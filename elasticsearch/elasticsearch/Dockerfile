FROM docker.elastic.co/elasticsearch/elasticsearch:7.2.0

RUN ./bin/elasticsearch-plugin install org.codelibs:elasticsearch-analysis-kuromoji-ipadic-neologd:7.2.0 && \
  ./bin/elasticsearch-plugin install analysis-icu && \
  ./bin/elasticsearch-plugin install analysis-kuromoji
