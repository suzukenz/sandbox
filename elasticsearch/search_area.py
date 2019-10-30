import argparse

from elasticsearch import Elasticsearch

es = Elasticsearch()

parser = argparse.ArgumentParser(description='indexer')
parser.add_argument('area', help='町域')

args = parser.parse_args()

res = es.search(index='geo', body={'query': {'match': {'area': args.area}}})
print("Got %d Hits:" % res['hits']['total']['value'])
for hit in res['hits']['hits']:
    print(hit["_source"])
