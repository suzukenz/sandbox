# ElasticSearch を試す

ElasticSearch + Python + elasticsearch_dsl を使って index の張り替えを試したコード

## Prerequisites

データは[ここ](https://www.post.japanpost.jp/zipcode/dl/oogaki-zip.html)からダウンロードして`testdata`ディレクトリに置きました

- 13TOKYO.CSV
- 27OSAKA.CSV

## Running

es と kibana 起動

```
docker-compose up

# http://localhost:5601/ でアクセスできる
```

index 作成

```
pipenv run index (tokyo|osaka)
```

search（町域で検索するだけの手抜き）

```
pipenv run search 大井
```
