# gas clasp starter

権限が必要な API 系を利用する時は appsscript.json に追記が必要

例:

```json
  "oauthScopes": [
    "https://www.googleapis.com/auth/devstorage.read_write",
    "https://www.googleapis.com/auth/drive",
    "https://www.googleapis.com/auth/spreadsheets",
    "https://www.googleapis.com/auth/script.external_request
  ]
```
