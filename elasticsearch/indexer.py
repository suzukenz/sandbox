import argparse
import csv

from elasticsearch.helpers import bulk
from elasticsearch_dsl import Index, Document, Text, Q
from elasticsearch_dsl.connections import connections

parser = argparse.ArgumentParser(description='indexer')
parser.add_argument('name', help='tokyo|osaka')

# Define a default Elasticsearch client
connections.create_connection(hosts=['localhost'])

FILE_PATHS = {
    'tokyo': './testdata/13TOKYO.CSV',
    'osaka': './testdata/27OSAKA.CSV'
}


class ZipCode(Document):
    zip_code = Text()
    prefecture = Text(analyzer='kuromoji')
    city = Text(analyzer='kuromoji')
    area = Text(analyzer='kuromoji')


def process_csv_row(index_name, row):
    return ZipCode(meta={'index': index_name, 'id': row[2]}, zip_code=row[2], prefecture=row[6], city=row[7], area=row[8])


if __name__ == "__main__":
    args = parser.parse_args()
    current_idx_name = args.name

    # create the mappings in elasticsearch
    ZipCode.init(index=current_idx_name)

    with open(FILE_PATHS[args.name]) as f:
        reader = csv.reader(f)
        header = next(reader)
        docs = [process_csv_row(current_idx_name, row).to_dict(True) for row in reader]
        client = connections.get_connection()
        bulk(client, docs)

    idx = Index(current_idx_name)

    idx.put_alias(name='geo')

    for k in FILE_PATHS.keys():
        if k != current_idx_name:
            idx = Index(k)
            if idx.exists():
                # idx.delete_alias(name='geo')
                idx.delete()
